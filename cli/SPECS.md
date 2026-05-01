shastack: The Unified Universal Stack Specificationshastack is a workspace-first meta-framework designed to unify Web, Mobile, Research, ML, and Embedded systems under a single command-line interface and a justfile orchestration layer.1. Core PhilosophyUnified Tooling: One command to rule them all (sha).Modular Initialization: Standalone features that can be toggled during workspace creation or added later.Flat Monorepo: One project per shastack workspace. Logic is segmented by domain-specific top-level folders.Just-Driven: All complex automation, including dependency management and parallel execution, lives in the justfile.Code-First API: APIs defined in Zod/Pydantic; clients generated automatically.Unified Secrets: Centralized environment management (.env.sha) for deployment and hardware keys.2. The CLI: sha (The shastack CLI)Workspace Lifecyclesha new <name>: Starts an interactive prompt to select features.sha add <feature>: Adds a new standalone module to the project.sha version [major|minor|patch]: Updates Semantic Versioning for the project.sha env [set|get] <key>: Manages project-wide environment variables in .env.sha.Execution & Deploymentsha run <feature>: Executes the development environment.sha build <feature>: Compiles artifacts (Binaries, PDFs, Web bundles, Model weights).sha test <feature>: Runs the test suite for the specified module.sha flash: Deploys firmware to hardware via avrdude, espidf, or mpy.sha deploy <feature> --target=[ftp|firebase|appwrite]: Triggers deployment pipelines.3. Directory Architecture (Modular & Flat)/shastack-project
├── .sha/ # CLI configuration and feature manifest
├── .github/workflows/ # Global CI/CD triggers
├── .env.sha # Project-wide environment variables (Git-ignored)
├── web/ # [Feature: Web Frontend/Backend]
│ ├── client/ # Angular (Default) or React
│ ├── server/ # Flask or Hono
│ ├── .github/ # Modular CI for Web
│ └── justfile # Local web automation
├── mobile/ # [Feature: Mobile App/Landing]
│ ├── app/ # Flutter App
│ ├── landing/ # Angular Landing Page
│ ├── .github/ # Modular CI for Mobile
│ └── justfile # Local mobile automation
├── research/ # [Feature: Research] (LaTeX)
│ ├── src/ # Modular LaTeX (chapters/sections)
│ ├── .github/ # Modular CI for Research (PDF build)
│ └── main.tex # Entry point
├── ml/ # [Feature: ML] (Python/Notebooks)
│ ├── notebooks/ # Experimentation Tier (01_eda, 02_training, etc.)
│ ├── src/ # Production Python package (Modular code)
│ ├── .github/ # Modular CI for ML (Model Validation)
│ ├── model_registry/ # Versioned production models
│ └── justfile # Local ML automation (uv based)
├── hardware/ # [Feature: Firmware] (C++/MicroPython)
│ ├── src/ # C++/Python source
│ ├── .github/ # Modular CI for Hardware (Compilation check)
│ └── VERSION # Semver file
├── shared/ # Hand-written types, constants, and generated clients
└── justfile # The MASTER task runner & Dependency Manager 4. Production StandardsA. Modular CI/CD (Per-Folder CI)Path-Based Triggers: Each module contains its own .github/workflows folder.Isolated Validation: A change in hardware/ should only trigger the hardware build and linting, not the web/ or ml/ pipelines.Global Gate: The root .github/workflows/main.yml acts as a coordinator for cross-domain integration tests.B. Observability & Logging (The "Pulse")Structured Logging: All modules must output logs in JSON format to stdout.Health Monitoring: Web backends must provide a GET /health endpoint. ML training must write a heartbeat.json.C. Validation & GatesModular Testing: sha test is recursive. It calls the justfile within the specific domain folder.Linting: Mandatory before any sha build.D. ML & Research RigorTiered Notebooks: All ML projects must start in ml/notebooks/ for EDA and experimentation before being refactored into the ml/src/ production package.Git-Pinned Weights: Every model in model_registry/ must be accompanied by a metadata.json containing the Git hash.PDF Artifacts: LaTeX builds must generate a version-stamped PDF.5. Deployment & SecretsTiered Environments: dev, staging, and prod targets.Secret Loading: The justfile loads .env.sha variables into the environment.6. Issue-Driven Development (IDD) StandardsIssue-First Rule: No code implementation is permitted without a corresponding Issue ID.Branching Strategy: Every task must be performed on a branch named issue-[ID]-[description].Traceability: Commit messages must reference the Issue ID (e.g., feat(web): add auth logic #12).Automated Scaffolding: The sha CLI should ideally support generating issues and branches from a roadmap file.7. The Orchestrator: justfileset shell := ["bash", "-uc"]

# --- Global Commands ---

# Install dependencies based on .sha/config.json

deps:
@echo "Installing project-wide dependencies..."
{{ if has_feature("web-client") }} cd web/client && npm install {{ endif }}
{{ if has_feature("web-server") }} cd web/server && uv sync {{ endif }}
{{ if has_feature("ml") }} cd ml && uv sync {{ endif }}

# Run tests per module

test module="all":
{{ if module == "all" || module == "web" }} cd web && just test {{ endif }}
{{ if module == "all" || module == "ml" }} cd ml && just test {{ endif }}

# --- Module Dev ---

dev-web:
just --parallel client server

# Open ML Notebooks

notebooks:
cd ml && uv run jupyter lab

# --- Global Sync ---

sync-all:
sha sync-api
just build
