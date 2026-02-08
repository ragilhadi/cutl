import { validateUrl, validateCode, validateTtl } from '../utils/validation';
import { getRemainingChars } from '../utils/clipboard';
import type { ShortenRequest } from '../api/types';

interface ShortenFormOptions {
  onSubmit: (request: ShortenRequest) => void;
  isLoading?: boolean;
}

/**
 * Creates the URL shortening form component
 * @param container - DOM element to append the form to
 * @param options - Configuration options
 * @returns Object with form control methods
 */
export function createShortenForm(
  container: HTMLElement,
  options: ShortenFormOptions
) {
  const { onSubmit } = options;

  // Create form HTML
  const formHtml = `
    <form id="shorten-form">
      <div class="form-group">
        <label for="url">URL to shorten *</label>
        <input
          type="url"
          id="url"
          name="url"
          placeholder="https://example.com/very-long-url"
          required
          autocomplete="url"
        >
      </div>

      <div class="form-group">
        <label for="code">Custom code (optional)</label>
        <input
          type="text"
          id="code"
          name="code"
          placeholder="my-custom-code"
          maxlength="32"
          autocomplete="off"
        >
        <div class="char-counter" id="code-counter">32 characters remaining</div>
      </div>

      <div class="form-group">
        <label for="ttl">Expiration time (optional)</label>
        <div class="ttl-presets">
          <button type="button" class="ttl-preset-btn" data-ttl="1h">1 hour</button>
          <button type="button" class="ttl-preset-btn" data-ttl="1d">1 day</button>
          <button type="button" class="ttl-preset-btn active" data-ttl="7d">7 days</button>
          <button type="button" class="ttl-preset-btn" data-ttl="30d">30 days</button>
        </div>
        <input
          type="text"
          id="ttl"
          name="ttl"
          placeholder="7d"
          value="7d"
        >
        <small class="help-text">
          Format: number + unit (s/m/h/d). Example: 5m, 1h, 3d, 30d. Default: 7d
        </small>
      </div>

      <button type="submit" id="submit-btn">
        <span id="btn-text">Shorten URL</span>
      </button>
    </form>
  `;

  container.innerHTML = formHtml;

  // Get form elements
  const form = container.querySelector('#shorten-form') as HTMLFormElement;
  const urlInput = container.querySelector('#url') as HTMLInputElement;
  const codeInput = container.querySelector('#code') as HTMLInputElement;
  const ttlInput = container.querySelector('#ttl') as HTMLInputElement;
  const codeCounter = container.querySelector('#code-counter') as HTMLElement;
  const submitBtn = container.querySelector('#submit-btn') as HTMLButtonElement;
  const btnText = container.querySelector('#btn-text') as HTMLElement;
  const presetBtns = container.querySelectorAll('.ttl-preset-btn');

  let selectedTtl = '7d';

  // Character counter for custom code
  codeInput.addEventListener('input', () => {
    const remaining = getRemainingChars(codeInput.value);
    codeCounter.textContent = `${remaining} character${remaining !== 1 ? 's' : ''} remaining`;

    if (remaining < 5) {
      codeCounter.classList.add('warning');
    } else {
      codeCounter.classList.remove('warning');
    }

    if (remaining === 0) {
      codeCounter.classList.add('error');
    } else {
      codeCounter.classList.remove('error');
    }
  });

  // TTL input - clear preset selection when typing custom value
  ttlInput.addEventListener('input', () => {
    presetBtns.forEach(btn => btn.classList.remove('active'));
  });

  // TTL preset buttons
  presetBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      presetBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      selectedTtl = btn.getAttribute('data-ttl') || '7d';
      ttlInput.value = selectedTtl;
    });
  });

  // Form submission
  form.addEventListener('submit', (e) => {
    e.preventDefault();

    // Validate all fields
    const urlError = validateUrl(urlInput.value);
    const codeError = validateCode(codeInput.value);
    const ttlError = validateTtl(ttlInput.value);

    // If any errors, show them and don't submit
    if (urlError || codeError || ttlError) {
      if (urlError) showInputError(urlInput, urlError);
      if (codeError) showInputError(codeInput, codeError);
      if (ttlError) showInputError(ttlInput, ttlError);
      return;
    }

    // Clear any previous errors
    clearAllErrors();

    // Build request
    const request: ShortenRequest = {
      url: urlInput.value.trim(),
    };

    if (codeInput.value.trim()) {
      request.code = codeInput.value.trim();
    }

    if (ttlInput.value.trim()) {
      request.ttl = ttlInput.value.trim();
    }

    onSubmit(request);
  });

  // Clear errors when user starts typing
  urlInput.addEventListener('input', () => clearFieldError(urlInput));
  codeInput.addEventListener('input', () => clearFieldError(codeInput));
  ttlInput.addEventListener('input', () => clearFieldError(ttlInput));

  // Helper function to clear error for a specific field
  function clearFieldError(input: HTMLInputElement): void {
    input.classList.remove('error');
    const fieldError = input.parentElement?.querySelector('.field-error');
    if (fieldError) {
      fieldError.remove();
    }
  }

  // Helper function to clear all field errors
  function clearAllErrors(): void {
    urlInput.classList.remove('error');
    codeInput.classList.remove('error');
    ttlInput.classList.remove('error');
    container.querySelectorAll('.field-error').forEach(el => el.remove());
  }

  // Helper function to show input error
  function showInputError(input: HTMLInputElement, message: string): void {
    input.classList.add('error');

    // Remove existing error message
    const existingError = input.parentElement?.querySelector('.field-error');
    if (existingError) {
      existingError.remove();
    }

    // Add error message
    const errorDiv = document.createElement('div');
    errorDiv.className = 'field-error';
    errorDiv.style.color = 'var(--color-error)';
    errorDiv.style.fontSize = 'var(--text-xs)';
    errorDiv.style.marginTop = 'var(--spacing-1)';
    errorDiv.textContent = message;
    input.parentElement?.appendChild(errorDiv);
  }

  // Return control methods
  return {
    /**
     * Sets the loading state of the form
     */
    setLoading: (loading: boolean) => {
      if (loading) {
        submitBtn.disabled = true;
        btnText.innerHTML = '<span class="spinner"></span>Shortening...';
      } else {
        submitBtn.disabled = false;
        btnText.textContent = 'Shorten URL';
      }
    },

    /**
     * Resets the form to its initial state
     */
    reset: () => {
      form.reset();
      clearAllErrors();
      codeCounter.textContent = '32 characters remaining';
      codeCounter.classList.remove('warning', 'error');

      // Reset TTL to default
      ttlInput.value = '7d';
      presetBtns.forEach(btn => {
        btn.classList.remove('active');
        if (btn.getAttribute('data-ttl') === '7d') {
          btn.classList.add('active');
        }
      });
    },

    /**
     * Gets the form element
     */
    getElement: () => form,
  };
}
