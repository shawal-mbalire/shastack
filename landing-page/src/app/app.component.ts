import { Component, effect, signal } from '@angular/core';

import { CommandsComponent } from './components/commands/commands.component';
import { FeaturesComponent } from './components/features/features.component';
import { FooterComponent } from './components/footer/footer.component';
import { HeaderComponent } from './components/header/header.component';
import { HeroComponent } from './components/hero/hero.component';

type Theme = 'light' | 'dark';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HeaderComponent, HeroComponent, FeaturesComponent, CommandsComponent, FooterComponent],
  template: `
    <div class="app-shell">
      <app-header [theme]="theme()" (toggleTheme)="toggleTheme()"></app-header>
      <main>
        <app-hero></app-hero>
        <app-features></app-features>
        <app-commands></app-commands>
      </main>
      <app-footer></app-footer>
    </div>
  `
})
export class AppComponent {
  readonly theme = signal<Theme>(this.resolveInitialTheme());

  constructor() {
    effect(() => {
      const root = document.documentElement;
      const currentTheme = this.theme();

      root.classList.remove('light', 'dark');
      root.classList.add(currentTheme);
      root.style.colorScheme = currentTheme;
      localStorage.setItem('theme', currentTheme);
    });
  }

  toggleTheme(): void {
    this.theme.update((currentTheme) => (currentTheme === 'dark' ? 'light' : 'dark'));
  }

  private resolveInitialTheme(): Theme {
    const storedTheme = localStorage.getItem('theme');

    if (storedTheme === 'light' || storedTheme === 'dark') {
      return storedTheme;
    }

    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
}
