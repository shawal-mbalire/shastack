import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { Github, LucideAngularModule, Moon, Sun } from 'lucide-angular';

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [LucideAngularModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <header class="site-header">
      <div class="container header-inner">
        <a class="brand" href="#top" aria-label="shastack home">
          <span class="brand-mark">sha</span>
          <span>shastack</span>
        </a>

        <nav class="header-links" aria-label="Primary">
          <a class="nav-link" href="#features">Features</a>
          <a class="nav-link" href="#commands">Commands</a>
          <a
            class="nav-link"
            href="https://github.com/shawal-mbalire/shastack/tree/main/docs"
            target="_blank"
            rel="noreferrer"
          >
            Docs
          </a>
        </nav>

        <div class="header-actions">
          <a
            class="icon-button"
            href="https://github.com/shawal-mbalire/shastack"
            target="_blank"
            rel="noreferrer"
            aria-label="View shastack on GitHub"
          >
            <lucide-icon [img]="Github" [size]="18"></lucide-icon>
          </a>

          <button
            type="button"
            class="icon-button"
            (click)="toggleTheme.emit()"
            [attr.aria-label]="theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'"
          >
            <lucide-icon [img]="theme === 'dark' ? Sun : Moon" [size]="18"></lucide-icon>
          </button>
        </div>
      </div>
    </header>
  `
})
export class HeaderComponent {
  @Input({ required: true }) theme!: 'light' | 'dark';
  @Output() readonly toggleTheme = new EventEmitter<void>();

  readonly Github = Github;
  readonly Moon = Moon;
  readonly Sun = Sun;
}
