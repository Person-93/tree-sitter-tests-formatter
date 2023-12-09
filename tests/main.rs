use insta::assert_snapshot;
use std::path::Path;

use tree_sitter_tests_formatter::format_tests_dir;

fn write_fixture(fixture: &str, tmpdir: &Path) {
    use std::io::Write;
    let mut file = std::fs::File::create(tmpdir.join("test.txt")).unwrap();
    write!(file, "{}", fixture).unwrap();
}

fn test_fixture(fixture: &str) -> String {
    let tmpdir = tempfile::tempdir().unwrap().into_path();
    write_fixture(fixture, &tmpdir);
    format_tests_dir(&tmpdir);
    std::fs::read_to_string(tmpdir.join("test.txt")).unwrap()
}

#[test]
fn test_parse_test_file_1() {
    let fixture = r#"===
test 1
===

void

---

(source_file)"#;
    assert_snapshot!(test_fixture(fixture));
}

#[test]
fn test_parse_test_file_2() {
    let fixture = r#"===
test 1
===

void

---

(source_file
    (function_declaration)
)"#;
    assert_snapshot!(test_fixture(fixture));
}

#[test]
fn test_parse_test_file_3() {
    let fixture = r#"===
test 1
===

void MyFunc() {}

---

(source_file
  (function_declaration
    returnType: (type
      (builtin_type)
    )
    name: (symbol)
    arguments: (argument_declarations)
    body: (block)
  )
)"#;
    assert_snapshot!(test_fixture(fixture));
}

#[test]
fn test_parse_test_file_4() {
    let fixture = r#"================
hardcoded symbol
================

using __intrinsics__.Handle;

---

(source_file
	(hardcoded_symbol
	))

======
assert
======

assert(true, "This is an assertion error");
static_assert(true, "This is an assertion error");

---

(source_file
    (assertion
        (function_call_arguments
        	(bool_literal)
            (bool_literal)
          	(string_literal)))
    (assertion
        (function_call_arguments
        	(bool_literal)
            (bool_literal)
          	(string_literal))))"#;
    assert_snapshot!(test_fixture(fixture));
}
