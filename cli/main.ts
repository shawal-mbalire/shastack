import { Command } from "commander";
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "fs";
import { join } from "path";

const program = new Command();
const CONFIG_PATH = ".sha/config.json";

function loadConfig() {
  if (!existsSync(CONFIG_PATH)) {
    console.error("Error: .sha/config.json not found.");
    process.exit(1);
  }
  return JSON.parse(readFileSync(CONFIG_PATH, "utf-8"));
}

function saveConfig(config: any) {
  writeFileSync(CONFIG_PATH, JSON.stringify(config, null, 2));
}

program
  .name("sha")
  .description("shastack CLI tool")
  .version("0.1.0");

program
  .command("new <module>")
  .description("Initialize a new module")
  .action((moduleName) => {
    const config = loadConfig();
    if (!config.modules[moduleName]) {
      console.error(`Error: Module '${moduleName}' is not defined in config.`);
      process.exit(1);
    }

    if (config.modules[moduleName].enabled) {
      console.log(`Module '${moduleName}' is already enabled.`);
    } else {
      config.modules[moduleName].enabled = true;
      saveConfig(config);
      console.log(`Enabled module: ${moduleName}`);
    }

    const modulePath = config.modules[moduleName].path;
    if (!existsSync(modulePath)) {
      mkdirSync(modulePath, { recursive: true });
      console.log(`Created directory: ${modulePath}`);
    }
  });

program
  .command("run <module> [cmd...]")
  .description("Run a command in a module")
  .action((moduleName, cmd) => {
    const config = loadConfig();
    if (!config.modules[moduleName]) {
      console.error(`Error: Module '${moduleName}' is not defined.`);
      process.exit(1);
    }

    if (!config.modules[moduleName].enabled) {
      console.error(`Error: Module '${moduleName}' is not enabled. Run 'sha new ${moduleName}' first.`);
      process.exit(1);
    }

    const modulePath = config.modules[moduleName].path;
    console.log(`Running in ${moduleName} (${modulePath}): ${cmd.join(" ")}`);
    
    // In a real implementation, we'd use Bun.spawn or similar.
    // For now, let's just log the intent as this is a scaffold.
  });

program.parse();
