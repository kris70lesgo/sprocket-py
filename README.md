# sprocket-py

**Proof-of-Concept:** Python bindings for [Sprocket](https://sprocket.bio/)'s WDL parser via [PyO3](https://pyo3.rs/).

## What this PoC does

Validates that Sprocket's core WDL parsing types can be exposed to Python using PyO3.

- `parse_wdl(source: str)` → returns an opaque `WdlDocument` backed by the real Rust AST
- Invalid WDL → raises a `ValueError` with a human-readable error message
- `WdlDocument` has a `__repr__` showing the workflow/task name

```python
>>> import sprocket_py
>>> doc = sprocket_py.parse_wdl("version 1.1\nworkflow foo {}")
>>> doc
<WdlDocument workflow=foo>
>>> sprocket_py.parse_wdl("this is garbage")
ValueError: a WDL document must start with a version statement
```

## What this PoC intentionally does NOT do

- ❌ Linting
- ❌ Formatting
- ❌ AST traversal APIs
- ❌ Stable Python API guarantees
- ❌ Performance tuning
- ❌ CLI

These are out of scope. This PoC exists solely to de-risk the core technical unknown: can Sprocket's Rust internals be cleanly wired to Python?

## Architecture

```
Python
  │
  │  (PyO3)
  ▼
Rust (sprocket-py)
  │
  │  (crates.io dependency)
  ▼
wdl-ast 0.21 (from Sprocket)
```

Key design choices:
- **Rust owns all data** — Python holds an opaque `WdlDocument` handle.
- **No invented parsing logic** — calls `wdl_ast::Document::parse()`, the same code path Rust users use.
- **`unsendable` pyclass** — rowan's `SyntaxNode` is `!Send`; the pyclass is pinned to its creation thread.

## How to run locally

```bash
# Prerequisites: Rust toolchain, Python 3.8+
git clone <this-repo>
cd sprocket-py

# Create a virtualenv and install
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop

# Test it
python3 -c "
import sprocket_py
doc = sprocket_py.parse_wdl('version 1.1\nworkflow foo {}')
print(doc)
"
```

## Project structure

```
sprocket-py/
├── Cargo.toml        # Rust crate: PyO3 + wdl-ast dependencies
├── pyproject.toml    # Python build config (maturin)
├── README.md
└── src/
    └── lib.rs        # All Rust code (~60 lines)
```
