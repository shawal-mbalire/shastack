// Workspace module - now delegates to domain/use_cases.rs
// This module is kept for backward compatibility.
// All business logic lives in domain/use_cases.rs

pub use crate::domain::use_cases::WorkspaceUseCases;
pub use crate::domain::models::*;
pub use crate::domain::errors::ShaError;
