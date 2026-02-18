// ── Helpers ───────────────────────────────────────────────────────
function codeBlock(id: string, code: string): string {
  const escaped = code
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
  return `
    <div class="relative group mt-3 rounded-xl bg-slate-900 dark:bg-slate-950 border border-slate-700 dark:border-slate-800 overflow-hidden">
      <pre class="overflow-x-auto p-4 text-xs text-slate-100 font-mono leading-relaxed"><code id="${id}">${escaped}</code></pre>
      <button data-copy="${id}" type="button"
        class="absolute top-2.5 right-2.5 px-2 py-1 rounded-md text-xs font-medium
               bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white
               opacity-0 group-hover:opacity-100 focus:opacity-100 transition-all cursor-pointer">
        Copy
      </button>
    </div>`;
}

function tab(id: string, label: string, active = false): string {
  return `<button role="tab" data-tab="${id}"
    class="tab-btn px-4 py-2.5 text-sm font-medium rounded-lg whitespace-nowrap transition-colors
           ${active
             ? 'bg-white dark:bg-slate-800 text-indigo-600 dark:text-indigo-400 shadow-sm'
             : 'text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-300'}"
  >${label}</button>`;
}

// ── Component ─────────────────────────────────────────────────────
export function createInstallGuide(container: HTMLElement) {
  container.innerHTML = `
    <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-2xl shadow-sm overflow-hidden">

      <!-- Header -->
      <div class="px-6 sm:px-8 pt-6 pb-0 border-b border-slate-200 dark:border-slate-800">
        <h2 class="text-lg font-semibold text-slate-900 dark:text-slate-50 mb-4">
          Install the CLI
        </h2>
        <!-- Tabs bar -->
        <div role="tablist" class="flex gap-1 bg-slate-100 dark:bg-slate-800/60 rounded-xl p-1 w-fit max-w-full overflow-x-auto">
          ${tab('quick',  'Quick Install', true)}
          ${tab('manual', 'Manual')}
          ${tab('source', 'From Source')}
          ${tab('config', 'Config')}
          ${tab('usage',  'Usage')}
        </div>
        <div class="h-px"></div>
      </div>

      <!-- Tab content -->
      <div class="px-6 sm:px-8 py-6 space-y-5">

        <div id="tab-quick" class="tab-content space-y-5">
          <div>
            <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Linux / macOS / Git Bash</h3>
            <p class="text-sm text-slate-500 dark:text-slate-400 mt-1">One-line install:</p>
            ${codeBlock('quick-unix', 'curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh | bash')}
          </div>
          <div>
            <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Windows (PowerShell)</h3>
            ${codeBlock('quick-win', 'irm https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 | iex')}
          </div>
          <div class="rounded-lg bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700/60 px-4 py-3">
            <p class="text-xs font-semibold text-slate-600 dark:text-slate-400 mb-2">The installer will:</p>
            <ul class="text-xs text-slate-500 dark:text-slate-400 space-y-1 list-disc list-inside">
              <li>Auto-detect your OS and architecture</li>
              <li>Download the binary from the latest GitHub release</li>
              <li>Install to <code class="bg-slate-200 dark:bg-slate-700 px-1 rounded font-mono">~/.local/bin/cutl</code> (Linux/macOS) or <code class="bg-slate-200 dark:bg-slate-700 px-1 rounded font-mono">%LOCALAPPDATA%\\cutl\\bin</code> (Windows)</li>
            </ul>
            <p class="text-xs font-semibold text-slate-600 dark:text-slate-400 mt-3 mb-1">Platforms:</p>
            <ul class="text-xs text-slate-500 dark:text-slate-400 space-y-0.5 list-disc list-inside">
              <li>Linux (x86_64, aarch64)</li>
              <li>macOS (x86_64, Apple Silicon)</li>
              <li>Windows (x86_64)</li>
            </ul>
          </div>
        </div>

        <div id="tab-manual" class="tab-content hidden space-y-5">
          <div>
            <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Linux / macOS</h3>
            ${codeBlock('manual-unix', `curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh -o install.sh
chmod +x install.sh
./install.sh`)}
          </div>
          <div>
            <h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Windows (PowerShell)</h3>
            ${codeBlock('manual-win', `Invoke-WebRequest -Uri https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 -OutFile install.ps1
.\install.ps1`)}
          </div>
        </div>

        <div id="tab-source" class="tab-content hidden space-y-3">
          <p class="text-sm text-slate-500 dark:text-slate-400">Requires <a href="https://rustup.rs" target="_blank" rel="noopener" class="text-indigo-600 dark:text-indigo-400 hover:underline">Rust</a> installed.</p>
          ${codeBlock('source', `git clone https://github.com/ragilhadi/cutl.git
cd cutl
cargo build --release
sudo cp target/release/cutl ~/.local/bin/`)}
        </div>

        <div id="tab-config" class="tab-content hidden space-y-5">
          <div>
            <p class="text-sm text-slate-500 dark:text-slate-400">Set the server URL (defaults to <code class="bg-slate-100 dark:bg-slate-800 px-1 rounded font-mono text-xs">https://cutl.my.id</code>):</p>
            ${codeBlock('cfg-server', 'export CUTL_SERVER="https://your-cutl-instance.com"')}
          </div>
          <div>
            <p class="text-sm text-slate-500 dark:text-slate-400">If your server requires authentication:</p>
            ${codeBlock('cfg-token', 'export CUTL_TOKEN="your-auth-token"')}
          </div>
          <p class="text-xs text-slate-400 dark:text-slate-500">For persistent config, add these to your shell profile (<code class="font-mono">~/.bashrc</code>, <code class="font-mono">~/.zshrc</code>, etc.)</p>
        </div>

        <div id="tab-usage" class="tab-content hidden">
          ${codeBlock('usage', `# Basic usage
cutl https://example.com

# Custom expiration
cutl https://example.com --ttl 3d

# Custom code
cutl https://example.com --code mylink

# Both options
cutl https://example.com --code docs --ttl 7d`)}
        </div>

      </div>
    </div>
  `;

  // ── Tab switching ──────────────────────────────────────────────
  const tabBtns = container.querySelectorAll<HTMLButtonElement>('.tab-btn');
  const tabContents = container.querySelectorAll<HTMLElement>('.tab-content');

  const activeTabClasses = ['bg-white', 'dark:bg-slate-800', 'text-indigo-600', 'dark:text-indigo-400', 'shadow-sm'];
  const inactiveTabClasses = ['text-slate-500', 'dark:text-slate-400', 'hover:text-slate-700', 'dark:hover:text-slate-300'];

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const id = btn.getAttribute('data-tab');
      tabBtns.forEach(b => {
        const isActive = b === btn;
        // Remove all state classes first
        b.classList.remove(...activeTabClasses, ...inactiveTabClasses);
        // Add the appropriate state classes
        if (isActive) {
          b.classList.add(...activeTabClasses);
        } else {
          b.classList.add(...inactiveTabClasses);
        }
      });
      tabContents.forEach(c => {
        c.classList.toggle('hidden', c.id !== `tab-${id}`);
      });
    });
  });

  // ── Code copy buttons ──────────────────────────────────────────
  container.querySelectorAll<HTMLButtonElement>('button[data-copy]').forEach(btn => {
    btn.addEventListener('click', async () => {
      const codeId = btn.getAttribute('data-copy')!;
      const code = container.querySelector(`#${codeId}`)?.textContent ?? '';
      try {
        await navigator.clipboard.writeText(code);
        btn.textContent = 'Copied!';
        btn.classList.add('bg-emerald-700', '!opacity-100');
        setTimeout(() => {
          btn.textContent = 'Copy';
          btn.classList.remove('bg-emerald-700', '!opacity-100');
        }, 2000);
      } catch {
        // Fallback for older browsers that do not support navigator.clipboard
        try {
          const ta = document.createElement('textarea');
          ta.value = code;
          ta.style.cssText = 'position:fixed;left:-9999px;top:0';
          document.body.appendChild(ta);
          ta.select();
          const successful = document.execCommand('copy');
          document.body.removeChild(ta);

          if (successful) {
            btn.textContent = 'Copied!';
            btn.classList.add('bg-emerald-700', '!opacity-100');
            setTimeout(() => {
              btn.textContent = 'Copy';
              btn.classList.remove('bg-emerald-700', '!opacity-100');
            }, 2000);
          }
        } catch (fallbackError) {
          // Best-effort: if copying fails here, avoid claiming success
          console.error('Copy to clipboard failed', fallbackError);
        }
      }
    });
  });
}
