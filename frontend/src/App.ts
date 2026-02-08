import { shortenUrl, ApiError } from './api/client';
import { createShortenForm } from './components/ShortenForm';
import { createResultCard } from './components/ResultCard';
import { createInstallGuide } from './components/InstallGuide';
import './styles/themes.css';
import './styles/main.css';

/**
 * Main Application class
 * Orchestrates all components and handles application state
 */
export class App {
  private appElement: HTMLElement;
  private formControls: ReturnType<typeof createShortenForm> | null = null;
  private resultCard: ReturnType<typeof createResultCard> | null = null;

  constructor() {
    // Get or create app container
    this.appElement = document.getElementById('app') as HTMLElement;
    if (!this.appElement) {
      throw new Error('App container not found');
    }
  }

  /**
   * Initialize and render the application
   */
  init(): void {
    // Create main container
    const container = document.createElement('div');
    container.className = 'container';

    // Create card
    const card = document.createElement('div');
    card.className = 'card';

    // Create header
    const header = `
      <div class="logo">🔗</div>
      <h1>cutl URL Shortener</h1>
    `;

    // Create form container
    const formContainer = document.createElement('div');
    formContainer.id = 'form-container';

    // Create error message container
    const errorContainer = document.createElement('div');
    errorContainer.id = 'error';
    errorContainer.className = 'error';

    // Create result container
    const resultContainer = document.createElement('div');
    resultContainer.id = 'result-container';

    // Create install guide container
    const installContainer = document.createElement('div');
    installContainer.id = 'install-container';

    // Assemble the card
    card.innerHTML = header;
    card.appendChild(formContainer);
    card.appendChild(errorContainer);
    card.appendChild(resultContainer);
    card.appendChild(installContainer);

    container.appendChild(card);
    this.appElement.appendChild(container);

    // Initialize components
    this.initializeComponents(formContainer, resultContainer, installContainer, errorContainer);
  }

  /**
   * Initialize all components
   */
  private initializeComponents(
    formContainer: HTMLElement,
    resultContainer: HTMLElement,
    installContainer: HTMLElement,
    _errorContainer: HTMLElement
  ): void {
    // Create form
    this.formControls = createShortenForm(formContainer, {
      onSubmit: this.handleSubmit.bind(this),
      isLoading: false,
    });

    // Create result card
    this.resultCard = createResultCard(resultContainer);

    // Create install guide
    createInstallGuide(installContainer);
  }

  /**
   * Handle form submission
   */
  private async handleSubmit(request: { url: string; code?: string; ttl?: string }): Promise<void> {
    // Hide previous errors and results
    this.hideError();
    this.resultCard?.hide();

    // Set loading state
    this.formControls?.setLoading(true);

    try {
      // Call API
      const response = await shortenUrl(request);

      // Show result
      this.resultCard?.show(response);

      // Scroll to result
      const resultDiv = document.getElementById('result');
      if (resultDiv) {
        resultDiv.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      }
    } catch (error) {
      // Handle error
      if (error instanceof ApiError) {
        this.showError(error.message);
      } else {
        this.showError('An unexpected error occurred. Please try again.');
      }

      // Shake form to indicate error
      const form = this.formControls?.getElement();
      if (form) {
        form.style.animation = 'shake 0.3s';
        setTimeout(() => {
          form.style.animation = '';
        }, 300);
      }
    } finally {
      // Reset loading state
      this.formControls?.setLoading(false);
    }
  }

  /**
   * Show error message
   */
  private showError(message: string): void {
    const errorDiv = document.getElementById('error');
    if (errorDiv) {
      errorDiv.textContent = message;
      errorDiv.classList.add('show');
    }
  }

  /**
   * Hide error message
   */
  private hideError(): void {
    const errorDiv = document.getElementById('error');
    if (errorDiv) {
      errorDiv.classList.remove('show');
    }
  }

  /**
   * Reset the application state
   */
  public reset(): void {
    this.hideError();
    this.resultCard?.hide();
    this.formControls?.reset();
  }
}

/**
 * Initialize the application when DOM is ready
 */
export function initApp(): App {
  // Wait for DOM to be ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      const app = new App();
      app.init();
      return app;
    });
  }

  const app = new App();
  app.init();
  return app;
}
