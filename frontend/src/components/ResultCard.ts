import { copyToClipboard, formatExpirationDate, getTimeRemaining } from '../utils/clipboard';
import type { ShortenResponse } from '../api/types';

/**
 * Creates a result card component to display the shortened URL
 * @param container - DOM element to append the result to
 * @returns Object with control methods
 */
export function createResultCard(container: HTMLElement) {
  // Create result HTML
  const resultHtml = `
    <div id="result" class="result">
      <div class="result-label">Your shortened URL:</div>
      <div class="result-url">
        <input type="text" id="short-url" readonly>
        <button type="button" class="btn-secondary" id="copy-btn">Copy</button>
      </div>
      <div class="result-expires">
        <span id="expires-at"></span>
        <span id="time-remaining" style="margin-left: var(--spacing-2);"></span>
      </div>
    </div>
  `;

  container.innerHTML = resultHtml;

  const resultDiv = container.querySelector('#result') as HTMLElement;
  const shortUrlInput = container.querySelector('#short-url') as HTMLInputElement;
  const copyBtn = container.querySelector('#copy-btn') as HTMLButtonElement;
  const expiresAtSpan = container.querySelector('#expires-at') as HTMLElement;
  const timeRemainingSpan = container.querySelector('#time-remaining') as HTMLElement;

  // Copy button functionality
  copyBtn.addEventListener('click', async () => {
    try {
      await copyToClipboard(shortUrlInput.value);

      // Visual feedback
      const originalText = copyBtn.textContent;
      copyBtn.textContent = 'Copied!';
      copyBtn.classList.add('success');

      setTimeout(() => {
        copyBtn.textContent = originalText;
        copyBtn.classList.remove('success');
      }, 2000);
    } catch (error) {
      // Show error toast
      showToast('Failed to copy to clipboard', 'error');
    }
  });

  // Helper function to show toast notification
  function showToast(message: string, type: 'success' | 'error' = 'success'): void {
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    document.body.appendChild(toast);

    setTimeout(() => {
      toast.remove();
    }, 3000);
  }

  // Return control methods
  return {
    /**
     * Shows the result with the shortened URL data
     */
    show: (data: ShortenResponse) => {
      shortUrlInput.value = data.short_url;
      expiresAtSpan.textContent = `Valid until ${formatExpirationDate(data.expires_at)}`;

      // Update time remaining
      const updateRemaining = () => {
        timeRemainingSpan.textContent = `(${getTimeRemaining(data.expires_at)} remaining)`;
      };
      updateRemaining();

      // Update time remaining every minute
      const interval = setInterval(updateRemaining, 60000);

      // Store interval ID for cleanup
      (resultDiv as any)._timeRemainingInterval = interval;

      resultDiv.classList.add('show');

      // Scroll result into view
      resultDiv.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    },

    /**
     * Hides the result card
     */
    hide: () => {
      resultDiv.classList.remove('show');

      // Clear time remaining interval
      const interval = (resultDiv as any)._timeRemainingInterval;
      if (interval) {
        clearInterval(interval);
        delete (resultDiv as any)._timeRemainingInterval;
      }
    },

    /**
     * Checks if the result is currently visible
     */
    isVisible: () => {
      return resultDiv.classList.contains('show');
    },
  };
}
