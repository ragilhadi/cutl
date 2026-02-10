/**
 * Creates the installation guide component with tabbed interface
 * @param container - DOM element to append the guide to
 */
export function createInstallGuide(container: HTMLElement) {
  const guideHtml = `
    <div class="install-section">
      <h2>📦 Install cutlcli</h2>

      <div class="tabs">
        <button class="tab active" data-tab="quick">Quick Install</button>
        <button class="tab" data-tab="manual">Manual Install</button>
        <button class="tab" data-tab="source">Build from Source</button>
        <button class="tab" data-tab="config">Configuration</button>
        <button class="tab" data-tab="usage">Usage</button>
      </div>

      <div class="tab-content active" data-content="quick">
        <h3>Quick Install (Recommended)</h3>
        
        <h4>Linux / macOS / Git Bash</h4>
        <p>Run this one-line command to install the latest version:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="quick-install-unix">Copy</button>
          <code id="quick-install-unix">curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh | bash</code>
        </div>

        <h4>Windows (PowerShell)</h4>
        <p>Run this command in PowerShell:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="quick-install-windows">Copy</button>
          <code id="quick-install-windows">irm https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 | iex</code>
        </div>

        <p>The installer will:</p>
        <ul>
          <li>Detect your OS and architecture automatically</li>
          <li>Download the appropriate binary from the latest GitHub release</li>
          <li>Install to <code>~/.local/bin/cutl</code> (Linux/macOS) or <code>%LOCALAPPDATA%\cutl\bin</code> (Windows)</li>
          <li>Make the binary executable</li>
        </ul>

        <h3>Supported Platforms</h3>
        <ul>
          <li>Linux (x86_64, aarch64)</li>
          <li>macOS (x86_64, aarch64/Apple Silicon)</li>
          <li>Windows (x86_64)</li>
        </ul>
      </div>

      <div class="tab-content" data-content="manual">
        <h3>Manual Install</h3>
        
        <h4>Linux / macOS / Git Bash</h4>
        <p>Download and inspect the script first:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="manual-install-unix">Copy</button>
          <code id="manual-install-unix">curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh -o install.sh
chmod +x install.sh
./install.sh</code>
        </div>

        <h4>Windows (PowerShell)</h4>
        <p>Download and inspect the script first:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="manual-install-windows">Copy</button>
          <code id="manual-install-windows">Invoke-WebRequest -Uri https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 -OutFile install.ps1
.\install.ps1</code>
        </div>
      </div>

      <div class="tab-content" data-content="source">
        <h3>Build from Source</h3>
        <p>If you prefer to build from source, you'll need Rust installed:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="source-install">Copy</button>
          <code id="source-install">git clone https://github.com/ragilhadi/cutl.git
cd cutl
cargo build --release
sudo cp target/release/cutl ~/.local/bin/</code>
        </div>
      </div>

      <div class="tab-content" data-content="config">
        <h3>Configuration</h3>
        <p>Set the server URL (defaults to https://cutl.my.id):</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="config-server">Copy</button>
          <code id="config-server">export CUTL_SERVER="https://your-cutl-instance.com"</code>
        </div>

        <p>If your server requires authentication:</p>
        <div class="code-block">
          <button class="code-block-copy" data-code="config-token">Copy</button>
          <code id="config-token">export CUTL_TOKEN="your-auth-token"</code>
        </div>

        <p>For persistent configuration, add these to your shell profile (~/.bashrc, ~/.zshrc, etc.)</p>
      </div>

      <div class="tab-content" data-content="usage">
        <h3>Usage Examples</h3>
        <div class="code-block">
          <button class="code-block-copy" data-code="usage-examples">Copy</button>
          <code id="usage-examples"># Basic usage
cutl https://example.com

# Custom expiration
cutl https://example.com --ttl 3d

# Custom code
cutl https://example.com --code mylink

# Both options
cutl https://example.com --code docs --ttl 7d</code>
        </div>
      </div>
    </div>
  `;

  container.innerHTML = guideHtml;

  // Tab switching functionality
  const tabs = container.querySelectorAll('.tab');
  const tabContents = container.querySelectorAll('.tab-content');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      const tabName = tab.getAttribute('data-tab');

      // Remove active class from all tabs and contents
      tabs.forEach(t => t.classList.remove('active'));
      tabContents.forEach(c => c.classList.remove('active'));

      // Add active class to clicked tab and corresponding content
      tab.classList.add('active');
      const content = container.querySelector(`[data-content="${tabName}"]`);
      if (content) {
        content.classList.add('active');
      }
    });
  });

  // Copy button functionality for code blocks
  const copyButtons = container.querySelectorAll('.code-block-copy');

  copyButtons.forEach(btn => {
    btn.addEventListener('click', async () => {
      const codeId = btn.getAttribute('data-code');
      if (!codeId) return;

      const codeElement = container.querySelector(`#${codeId}`);
      if (!codeElement) return;

      const code = codeElement.textContent || '';

      try {
        await navigator.clipboard.writeText(code);

        // Visual feedback
        btn.textContent = 'Copied!';
        btn.classList.add('copied');

        setTimeout(() => {
          btn.textContent = 'Copy';
          btn.classList.remove('copied');
        }, 2000);
      } catch (error) {
        // Fallback for older browsers
        const textArea = document.createElement('textarea');
        textArea.value = code;
        textArea.style.position = 'fixed';
        textArea.style.left = '-9999px';
        document.body.appendChild(textArea);
        textArea.select();

        try {
          document.execCommand('copy');
          btn.textContent = 'Copied!';
          btn.classList.add('copied');
          setTimeout(() => {
            btn.textContent = 'Copy';
            btn.classList.remove('copied');
          }, 2000);
        } catch (err) {
          console.error('Failed to copy:', err);
        }

        document.body.removeChild(textArea);
      }
    });
  });
}
