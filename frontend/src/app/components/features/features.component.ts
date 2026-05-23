import { ChangeDetectionStrategy, Component } from '@angular/core';
import {
  BrainCircuit,
  Cpu,
  FlaskConical,
  GitBranch,
  Globe,
  LucideAngularModule,
  Smartphone
} from 'lucide-angular';

@Component({
  selector: 'app-features',
  standalone: true,
  imports: [LucideAngularModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <section id="features" class="section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">One CLI for every build surface</h2>
          <p class="section-copy">
            shastack standardizes the workflows teams actually ship: frontend, mobile, research,
            machine learning, hardware, and issue-driven delivery.
          </p>
        </div>

        <div class="features-grid">
          @for (feature of features; track feature.title) {
            <article class="feature-card">
              <div class="feature-icon">
                <lucide-icon [img]="feature.icon" [size]="22"></lucide-icon>
              </div>
              <h3 class="feature-title">{{ feature.title }}</h3>
              <p class="feature-description">{{ feature.description }}</p>
            </article>
          }
        </div>
      </div>
    </section>
  `
})
export class FeaturesComponent {
  readonly features = [
    {
      icon: Globe,
      title: 'Web',
      description:
        'Angular client + Hono (Bun) or Flask (uv) backends, RBAC, and Zod/Pydantic validation.'
    },
    {
      icon: Smartphone,
      title: 'Mobile',
      description: 'Flutter with Drift (offline-first), Riverpod, and biometric auth patterns.'
    },
    {
      icon: FlaskConical,
      title: 'Research',
      description: 'LaTeX workspace with modular chapters, semantic justfiles, and CI PDF generation.'
    },
    {
      icon: BrainCircuit,
      title: 'ML',
      description: 'uv-managed Python environments, Polars + scikit-learn, and reproducible pipelines.'
    },
    {
      icon: Cpu,
      title: 'Hardware',
      description: 'ESP32 firmware (ESP-IDF, Arduino, MicroPython, Rust) with watchdog and OTA patterns.'
    },
    {
      icon: GitBranch,
      title: 'Issue-Driven',
      description: 'IDD workflow: automated branch creation and PR management via gh CLI integration.'
    }
  ] as const;
}
