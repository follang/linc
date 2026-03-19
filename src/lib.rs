pub mod diagnostics;
pub mod ir;

pub use diagnostics::{Diagnostic, DiagnosticKind, Severity};
pub use ir::{BindingPackage, BindingItem, BindingType};
