import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  selector: 'app-prerequisites',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <section id="prerequisites" class="section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">Prerequisites</h2>
          <p class="section-copy">
            shastack leverages professional, industry-standard tools. Ensure these are installed on your system or run <code>sha deps</code> to check.
          </p>
        </div>

        <div class="prerequisites-grid">
          @for (tool of tools; track tool.name) {
            <a [href]="tool.url" target="_blank" rel="noopener noreferrer" class="tool-card">
              <span class="tool-name">{{ tool.name }}</span>
              <span class="tool-desc">{{ tool.description }}</span>
            </a>
          }
        </div>
      </div>
    </section>
  `
})
export class PrerequisitesComponent {
  readonly tools = [
    { name: 'just', description: 'Command runner', url: 'https://just.systems/man/en/chapter_4.html' },
    { name: 'git', description: 'Version control', url: 'https://git-scm.com/downloads' },
    { name: 'gh', description: 'GitHub CLI', url: 'https://cli.github.com/' },
    { name: 'bun', description: 'Fast JS runtime', url: 'https://bun.sh/' },
    { name: 'uv', description: 'Fast Python manager', url: 'https://github.com/astral-sh/uv' },
    { name: 'angular', description: 'Web framework', url: 'https://angular.dev/' },
    { name: 'flutter', description: 'Mobile framework', url: 'https://docs.flutter.dev/get-started/install' },
    { name: 'latex', description: 'Typesetting system', url: 'https://www.latex-project.org/get/' },
    { name: 'pio', description: 'Hardware core', url: 'https://docs.platformio.org/en/latest/core/installation.html' }
  ] as const;
}
