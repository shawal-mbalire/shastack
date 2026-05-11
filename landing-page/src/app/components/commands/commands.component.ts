import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  selector: 'app-commands',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  styles: [':host { display: block; }'],
  template: `
    <section id="commands" class="section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">Command surface</h2>
          <p class="section-copy">
            A concise CLI for scaffolding, running, testing, deployment, and issue-driven delivery.
          </p>
        </div>

        <div class="commands-shell">
          <div class="table-wrap">
            <table class="command-table">
              <thead>
                <tr>
                  <th scope="col">Command</th>
                  <th scope="col">Description</th>
                </tr>
              </thead>
              <tbody>
                @for (command of commands; track command.name) {
                  <tr>
                    <td class="command-cell"><code>{{ command.name }}</code></td>
                    <td>{{ command.description }}</td>
                  </tr>
                }
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </section>
  `
})
export class CommandsComponent {
  readonly commands = [
    {
      name: 'sha new <name>',
      description: 'Scaffold a new workspace'
    },
    {
      name: 'sha add <feature>',
      description: 'Add a module to existing workspace'
    },
    {
      name: 'sha run <feature>',
      description: 'Run the dev server'
    },
    {
      name: 'sha build <feature>',
      description: 'Compile artifacts'
    },
    {
      name: 'sha test <feature>',
      description: 'Run tests'
    },
    {
      name: 'sha deploy <feature> --target',
      description: 'Deploy to target'
    },
    {
      name: 'sha pulse',
      description: 'Health check all modules'
    },
    {
      name: 'sha issue start <id> <desc>',
      description: 'Start an IDD branch'
    },
    {
      name: 'sha registry pin <model>',
      description: 'Pin a model to git hash'
    }
  ] as const;
}
