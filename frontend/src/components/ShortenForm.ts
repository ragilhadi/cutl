import { validateUrl, validateCode, validateTtl } from '../utils/validation';
import type { ShortenRequest } from '../api/types';

interface ShortenFormOptions {
  onSubmit: (request: ShortenRequest) => void;
}

const TTL_PRESETS = [
  { label: '1h', value: '1h' },
  { label: '1d', value: '1d' },
  { label: '7d', value: '7d' },
  { label: '30d', value: '30d' },
];
const DEFAULT_TTL = '7d';

// ── Classes ──────────────────────────────────────────────────────
const inputBase =
  'w-full rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 ' +
  'px-3 py-2.5 text-sm text-slate-900 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-500 ' +
  'focus:outline-none focus:ring-2 focus:ring-indigo-500/60 focus:border-indigo-500 dark:focus:border-indigo-400 ' +
  'transition-colors';
const inputError =
  'border-red-400 dark:border-red-600 focus:ring-red-400/40 focus:border-red-400 dark:focus:border-red-500';
const labelCls =
  'block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5';
const helpCls =
  'text-xs text-slate-400 dark:text-slate-500 mt-1';
const fieldErrorCls =
  'text-xs text-red-600 dark:text-red-400 mt-1';

export function createShortenForm(container: HTMLElement, options: ShortenFormOptions) {
  const { onSubmit } = options;

  container.innerHTML = `
    <form id="shorten-form" class="space-y-5" novalidate>

      <!-- URL -->
      <div>
        <label for="url" class="${labelCls}">URL to shorten <span class="text-red-500">*</span></label>
        <input type="url" id="url" name="url"
          placeholder="https://example.com/very-long-url"
          autocomplete="url"
          class="${inputBase}" />
        <p id="url-error" class="${fieldErrorCls} hidden"></p>
      </div>

      <!-- Custom code -->
      <div>
        <label for="code" class="${labelCls}">Custom code <span class="font-normal text-slate-400 dark:text-slate-500">(optional)</span></label>
        <input type="text" id="code" name="code"
          placeholder="my-link"
          maxlength="32"
          autocomplete="off"
          class="${inputBase}" />
        <div class="flex justify-between mt-1">
          <p id="code-error" class="${fieldErrorCls} hidden"></p>
          <span id="code-counter" class="text-xs text-slate-400 dark:text-slate-500 ml-auto">32 remaining</span>
        </div>
      </div>

      <!-- TTL -->
      <div>
        <label class="${labelCls}">Expiration</label>
        <div id="ttl-presets" class="flex gap-2 mb-2 flex-wrap">
          ${TTL_PRESETS.map(p => `
            <button type="button" data-ttl="${p.value}"
              class="ttl-preset px-3 py-1.5 rounded-md text-sm font-medium border transition-colors
                     ${p.value === DEFAULT_TTL
                       ? 'bg-indigo-600 border-indigo-600 text-white'
                       : 'bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-700 text-slate-600 dark:text-slate-400 hover:border-indigo-400 dark:hover:border-indigo-500'}">
              ${p.label}
            </button>`).join('')}
        </div>
        <input type="text" id="ttl" name="ttl"
          placeholder="7d" value="${DEFAULT_TTL}"
          class="${inputBase}" />
        <p class="${helpCls}">Format: 5m, 1h, 3d, 30d &mdash; min 5 min, max 30 days</p>
        <p id="ttl-error" class="${fieldErrorCls} hidden"></p>
      </div>

      <!-- Submit -->
      <button type="submit" id="submit-btn"
        class="w-full py-2.5 px-4 rounded-lg bg-indigo-600 hover:bg-indigo-700 active:bg-indigo-800
               text-white text-sm font-semibold transition-colors shadow-sm
               disabled:opacity-60 disabled:cursor-not-allowed flex items-center justify-center gap-2 cursor-pointer">
        <span id="btn-icon" class="hidden">
          <svg class="animate-spin" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
            fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
        </span>
        <span id="btn-text">Shorten URL</span>
      </button>

    </form>
  `;

  const form = container.querySelector('#shorten-form') as HTMLFormElement;
  const urlInput = container.querySelector('#url') as HTMLInputElement;
  const codeInput = container.querySelector('#code') as HTMLInputElement;
  const ttlInput = container.querySelector('#ttl') as HTMLInputElement;
  const codeCounter = container.querySelector('#code-counter') as HTMLElement;
  const submitBtn = container.querySelector('#submit-btn') as HTMLButtonElement;
  const btnText = container.querySelector('#btn-text') as HTMLElement;
  const btnIcon = container.querySelector('#btn-icon') as HTMLElement;
  const presetBtns = container.querySelectorAll<HTMLButtonElement>('.ttl-preset');

  let selectedTtl = DEFAULT_TTL;

  // ── Character counter ─────────────────────────────────────────
  codeInput.addEventListener('input', () => {
    const rem = 32 - codeInput.value.length;
    codeCounter.textContent = `${rem} remaining`;
    codeCounter.classList.toggle('text-amber-500', rem < 8 && rem > 0);
    codeCounter.classList.toggle('text-red-500', rem === 0);
  });

  // ── TTL presets ───────────────────────────────────────────────
  const activePresetCls = ['bg-indigo-600', 'border-indigo-600', 'text-white'];
  const inactivePresetCls = ['bg-white', 'dark:bg-slate-800', 'border-slate-200', 'dark:border-slate-700', 'text-slate-600', 'dark:text-slate-400', 'hover:border-indigo-400', 'dark:hover:border-indigo-500'];

  function setActivePreset(value: string | null) {
    presetBtns.forEach(btn => {
      const isActive = btn.getAttribute('data-ttl') === value;
      activePresetCls.forEach(c => btn.classList.toggle(c, isActive));
      inactivePresetCls.forEach(c => btn.classList.toggle(c, !isActive));
    });
  }

  ttlInput.addEventListener('input', () => setActivePreset(null));

  presetBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      selectedTtl = btn.getAttribute('data-ttl') ?? DEFAULT_TTL;
      ttlInput.value = selectedTtl;
      setActivePreset(selectedTtl);
      clearFieldError(ttlInput, 'ttl-error');
    });
  });

  // ── Validation helpers ────────────────────────────────────────
  function showFieldError(input: HTMLInputElement, errorId: string, message: string) {
    input.className = `${inputBase} ${inputError}`;
    const errEl = container.querySelector(`#${errorId}`) as HTMLElement;
    errEl.textContent = message;
    errEl.classList.remove('hidden');
  }

  function clearFieldError(input: HTMLInputElement, errorId: string) {
    input.className = inputBase;
    const errEl = container.querySelector(`#${errorId}`) as HTMLElement;
    errEl.classList.add('hidden');
  }

  urlInput.addEventListener('input', () => clearFieldError(urlInput, 'url-error'));
  codeInput.addEventListener('input', () => clearFieldError(codeInput, 'code-error'));
  ttlInput.addEventListener('input', () => clearFieldError(ttlInput, 'ttl-error'));

  // ── Submit ────────────────────────────────────────────────────
  form.addEventListener('submit', (e) => {
    e.preventDefault();

    const urlErr = validateUrl(urlInput.value);
    const codeErr = validateCode(codeInput.value);
    const ttlErr = validateTtl(ttlInput.value);

    if (urlErr || codeErr || ttlErr) {
      if (urlErr) showFieldError(urlInput, 'url-error', urlErr);
      if (codeErr) showFieldError(codeInput, 'code-error', codeErr);
      if (ttlErr) showFieldError(ttlInput, 'ttl-error', ttlErr);
      return;
    }

    const req: ShortenRequest = { url: urlInput.value.trim() };
    if (codeInput.value.trim()) req.code = codeInput.value.trim();
    if (ttlInput.value.trim()) req.ttl = ttlInput.value.trim();
    onSubmit(req);
  });

  // ── Public API ────────────────────────────────────────────────
  return {
    setLoading(loading: boolean) {
      submitBtn.disabled = loading;
      btnIcon.classList.toggle('hidden', !loading);
      btnText.textContent = loading ? 'Shortening…' : 'Shorten URL';
    },
    reset() {
      form.reset();
      clearFieldError(urlInput, 'url-error');
      clearFieldError(codeInput, 'code-error');
      clearFieldError(ttlInput, 'ttl-error');
      codeCounter.textContent = '32 remaining';
      codeCounter.className = 'text-xs text-slate-400 dark:text-slate-500 ml-auto';
      ttlInput.value = DEFAULT_TTL;
      selectedTtl = DEFAULT_TTL;
      setActivePreset(DEFAULT_TTL);
    },
    getElement: () => form,
  };
}


