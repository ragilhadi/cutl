import { copyToClipboard, formatExpirationDate, getTimeRemaining } from '../utils/clipboard';
import type { ShortenResponse } from '../api/types';

// ── Toast ─────────────────────────────────────────────────────────
function showToast(message: string, variant: 'success' | 'error' = 'success'): void {
  const toast = document.createElement('div');
  toast.className = [
    'fixed bottom-6 right-6 z-50 flex items-center gap-2.5 px-4 py-3 rounded-xl shadow-lg text-sm font-medium',
    'animate-toast-in',
    variant === 'success'
      ? 'bg-emerald-600 text-white'
      : 'bg-red-600 text-white',
  ].join(' ');

  const icon = variant === 'success'
    ? `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
         stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
         <path d="M20 6 9 17l-5-5"/></svg>`
    : `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
         stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
         <path d="M18 6 6 18M6 6l12 12"/></svg>`;

  toast.innerHTML = icon + message;
  document.body.appendChild(toast);
  setTimeout(() => toast.remove(), 3000);
}

// ── Component ─────────────────────────────────────────────────────
export function createResultCard(container: HTMLElement, onReset?: () => void) {
  container.innerHTML = '';

  const wrapper = document.createElement('div');
  wrapper.id = 'result-card';
  wrapper.className = 'hidden';
  container.appendChild(wrapper);

  let intervalId: ReturnType<typeof setInterval> | null = null;

  function render(data: ShortenResponse): void {
    wrapper.innerHTML = `
      <div class="mt-1 rounded-xl bg-emerald-50 dark:bg-emerald-950/30 border border-emerald-200 dark:border-emerald-800 p-5 space-y-4 animate-slide-down">

        <!-- Label -->
        <div class="flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class="text-emerald-600 dark:text-emerald-400 shrink-0">
            <path d="M20 6 9 17l-5-5"/>
          </svg>
          <span class="text-sm font-semibold text-emerald-800 dark:text-emerald-300">URL shortened!</span>
        </div>

        <!-- Short URL row -->
        <div class="flex items-center gap-2">
          <input type="text" id="short-url-input" readonly
            value="${escapeAttr(data.short_url)}"
            class="flex-1 min-w-0 rounded-lg border border-emerald-200 dark:border-emerald-800
                   bg-white dark:bg-slate-900 px-3 py-2 text-sm font-mono
                   text-slate-900 dark:text-slate-100 focus:outline-none
                   focus:ring-2 focus:ring-indigo-500/50 transition-colors select-all" />
          <button id="copy-btn" type="button"
            class="shrink-0 px-3 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-700 active:bg-indigo-800
                   text-white text-sm font-medium transition-colors cursor-pointer flex items-center gap-1.5">
            <svg id="copy-icon" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
              <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
            </svg>
            <span id="copy-label">Copy</span>
          </button>
        </div>

        <!-- Open link -->
        <a href="${escapeAttr(data.short_url)}" target="_blank" rel="noopener"
           class="text-xs text-indigo-600 dark:text-indigo-400 hover:underline inline-flex items-center gap-1">
          Open link
          <svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M15 3h6v6M10 14 21 3M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
          </svg>
        </a>

        <!-- Expiry -->
        <div class="flex items-start justify-between flex-wrap gap-2">
          <p id="expiry-text" class="text-xs text-slate-500 dark:text-slate-400"></p>
          ${onReset ? `
          <button id="reset-btn" type="button"
            class="text-xs text-slate-500 dark:text-slate-400 hover:text-indigo-600 dark:hover:text-indigo-400
                   transition-colors underline-offset-2 hover:underline cursor-pointer">
            Shorten another
          </button>` : ''}
        </div>
      </div>
    `;

    // Update expiry
    const expiryText = wrapper.querySelector('#expiry-text') as HTMLElement;
    const update = () => {
      expiryText.textContent = `Expires ${formatExpirationDate(data.expires_at)} · ${getTimeRemaining(data.expires_at)} remaining`;
    };
    update();
    // Clear any existing interval before creating a new one
    if (intervalId) clearInterval(intervalId);
    intervalId = setInterval(update, 60_000);

    // Copy button
    const copyBtn = wrapper.querySelector('#copy-btn') as HTMLButtonElement;
    const copyLabel = wrapper.querySelector('#copy-label') as HTMLElement;
    const copyIcon = wrapper.querySelector('#copy-icon') as SVGElement;

    copyBtn.addEventListener('click', async () => {
      try {
        await copyToClipboard(data.short_url);
        copyLabel.textContent = 'Copied!';
        copyIcon.innerHTML = `<path d="M20 6 9 17l-5-5"/>`;
        copyBtn.classList.remove('bg-indigo-600', 'hover:bg-indigo-700');
        copyBtn.classList.add('bg-emerald-600', 'hover:bg-emerald-700');
        showToast('Copied to clipboard!');
        setTimeout(() => {
          copyLabel.textContent = 'Copy';
          copyIcon.innerHTML = `<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
            <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>`;
          copyBtn.classList.remove('bg-emerald-600', 'hover:bg-emerald-700');
          copyBtn.classList.add('bg-indigo-600', 'hover:bg-indigo-700');
        }, 2000);
      } catch {
        showToast('Failed to copy', 'error');
      }
    });

    // Reset button
    const resetBtn = wrapper.querySelector('#reset-btn');
    resetBtn?.addEventListener('click', () => onReset?.());
  }

  return {
    show(data: ShortenResponse) {
      render(data);
      wrapper.classList.remove('hidden');
    },
    hide() {
      wrapper.classList.add('hidden');
      wrapper.innerHTML = '';
      if (intervalId !== null) { clearInterval(intervalId); intervalId = null; }
    },
    isVisible: () => !wrapper.classList.contains('hidden'),
  };
}

function escapeAttr(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}


