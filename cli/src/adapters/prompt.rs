use crate::domain::errors::ShaError;
use crate::domain::ports::PromptPort;
use inquire::{MultiSelect, Confirm};

pub struct RealPrompt;

impl PromptPort for RealPrompt {
    fn multi_select(&self, message: &str, options: &[&str]) -> Result<Vec<String>, ShaError> {
        let options_vec: Vec<&str> = options.to_vec();
        let selected = MultiSelect::new(message, options_vec)
            .with_vim_mode(true)
            .with_help_message("↑↓/jk to move, space to select, enter to confirm")
            .prompt()
            .map_err(|e| ShaError::IoError(e.to_string()))?;
        Ok(selected.into_iter().map(String::from).collect())
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool, ShaError> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|e| ShaError::IoError(e.to_string()))
    }

    fn input(&self, message: &str) -> Result<Option<String>, ShaError> {
        let value = inquire::Text::new(message)
            .prompt()
            .map_err(|e| ShaError::IoError(e.to_string()))?;
        Ok(Some(value))
    }
}
