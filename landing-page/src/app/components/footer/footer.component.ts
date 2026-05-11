import { ChangeDetectionStrategy, Component } from '@angular/core';
import { Github, LucideAngularModule } from 'lucide-angular';

@Component({
  selector: 'app-footer',
  standalone: true,
  imports: [LucideAngularModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <footer class="site-footer">
      <div class="container footer-inner">
        <div class="footer-meta">
          <span>shastack — MIT License</span>
          <span class="footer-note">Built with shastack</span>
        </div>

        <div class="footer-links">
          <a
            class="footer-link"
            href="https://github.com/shawal-mbalire/shastack"
            target="_blank"
            rel="noreferrer"
          >
            <lucide-icon [img]="Github" [size]="18"></lucide-icon>
            <span>GitHub</span>
          </a>
        </div>
      </div>
    </footer>
  `
})
export class FooterComponent {
  readonly Github = Github;
}
