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
      description: 'Scaffold a new universal workspace'
    },
    {
      name: 'sha add <feature>',
      description: 'Add a module or current dir (sha add .)'
    },
    {
      name: 'sha restore',
      description: 'Repair missing files in enabled features'
    },
    {
      name: 'sha upgrade',
      description: 'Self-update the sha CLI to latest'
    },
    {
      name: 'sha pulse',
      description: 'Check health status across all modules'
    },
    {
      name: 'sha version auto',
      description: 'Automated SemVer via Conventional Commits'
    },
    {
      name: 'sha issue start <id>',
      description: 'Fetch issue & start IDD branch via gh'
    },
    {
      name: 'sha sync-api',
      description: 'Coordinate types across the stack'
    },
    {
      name: 'just <command>',
      description: 'Task execution (build, test, run, flash)'
    }
  ] as const;
}
