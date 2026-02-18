import { shortenUrl, ApiError } from './api/client';
import { createShortenForm } from './components/ShortenForm';
import { createResultCard } from './components/ResultCard';
import { createInstallGuide } from './components/InstallGuide';

// ── Theme helpers ─────────────────────────────────────────────────
const THEME_KEY = 'cutl-theme';

function getTheme(): 'dark' | 'light' {
  const stored = localStorage.getItem(THEME_KEY);
  if (stored === 'dark' || stored === 'light') return stored;
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: 'dark' | 'light'): void {
  document.documentElement.classList.toggle('dark', theme === 'dark');
  localStorage.setItem(THEME_KEY, theme);
}

function toggleTheme(): void {
  const next = getTheme() === 'dark' ? 'light' : 'dark';
  applyTheme(next);
  const btn = document.getElementById('theme-toggle');
  if (btn) btn.innerHTML = themeIcon(next);
}

function themeIcon(theme: 'dark' | 'light'): string {
  return theme === 'dark'
    ? `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66
        17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>
      </svg>`
    : `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
      </svg>`;
}

// ── Helper ────────────────────────────────────────────────────────
function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  return e;
}

// ── App ───────────────────────────────────────────────────────────
export class App {
  private appElement: HTMLElement;
  private formControls: ReturnType<typeof createShortenForm> | null = null;
  private resultCard: ReturnType<typeof createResultCard> | null = null;

  constructor() {
    this.appElement = document.getElementById('app') as HTMLElement;
    if (!this.appElement) throw new Error('App container not found');
    applyTheme(getTheme());
  }

  init(): void {
    this.appElement.innerHTML = '';

    const page = el('div', 'min-h-dvh flex flex-col animate-fade-in');

    // ── Header ──────────────────────────────────────────────────
    const header = el('header', 'sticky top-0 z-10 border-b border-slate-200 dark:border-slate-800 bg-white/80 dark:bg-slate-950/80 backdrop-blur-sm');
    const headerInner = el('div', 'max-w-2xl mx-auto px-4 h-14 flex items-center justify-between');
    const brand = el('div', 'flex items-center gap-2');
    brand.innerHTML = `
      <span class="text-xl font-bold tracking-tight text-indigo-600 dark:text-indigo-400">cutl</span>
      <span class="text-slate-400 dark:text-slate-600 text-sm font-medium hidden sm:inline">URL Shortener</span>
    `;
    const themeBtn = el('button', 'p-2 rounded-lg text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors cursor-pointer');
    themeBtn.id = 'theme-toggle';
    themeBtn.setAttribute('aria-label', 'Toggle theme');
    themeBtn.innerHTML = themeIcon(getTheme());
    themeBtn.addEventListener('click', toggleTheme);
    headerInner.appendChild(brand);
    headerInner.appendChild(themeBtn);
    header.appendChild(headerInner);
    page.appendChild(header);

    // ── Main ────────────────────────────────────────────────────
    const main = el('main', 'flex-1');
    const mainInner = el('div', 'max-w-2xl mx-auto px-4 py-10 space-y-6');

    const hero = el('div', 'text-center space-y-2 mb-8');
    hero.innerHTML = `
      <h1 class="text-3xl sm:text-4xl font-bold text-slate-900 dark:text-slate-50 tracking-tight">
        Shorten any URL instantly
      </h1>
      <p class="text-slate-500 dark:text-slate-400 text-base">
        Free, open-source, self-hostable — custom codes and expiry included.
      </p>
    `;
    mainInner.appendChild(hero);

    const card = el('div', 'bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-2xl shadow-sm p-6 sm:p-8 space-y-5 animate-slide-up');

    const errorBanner = el('div', 'hidden rounded-lg bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-400 px-4 py-3 text-sm font-medium');
    errorBanner.id = 'error-banner';

    const formContainer = el('div', '');
    formContainer.id = 'form-container';

    const resultContainer = el('div', '');
    resultContainer.id = 'result-container';

    card.appendChild(errorBanner);
    card.appendChild(formContainer);
    card.appendChild(resultContainer);
    mainInner.appendChild(card);

    const installContainer = el('div', '');
    installContainer.id = 'install-container';
    mainInner.appendChild(installContainer);

    main.appendChild(mainInner);
    page.appendChild(main);

    // ── Footer ──────────────────────────────────────────────────
    const footer = el('footer', 'border-t border-slate-200 dark:border-slate-800 py-6');
    footer.innerHTML = `
      <p class="text-center text-xs text-slate-400 dark:text-slate-600">
        © ${new Date().getFullYear()} cutl — self-hosted URL shortener ·
        <a href="https://github.com/ragilhadi/cutl" target="_blank" rel="noopener"
           class="hover:text-indigo-500 transition-colors">GitHub</a>
      </p>
    `;
    page.appendChild(footer);
    this.appElement.appendChild(page);

    // Init components
    this.formControls = createShortenForm(formContainer, { onSubmit: this.handleSubmit.bind(this) });
    this.resultCard = createResultCard(resultContainer, () => this.reset());
    createInstallGuide(installContainer);
  }

  private async handleSubmit(req: { url: string; code?: string; ttl?: string }): Promise<void> {
    this.hideError();
    this.resultCard?.hide();
    this.formControls?.setLoading(true);
    try {
      const response = await shortenUrl(req);
      this.resultCard?.show(response);
      document.getElementById('result-container')?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    } catch (err) {
      const msg = err instanceof ApiError ? err.message : 'An unexpected error occurred. Please try again.';
      this.showError(msg);
    } finally {
      this.formControls?.setLoading(false);
    }
  }

  private showError(message: string): void {
    const banner = document.getElementById('error-banner');
    if (!banner) return;
    banner.textContent = message;
    banner.classList.remove('hidden');
    banner.classList.add('animate-shake');
    setTimeout(() => banner.classList.remove('animate-shake'), 400);
  }

  private hideError(): void {
    document.getElementById('error-banner')?.classList.add('hidden');
  }

  public reset(): void {
    this.hideError();
    this.resultCard?.hide();
    this.formControls?.reset();
  }
}

export function initApp(): App {
  const run = () => { const a = new App(); a.init(); return a; };
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => run());
  }
  return run();
}


