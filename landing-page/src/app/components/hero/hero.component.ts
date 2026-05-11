import { ChangeDetectionStrategy, Component, OnDestroy, signal } from '@angular/core';
import { Check, Copy, Github, LucideAngularModule } from 'lucide-angular';

@Component({
  selector: 'app-hero',
  standalone: true,
  imports: [LucideAngularModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <section id="top" class="hero-section">
      <div class="container hero-content">
        <div class="hero-copy">
          <p class="hero-kicker">Production-ready scaffolding</p>
          <h1 class="hero-title">shastack</h1>
          <p class="hero-tagline">The Unified Universal Stack CLI</p>
          <p class="hero-subtagline">One tool. Every domain. Production grade from day one.</p>

          <div class="hero-actions">
            <a
              class="button button--primary"
              href="https://github.com/shawal-mbalire/shastack"
              target="_blank"
              rel="noreferrer"
            >
              <lucide-icon [img]="Github" [size]="18"></lucide-icon>
              <span>View on GitHub</span>
            </a>
            <a
              class="button button--secondary"
              href="https://github.com/shawal-mbalire/shastack/tree/main/docs"
              target="_blank"
              rel="noreferrer"
            >
              Read Docs
            </a>
          </div>
        </div>

        <div class="install-card">
          <p class="install-label">Install with one command</p>
          <button
            type="button"
            class="install-block"
            (click)="copyCommand()"
            aria-label="Copy install command"
          >
            <code class="install-command">{{ installCommand }}</code>
            <span class="install-copy">
              <lucide-icon [img]="copied() ? Check : Copy" [size]="18"></lucide-icon>
              <span>{{ copied() ? 'Copied!' : 'Copy' }}</span>
            </span>
          </button>
          <p class="install-note">Click the command block to copy it to your clipboard.</p>
        </div>
      </div>
    </section>
  `
})
export class HeroComponent implements OnDestroy {
  readonly Check = Check;
  readonly Copy = Copy;
  readonly Github = Github;
  readonly installCommand =
    'curl -sSfL https://raw.githubusercontent.com/shawal-mbalire/shastack/main/cli/scripts/install.sh | bash';
  readonly copied = signal(false);

  private copiedTimerId?: number;

  async copyCommand(): Promise<void> {
    try {
      await navigator.clipboard.writeText(this.installCommand);
      this.copied.set(true);
      this.resetCopiedState();
    } catch {
      this.copied.set(false);
    }
  }

  ngOnDestroy(): void {
    if (this.copiedTimerId) {
      window.clearTimeout(this.copiedTimerId);
    }
  }

  private resetCopiedState(): void {
    if (this.copiedTimerId) {
      window.clearTimeout(this.copiedTimerId);
    }

    this.copiedTimerId = window.setTimeout(() => {
      this.copied.set(false);
    }, 2000);
  }
}
