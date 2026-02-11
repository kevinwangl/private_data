pub mod validator;
#[cfg(test)]
mod tests;

pub use validator::{DataValidator, ValidationResult, ValidationError, ValidationWarning, Severity};
