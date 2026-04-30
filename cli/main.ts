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

const ENV_PATH = ".env.sha";

function loadEnv() {
  if (!existsSync(ENV_PATH)) return {};
  const content = readFileSync(ENV_PATH, "utf-8");
  return content.split("\n").reduce((acc: any, line) => {
    const [key, ...value] = line.split("=");
    if (key) acc[key.trim()] = value.join("=").trim();
    return acc;
  }, {});
}

function saveEnv(env: any) {
  const content = Object.entries(env)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");
  writeFileSync(ENV_PATH, content + "\n");
}

const envCmd = program.command("env").description("Manage environment secrets");

envCmd
  .command("set <key> <value>")
  .description("Set an environment variable")
  .action((key, value) => {
    const env = loadEnv();
    env[key] = value;
    saveEnv(env);
    console.log(`Set ${key}=${value} in ${ENV_PATH}`);
  });

envCmd
  .command("get <key>")
  .description("Get an environment variable")
  .action((key) => {
    const env = loadEnv();
    if (env[key]) {
      console.log(env[key]);
    } else {
      console.error(`Error: ${key} not found in ${ENV_PATH}`);
    }
  });

envCmd
  .command("list")
  .description("List all environment variables")
  .action(() => {
    const env = loadEnv();
    Object.entries(env).forEach(([key, value]) => {
      console.log(`${key}=${value}`);
    });
  });

program.parse();
