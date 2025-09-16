use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::Arc;
use tracing::info;

mod database;
mod ifsc_service;
mod memory_store;
mod models;

use database::DatabasePool;
use ifsc_service::IfscService;
use memory_store::MemoryStore;
use models::{ApiResponse, BankData, IfscValidationResponse};

#[derive(Serialize)]
struct StatsResponse {
    cache_size: usize,
    memory_usage_bytes: usize,
}

#[derive(Clone)]
pub struct AppState {
    db_pool: DatabasePool,
    memory_store: Arc<MemoryStore>,
    ifsc_service: Arc<IfscService>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let db_pool = database::init_pool().await?;
    database::run_migrations(&db_pool).await?;

    let memory_store = Arc::new(MemoryStore::new().await?);

    memory_store.load_from_database(&db_pool).await?;
    
    let ifsc_service = Arc::new(IfscService::new());

    let state = AppState {
        db_pool,
        memory_store,
        ifsc_service,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/stats", get(get_stats))
        .route("/validate/{ifsc_code}", get(validate_ifsc))
        .route("/bank/{ifsc_code}", get(get_bank_data))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    info!("Starting IFSC Validation Suite on 0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("IFSC Validation Suite is running".to_string()),
        message: "Health check passed".to_string(),
    })
}

async fn get_stats(State(state): State<AppState>) -> Json<ApiResponse<StatsResponse>> {
    let (cache_size, memory_usage) = state.memory_store.get_stats();
    Json(ApiResponse {
        success: true,
        data: Some(StatsResponse {
            cache_size,
            memory_usage_bytes: memory_usage,
        }),
        message: "Statistics retrieved successfully".to_string(),
    })
}

async fn validate_ifsc(
    State(state): State<AppState>,
    Path(ifsc_code): Path<String>,
) -> Result<Json<ApiResponse<IfscValidationResponse>>, StatusCode> {
    let ifsc_code = ifsc_code.to_uppercase();

    if let Some(bank_data) = state.memory_store.get(&ifsc_code).await {
        return Ok(Json(ApiResponse {
            success: true,
            data: Some(IfscValidationResponse {
                ifsc_code: ifsc_code.clone(),
                valid: true,
                bank_data: Some(bank_data),
            }),
            message: "IFSC code is valid (from cache)".to_string(),
        }));
    }

    match state.ifsc_service.fetch_bank_data(&ifsc_code).await {
        Ok(bank_data) => {
            // Store in database
            if let Err(e) = database::store_bank_data(&state.db_pool, &bank_data).await {
                tracing::error!("Failed to store bank data in database: {}", e);
            }

            // Store in memory
            state.memory_store.insert(ifsc_code.clone(), bank_data.clone()).await;

            Ok(Json(ApiResponse {
                success: true,
                data: Some(IfscValidationResponse {
                    ifsc_code,
                    valid: true,
                    bank_data: Some(bank_data),
                }),
                message: "IFSC code is valid (fetched from API)".to_string(),
            }))
        }
        Err(_) => Ok(Json(ApiResponse {
            success: false,
            data: Some(IfscValidationResponse {
                ifsc_code,
                valid: false,
                bank_data: None,
            }),
            message: "Invalid IFSC code".to_string(),
        })),
    }
}

async fn get_bank_data(
    State(state): State<AppState>,
    Path(ifsc_code): Path<String>,
) -> Result<Json<ApiResponse<BankData>>, StatusCode> {
    let ifsc_code = ifsc_code.to_uppercase();

    if let Some(bank_data) = state.memory_store.get(&ifsc_code).await {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(bank_data),
            message: "Bank data retrieved successfully".to_string(),
        }))
    } else {
        Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: "IFSC code not found".to_string(),
        }))
    }
}