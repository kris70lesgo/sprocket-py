use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use wdl_ast::{Ast, AstToken, Document, Severity};
use wdl_ast::v1::DocumentItem;

/// An opaque handle to a parsed WDL document.
///
/// Rust owns all real data; Python only holds this handle.
/// Marked `unsendable` because rowan's `SyntaxNode` is !Send/!Sync.
#[pyclass(unsendable)]
struct WdlDocument {
    document: Document,
}

#[pymethods]
impl WdlDocument {
    fn __repr__(&self) -> String {
        match self.document.ast() {
            Ast::V1(ast) => {
                // Try to find a workflow name first
                for item in ast.items() {
                    if let DocumentItem::Workflow(wf) = item {
                        return format!("<WdlDocument workflow={}>", wf.name().text());
                    }
                }
                // Fall back to task name
                for item in ast.items() {
                    if let DocumentItem::Task(task) = item {
                        return format!("<WdlDocument task={}>", task.name().text());
                    }
                }
                "<WdlDocument parsed successfully>".to_string()
            }
            Ast::Unsupported => "<WdlDocument unsupported version>".to_string(),
        }
    }
}

/// Parse a WDL source string into a `WdlDocument`.
///
/// Returns a `WdlDocument` on success.
/// Raises `ValueError` with a human-readable message on parse errors.
#[pyfunction]
fn parse_wdl(source: &str) -> PyResult<WdlDocument> {
    let (document, diagnostics) = Document::parse(source);

    // Collect error-severity diagnostics
    let errors: Vec<String> = diagnostics
        .iter()
        .filter(|d| d.severity() == Severity::Error)
        .map(|d| d.message().to_string())
        .collect();

    if !errors.is_empty() {
        return Err(PyValueError::new_err(errors.join("\n")));
    }

    Ok(WdlDocument { document })
}

/// The `sprocket_py` Python module, implemented in Rust.
#[pymodule]
fn sprocket_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_wdl, m)?)?;
    m.add_class::<WdlDocument>()?;
    Ok(())
}
