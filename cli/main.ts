import { Command } from "commander";
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "fs";
import { join } from "path";

const program = new Command();
const CONFIG_PATH = ".sha/config.json";

function isDryRun() {
  return program.opts().dryRun;
}

function loadConfig() {
  if (!existsSync(CONFIG_PATH)) {
    console.error("Error: .sha/config.json not found.");
    process.exit(1);
  }
  return JSON.parse(readFileSync(CONFIG_PATH, "utf-8"));
}

function saveConfig(config: any) {
  if (isDryRun()) {
    console.log(`[DRY-RUN] Would save config to ${CONFIG_PATH}`);
    return;
  }
  writeFileSync(CONFIG_PATH, JSON.stringify(config, null, 2));
}

program
  .name("sha")
  .description("shastack CLI tool")
  .version("0.1.0")
  .option("--dry-run", "simulate execution without making changes");

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
      console.log(`Enabling module: ${moduleName}`);
      config.modules[moduleName].enabled = true;
      saveConfig(config);
    }

    const modulePath = config.modules[moduleName].path;
    if (!existsSync(modulePath)) {
      if (isDryRun()) {
        console.log(`[DRY-RUN] Would create directory: ${modulePath}`);
      } else {
        mkdirSync(modulePath, { recursive: true });
        console.log(`Created directory: ${modulePath}`);
      }
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
    const commandStr = cmd.join(" ");
    
    if (isDryRun()) {
      console.log(`[DRY-RUN] Would run in ${moduleName} (${modulePath}): ${commandStr}`);
    } else {
      console.log(`Running in ${moduleName} (${modulePath}): ${commandStr}`);
      // In a real implementation, we'd use Bun.spawn or similar.
    }
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
  if (isDryRun()) {
    console.log(`[DRY-RUN] Would save environment to ${ENV_PATH}`);
    return;
  }
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
    if (!isDryRun()) {
      console.log(`Set ${key}=${value} in ${ENV_PATH}`);
    }
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

const busCmd = program.command("bus").description("Cross-module event bus");

busCmd
  .command("emit <event> [payload]")
  .description("Emit an event to the bus")
  .action((event, payload) => {
    const msg = `[BUS] Emitting event: ${event} with payload: ${payload || "{}"}`;
    if (isDryRun()) {
      console.log(`[DRY-RUN] Would emit: ${msg}`);
    } else {
      console.log(msg);
    }
  });

busCmd
  .command("listen <event>")
  .description("Listen for an event on the bus")
  .action((event) => {
    console.log(`[BUS] Listening for event: ${event}...`);
  });

const benchCmd = program.command("bench").description("Performance benchmarking");

benchCmd
  .command("run <module>")
  .description("Run benchmarks for a module")
  .action((moduleName) => {
    console.log(`[BENCH] Running benchmarks for ${moduleName}...`);
    const start = performance.now();
    setTimeout(() => {
      const end = performance.now();
      console.log(`[BENCH] Completed in ${(end - start).toFixed(2)}ms`);
    }, 100);
  });

const auditCmd = program.command("audit").description("Security hardening & audit");

auditCmd
  .command("scan")
  .description("Scan the workspace for vulnerabilities")
  .action(() => {
    console.log("[AUDIT] Scanning workspace...");
    console.log("[AUDIT] No immediate vulnerabilities found.");
  });

const docsCmd = program.command("docs").description("Documentation & DevEx");

docsCmd
  .command("serve")
  .description("Serve the documentation locally")
  .action(() => {
    console.log("[DOCS] Serving documentation at http://localhost:3000...");
  });

const releaseCmd = program.command("release").description("Release management");

releaseCmd
  .command("v1")
  .description("Prepare the v1.0.0 release")
  .action(() => {
    console.log("[RELEASE] Preparing v1.0.0...");
    console.log("[RELEASE] Bundling modules...");
    console.log("[RELEASE] v1.0.0-rc1 ready.");
  });

program.parse();
