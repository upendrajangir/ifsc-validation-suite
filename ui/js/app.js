
document.addEventListener('DOMContentLoaded', function() {
  // Get DOM elements
  const ifscInput = document.getElementById('ifscInput');
  const validateBtn = document.getElementById('validateBtn');
  const btnText = document.getElementById('btnText');
  const btnLoader = document.getElementById('btnLoader');
  const inputError = document.getElementById('inputError');

  // Create minimal overlay and modal
  const overlay = createMinimalOverlay();
  const resultModal = createMinimalModal();

  document.body.appendChild(overlay);
  document.body.appendChild(resultModal);

  /**
   * Creates clean overlay
   */
  function createMinimalOverlay() {
    const overlay = document.createElement('div');
    overlay.className = `
      fixed inset-0 bg-black/30 backdrop-blur-sm z-50
      opacity-0 pointer-events-none transition-all duration-300
    `;
    return overlay;
  }

  /**
   * Creates minimal modal
   */
  function createMinimalModal() {
    const modal = document.createElement('div');
    modal.className = `
      fixed inset-4 z-50 opacity-0 pointer-events-none
      transition-all duration-300 transform scale-95
    `;
    return modal;
  }

  /**
   * IFSC validation
   */
  function validateIFSC(ifsc) {
    if (!ifsc || typeof ifsc !== 'string') return false;
    ifsc = ifsc.trim().toUpperCase();
    const ifscRegex = /^[A-Z]{4}0[A-Z0-9]{6}$/;
    return ifscRegex.test(ifsc) && ifsc.length === 11;
  }

  /**
   * Show loading state
   */
  function showLoading() {
    btnText.classList.add('hidden');
    btnLoader.classList.remove('hidden');
    validateBtn.disabled = true;
    validateBtn.classList.add('opacity-70');

    showModal();
    showLoadingModal();
    document.body.style.overflow = 'hidden';
  }

  /**
   * Hide loading state
   */
  function hideLoading() {
    btnText.classList.remove('hidden');
    btnLoader.classList.add('hidden');
    validateBtn.disabled = false;
    validateBtn.classList.remove('opacity-70');
  }

  /**
   * Show modal
   */
  function showModal() {
    overlay.style.pointerEvents = 'auto';
    overlay.style.opacity = '1';
    resultModal.style.pointerEvents = 'auto';
    resultModal.style.opacity = '1';
    resultModal.style.transform = 'scale(1)';
  }

  /**
   * Hide modal
   */
  function hideModal() {
    overlay.style.opacity = '0';
    overlay.style.pointerEvents = 'none';
    resultModal.style.opacity = '0';
    resultModal.style.pointerEvents = 'none';
    resultModal.style.transform = 'scale(0.95)';

    document.body.style.overflow = 'auto';

    setTimeout(() => {
      if (ifscInput) ifscInput.value = '';
      hideError();
    }, 300);
  }

  /**
   * Show loading modal
   */
  function showLoadingModal() {
    resultModal.innerHTML = `
      <div class="h-full flex items-center justify-center">
        <div class="bg-white rounded-3xl p-8 max-w-sm w-full mx-4 text-center shadow-xl">
          <div class="w-16 h-16 bg-gradient-to-r from-primary to-secondary rounded-2xl mx-auto mb-6 flex items-center justify-center">
            <i class="ri-loader-4-line text-white text-2xl animate-spin"></i>
          </div>
          <h3 class="text-xl font-semibold text-gray-900 mb-2">Validating IFSC</h3>
          <p class="text-gray-600">Please wait a moment...</p>
        </div>
      </div>
    `;
  }

  /**
   * Show success result
   */
  function showSuccess(data) {
    const bank = data.bank_data;

    resultModal.innerHTML = `
      <div class="h-full flex flex-col bg-white rounded-3xl mx-4 overflow-hidden shadow-2xl">

        <!-- Header -->
        <div class="bg-gradient-to-r from-primary to-secondary p-6 text-white">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="w-12 h-12 bg-white/20 rounded-xl flex items-center justify-center">
                <i class="ri-check-line text-white text-xl"></i>
              </div>
              <div>
                <h2 class="text-lg font-semibold">Valid IFSC Code</h2>
                <p class="text-teal-100 text-sm">${bank.ifsc}</p>
              </div>
            </div>
            <button class="close-btn w-10 h-10 bg-white/20 hover:bg-white/30 rounded-xl flex items-center justify-center transition-colors">
              <i class="ri-close-line text-white"></i>
            </button>
          </div>
        </div>

        <!-- Content -->
        <div class="flex-1 p-6 overflow-y-auto">
          <div class="space-y-6">

            <!-- Bank Info -->
            <div>
              <h3 class="text-2xl font-bold text-gray-900 mb-1">${bank.bank}</h3>
              <p class="text-lg text-gray-600 font-medium">${bank.branch}</p>
            </div>

            <!-- Details -->
            <div class="space-y-4">
              <div class="flex justify-between py-2 border-b border-gray-100">
                <span class="text-gray-600 font-medium">City</span>
                <span class="text-gray-900 font-semibold">${bank.city}</span>
              </div>

              ${bank.contact ? `
              <div class="flex justify-between py-2 border-b border-gray-100">
                <span class="text-gray-600 font-medium">Phone</span>
                <a href="tel:${bank.contact}" class="text-primary font-semibold hover:underline">${bank.contact}</a>
              </div>
              ` : ''}

              <div class="py-2">
                <span class="text-gray-600 font-medium block mb-2">Address</span>
                <p class="text-gray-900 leading-relaxed">${bank.address}</p>
              </div>
            </div>

            <!-- Services -->
            <div>
              <h4 class="font-semibold text-gray-900 mb-3">Services</h4>
              <div class="flex space-x-3">
                ${createServiceBadge('RTGS', bank.rtgs)}
                ${createServiceBadge('NEFT', bank.neft)}
                ${createServiceBadge('IMPS', bank.imps)}
              </div>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="p-6 border-t border-gray-100">
          <div class="flex space-x-3">
            <button class="copy-btn flex-1 bg-gray-100 hover:bg-gray-200 text-gray-900 py-3 rounded-xl font-medium transition-colors">
              <i class="ri-file-copy-line mr-2"></i>Copy
            </button>
            <button class="another-btn flex-1 bg-gradient-to-r from-primary to-secondary text-white py-3 rounded-xl font-medium hover:from-secondary hover:to-primary transition-all">
              Validate Another
            </button>
          </div>
        </div>
      </div>
    `;

    addSuccessListeners(bank);
    hideLoading();
  }

  /**
   * Show error result
   */
  function showError() {
    resultModal.innerHTML = `
      <div class="h-full flex items-center justify-center">
        <div class="bg-white rounded-3xl p-8 max-w-sm w-full mx-4 text-center shadow-xl">
          <div class="w-16 h-16 bg-red-500 rounded-2xl mx-auto mb-6 flex items-center justify-center">
            <i class="ri-close-line text-white text-2xl"></i>
          </div>
          <h3 class="text-xl font-semibold text-gray-900 mb-2">Invalid IFSC Code</h3>
          <p class="text-gray-600 mb-6">Please check the code and try again</p>
          <div class="space-y-3">
            <button class="try-again-btn w-full bg-gradient-to-r from-primary to-secondary text-white py-3 rounded-xl font-medium">
              Try Again
            </button>
            <button class="close-error-btn w-full bg-gray-100 text-gray-900 py-3 rounded-xl font-medium">
              Close
            </button>
          </div>
        </div>
      </div>
    `;

    addErrorListeners();
    hideLoading();
  }

  /**
   * Create service badge
   */
  function createServiceBadge(service, isActive) {
    const bgClass = isActive ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800';
    const icon = isActive ? 'ri-check-line' : 'ri-close-line';

    return `
      <div class="px-3 py-1 ${bgClass} rounded-lg text-sm font-medium flex items-center">
        <i class="${icon} mr-1"></i>${service}
      </div>
    `;
  }

  /**
   * Add success event listeners
   */
  function addSuccessListeners(bank) {
    const closeBtn = resultModal.querySelector('.close-btn');
    const copyBtn = resultModal.querySelector('.copy-btn');
    const anotherBtn = resultModal.querySelector('.another-btn');

    closeBtn?.addEventListener('click', hideModal);
    copyBtn?.addEventListener('click', () => copyDetails(bank, copyBtn));
    anotherBtn?.addEventListener('click', hideModal);
  }

  /**
   * Add error event listeners
   */
  function addErrorListeners() {
    const tryAgainBtn = resultModal.querySelector('.try-again-btn');
    const closeErrorBtn = resultModal.querySelector('.close-error-btn');

    tryAgainBtn?.addEventListener('click', hideModal);
    closeErrorBtn?.addEventListener('click', hideModal);
  }

  /**
   * Copy bank details
   */
  function copyDetails(bank, button) {
    const text = `Bank: ${bank.bank}\nBranch: ${bank.branch}\nIFSC: ${bank.ifsc}\nCity: ${bank.city}\nAddress: ${bank.address}${bank.contact ? `\nPhone: ${bank.contact}` : ''}`;

    if (navigator.clipboard) {
      navigator.clipboard.writeText(text).then(() => {
        const original = button.innerHTML;
        button.innerHTML = '<i class="ri-check-line mr-2"></i>Copied!';
        button.classList.add('bg-green-100', 'text-green-800');

        setTimeout(() => {
          button.innerHTML = original;
          button.classList.remove('bg-green-100', 'text-green-800');
        }, 2000);
      });
    }
  }

  /**
   * API validation
   */
  async function validateWithAPI(ifscCode) {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 10000);

    try {
      const response = await fetch(`https://ifsc.guardor.in/validate/${ifscCode}`, {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
        },
        signal: controller.signal
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      clearTimeout(timeoutId);
      throw error;
    }
  }

  /**
   * Main validation function
   */
  async function performValidation() {
    if (!ifscInput) return;

    const ifscCode = ifscInput.value.trim().toUpperCase();

    hideError();

    if (!ifscCode) {
      showError('Please enter an IFSC code');
      return;
    }

    if (!validateIFSC(ifscCode)) {
      showError('Please enter a valid IFSC code format');
      return;
    }

    showLoading();

    try {
      const response = await validateWithAPI(ifscCode);

      if (response.success && response.data && response.data.valid) {
        showSuccess(response.data);
      } else {
        showError();
      }
    } catch (error) {
      console.error('Validation error:', error);
      showError();
    }
  }

  /**
   * Show input error
   */
  function showError(message) {
    if (!inputError || !ifscInput) return;

    if (message) {
      inputError.querySelector('span').textContent = message;
      inputError.classList.remove('hidden');
      ifscInput.classList.add('border-red-300', 'shake');

      setTimeout(() => {
        ifscInput.classList.remove('shake');
      }, 400);
    }
  }

  /**
   * Hide input error
   */
  function hideError() {
    if (inputError) inputError.classList.add('hidden');
    if (ifscInput) ifscInput.classList.remove('border-red-300');
  }

  // Event listeners
  if (validateBtn) {
    validateBtn.addEventListener('click', performValidation);
  }

  if (ifscInput) {
    ifscInput.addEventListener('input', function(e) {
      let value = e.target.value.toUpperCase().replace(/[^A-Z0-9]/g, '');
      if (value.length > 11) value = value.slice(0, 11);
      e.target.value = value;
      hideError();
    });

    ifscInput.addEventListener('keypress', function(e) {
      if (e.key === 'Enter') {
        e.preventDefault();
        performValidation();
      }
    });
  }

  // Close modal on outside click
  overlay.addEventListener('click', function(e) {
    if (e.target === overlay) hideModal();
  });

  // Escape key to close
  document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && overlay.style.opacity === '1') {
      hideModal();
    }
  });

  console.log('✨ Minimal Guardor IFSC Validator initialized');
});
