<div align="center">
  <h1><code>Tree-Sitter Tests Formatter</code></h1>
  <p>
    <strong>A formatter for Tree-Sitter test files</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://crates.io/crates/tree_sitter_tests_formatter">
      <img alt="Crates.io" src="https://img.shields.io/crates/d/tree-sitter-tests-formatter">
    </a>
    <a href="https://crates.io/crates/tree_sitter_tests_formatter">
      <img alt="Crates.io" src="https://img.shields.io/crates/v/tree-sitter-tests-formatter">
    </a>
    <a href="https://github.com/Sarrus1/tree-sitter-tests-formatter/actions/workflows/release.yml">
      <img
        alt="Github release status"
        src="https://github.com/Sarrus1/tree-sitter-tests-formatter/actions/workflows/release.yml/badge.svg"
      />
    </a>
    <a href="https://codecov.io/gh/Sarrus1/tree-sitter-tests-formatter" > 
      <img
        alt="Code coverage"
        src="https://codecov.io/gh/Sarrus1/tree-sitter-tests-formatter/branch/main/graph/badge.svg?token=5T6QQZYPQ6"/> 
    </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/Sarrus1/tree-sitter-tests-formatter">
  </p>
</div>

# Examples

## CLI

```
tsfmt --path /Users/charles/Developer/tree-sitter-sourcepawn/test/corpus
```

## Code usage

```rust
use tree_sitter_tests_formatter::format_files;

fn main() {
    let path = PathBuf::from("tests/corpus");
    format_tests_dir(&path);
}
```
