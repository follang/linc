use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKind {
    PreprocessingFailed,
    ParseFailed,
    ProbeUnavailable,
    ProbeFailed,
    DeclarationUnsupported,
    DeclarationPartial,
    SymbolMissing,
    SymbolAmbiguous,
    SymbolMatched,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub item_name: Option<String>,
    pub artifact_path: Option<String>,
}

impl Diagnostic {
    pub fn error(kind: DiagnosticKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            severity: Severity::Error,
            message: message.into(),
            location: None,
            item_name: None,
            artifact_path: None,
        }
    }

    pub fn warning(kind: DiagnosticKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            severity: Severity::Warning,
            message: message.into(),
            location: None,
            item_name: None,
            artifact_path: None,
        }
    }

    pub fn with_location(mut self, file: Option<String>, offset: usize) -> Self {
        self.location = Some(SourceLocation { file, offset });
        self
    }

    pub fn with_item(mut self, name: impl Into<String>) -> Self {
        self.item_name = Some(name.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_error_construction() {
        let d = Diagnostic::error(DiagnosticKind::ParseFailed, "unexpected token");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.kind, DiagnosticKind::ParseFailed);
        assert_eq!(d.message, "unexpected token");
        assert!(d.location.is_none());
        assert!(d.item_name.is_none());
    }

    #[test]
    fn diagnostic_warning_with_location() {
        let d = Diagnostic::warning(DiagnosticKind::DeclarationPartial, "bitfield ignored")
            .with_location(Some("test.h".into()), 42)
            .with_item("my_struct");
        assert_eq!(d.severity, Severity::Warning);
        assert_eq!(d.location.as_ref().unwrap().offset, 42);
        assert_eq!(d.item_name.as_deref(), Some("my_struct"));
    }

    #[test]
    fn diagnostic_serialization_roundtrip() {
        let d = Diagnostic::error(DiagnosticKind::SymbolMissing, "symbol not found")
            .with_item("foo_func");
        let json = serde_json::to_string(&d).unwrap();
        let d2: Diagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(d, d2);
    }
}
