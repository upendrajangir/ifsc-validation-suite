use crate::database::{fetch_all_bank_data, DatabasePool};
use crate::models::BankData;
use dashmap::DashMap;
use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MemoryStore {
    cache: Arc<DashMap<String, BankData>>,
    mmap_data: Arc<RwLock<Option<MmapMut>>>,
    file_path: String,
}

impl MemoryStore {
    pub async fn new() -> anyhow::Result<Self> {
        let file_path = "ifsc_data.bin".to_string();

        let store = Self {
            cache: Arc::new(DashMap::new()),
            mmap_data: Arc::new(RwLock::new(None)),
            file_path,
        };

        store.init_mmap_file().await?;

        Ok(store)
    }

    async fn init_mmap_file(&self) -> anyhow::Result<()> {
        let path = Path::new(&self.file_path);

        if !path.exists() {
            let mut file = std::fs::File::create(path)?;
            file.write_all(&[])?;
            file.sync_all()?;
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let file_len = file.metadata()?.len();
        if file_len < 1024 {
            file.set_len(1024 * 1024)?;
        }

        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

        let mut mmap_guard = self.mmap_data.write().await;
        *mmap_guard = Some(mmap);

        Ok(())
    }

    pub async fn get(&self, ifsc_code: &str) -> Option<BankData> {
        self.cache.get(ifsc_code).map(|entry| entry.clone())
    }

    pub async fn insert(&self, ifsc_code: String, bank_data: BankData) {
        self.cache.insert(ifsc_code, bank_data);

        let cache_clone = Arc::clone(&self.cache);
        let mmap_data_clone = Arc::clone(&self.mmap_data);
        tokio::spawn(async move {
            if let Err(e) = Self::persist_to_mmap(cache_clone, mmap_data_clone).await {
                tracing::error!("Failed to persist data to mmap: {}", e);
            }
        });
    }

    pub async fn load_from_database(&self, pool: &DatabasePool) -> anyhow::Result<()> {
        let bank_data_list = fetch_all_bank_data(pool).await?;

        tracing::info!("Loading {} records from database into memory", bank_data_list.len());

        for bank_data in bank_data_list {
            self.cache.insert(bank_data.ifsc.clone(), bank_data);
        }

        Self::persist_to_mmap(Arc::clone(&self.cache), Arc::clone(&self.mmap_data)).await?;

        tracing::info!("Successfully loaded data into memory and persisted to mmap");

        Ok(())
    }

    async fn persist_to_mmap(
        cache: Arc<DashMap<String, BankData>>,
        mmap_data: Arc<RwLock<Option<MmapMut>>>,
    ) -> anyhow::Result<()> {
        // Collect all data
        let data: Vec<(String, BankData)> = cache
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();

        let serialized = serde_json::to_vec(&data)?;

        let needs_resize = {
            let mmap_guard = mmap_data.read().await;
            if let Some(ref mmap) = *mmap_guard {
                serialized.len() > mmap.len()
            } else {
                true
            }
        };

        if needs_resize {
            // Resize file and recreate mmap
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open("ifsc_data.bin")?;

            let new_size = (serialized.len() * 2).max(1024 * 1024); // At least 1MB
            file.set_len(new_size as u64)?;

            let new_mmap = unsafe { MmapOptions::new().map_mut(&file)? };
            let mut mmap_guard = mmap_data.write().await;
            *mmap_guard = Some(new_mmap);
        }

        let mut mmap_guard = mmap_data.write().await;
        if let Some(ref mut mmap) = *mmap_guard {
            if serialized.len() <= mmap.len() {
                // Write length first (8 bytes)
                let len_bytes = (serialized.len() as u64).to_le_bytes();
                mmap[0..8].copy_from_slice(&len_bytes);

                // Write data
                mmap[8..8 + serialized.len()].copy_from_slice(&serialized);

                // Flush to disk
                mmap.flush()?;
            }
        }

        Ok(())
    }
    

    pub fn get_stats(&self) -> (usize, usize) {
        let cache_size = self.cache.len();
        let memory_usage = cache_size * std::mem::size_of::<BankData>();
        (cache_size, memory_usage)
    }
}