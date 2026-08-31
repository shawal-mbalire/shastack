use crate::domain::errors::ShaError;
use crate::domain::ports::ScaffoldPort;
use std::path::Path;
use std::process::Command;
use std::fs;

pub struct RealScaffolder;

impl ScaffoldPort for RealScaffolder {
    fn scaffold_angular(&self, dir: &Path) -> Result<(), ShaError> {
        let status = Command::new("npx")
            .arg("@angular/cli@18")
            .arg("new")
            .arg("frontend")
            .arg("--directory")
            .arg(dir.to_str().unwrap_or("."))
            .arg("--style=scss")
            .arg("--ssr=false")
            .arg("--ai-config=none")
            .arg("--skip-git=true")
            .arg("--defaults=true")
            .status()?;

        if !status.success() {
            return Err(ShaError::ScaffoldError("Failed to scaffold Angular project".to_string()));
        }
        Ok(())
    }

    fn scaffold_python(&self, dir: &Path, deps: &[&str]) -> Result<(), ShaError> {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let status = Command::new("uv")
            .arg("init")
            .current_dir(dir)
            .status()?;
        if !status.success() {
            return Err(ShaError::ScaffoldError("Failed to initialize Python project with uv".to_string()));
        }

        if !deps.is_empty() {
            let status = Command::new("uv")
                .arg("add")
                .args(deps)
                .current_dir(dir)
                .status()?;
            if !status.success() {
                return Err(ShaError::ScaffoldError("Failed to add dependencies with uv".to_string()));
            }
        }
        Ok(())
    }

    fn scaffold_flutter(&self, dir: &Path) -> Result<(), ShaError> {
        let status = Command::new("flutter")
            .arg("create")
            .arg("--project-name=app")
            .arg(".")
            .current_dir(dir)
            .status()?;
        if !status.success() {
            return Err(ShaError::ScaffoldError("Failed to scaffold Flutter project".to_string()));
        }
        Ok(())
    }

    fn scaffold_research(&self, dir: &Path) -> Result<(), ShaError> {
        let src_dir = dir.join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(
            src_dir.join("main.tex"),
            r#"\documentclass{article}
\begin{titlepage}
\title{Research Paper}
\author{shastack}
\end{titlepage}
\begin{document}
\maketitle
\section{Introduction}
Hello from shastack!
\end{document}"#,
        )?;
        self.write_justfile(
            dir,
            r#"set shell := ["bash", "-uc"]
build:
    pdflatex -output-directory=../artifacts src/main.tex
"#,
        )?;
        Ok(())
    }

    fn write_justfile(&self, dir: &Path, content: &str) -> Result<(), ShaError> {
        fs::write(dir.join("justfile"), content)?;
        Ok(())
    }

    fn write_ci_workflow(&self, dir: &Path, content: &str) -> Result<(), ShaError> {
        fs::write(dir.join("main.yml"), content)?;
        Ok(())
    }
}
