use std::{cmp::max, fs::read_to_string, fs::File, path::Path, path::PathBuf, vec};

use lazy_static::lazy_static;
use regex::Regex;

mod s_exp_formatter;

pub use s_exp_formatter::format_s_expr;

/// Format tree-sitter test files in the given directory.
///
/// # Arguments
/// `path` - The directory to search for tree-sitter test files.
pub fn format_tests_dir(path: &Path) {
    assert!(path.exists(), "Path does not exist: {:?}", path);
    assert!(path.is_dir(), "Path is not a directory: {:?}", path);
    let test_files = walk_tests_dir(path);
    for test_file in test_files.iter() {
        test_file.format_file(test_file.path());
    }
}

/// Format a tree-sitter test file.
///
/// # Arguments
/// `path` - The path to the tree-sitter test file.
pub fn format_test_file(path: &Path) {
    assert!(path.exists(), "Path does not exist: {:?}", path);
    assert!(path.is_file(), "Path is not a file: {:?}", path);
    let test_file = TestFile::from_file(path);
    test_file.format_file(path);
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct TestFile {
    path: PathBuf,
    fixtures: Vec<Fixture>,
}

impl TestFile {
    pub fn from_file(path: &Path) -> Self {
        enum State {
            InFixtureName,
            InFixture,
            InExpected,
            None,
        }
        lazy_static! {
            static ref RE_FIXTURE_NAME_SEP: Regex = Regex::new(r"^====*$").unwrap();
            static ref RE_FIXTURE_SEP: Regex = Regex::new(r"^----*$").unwrap();
        }
        let mut state: State = State::None;
        let mut fixtures = Vec::new();
        let mut cur = Fixture::default();
        for line in read_to_string(path).unwrap().lines() {
            match state {
                State::None => {
                    // Looking for fixture name
                    if RE_FIXTURE_NAME_SEP.is_match(line) {
                        state = State::InFixtureName;
                    }
                }
                State::InFixtureName => {
                    // Looking for fixture
                    if RE_FIXTURE_NAME_SEP.is_match(line) {
                        state = State::InFixture;
                    }
                    // Fixture name line
                    else {
                        cur.name = line.to_string();
                    }
                }
                State::InFixture => {
                    // Looking for expected
                    if RE_FIXTURE_SEP.is_match(line) {
                        state = State::InExpected;
                    }
                    // Fixture line
                    else {
                        cur.input.push_str(line);
                        cur.input.push('\n');
                    }
                }
                State::InExpected => {
                    // Looking for next fixture
                    if RE_FIXTURE_NAME_SEP.is_match(line) {
                        state = State::InFixtureName;
                        fixtures.push(cur.clone());
                        cur = Fixture::default();
                    } else {
                        cur.expected.push_str(line);
                        cur.expected.push('\n');
                    }
                }
            }
        }
        fixtures.push(cur);

        Self {
            path: path.to_path_buf(),
            fixtures,
        }
    }

    pub fn format_file(&self, path: &Path) {
        use std::io::Write;
        let formatted = self.format();
        let mut file = File::create(path).unwrap();
        write!(file, "{}", formatted).unwrap();
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn format(&self) -> String {
        let mut res = Vec::new();
        for fixture in self.fixtures.iter() {
            res.push(fixture.format());
        }

        res.join("\n".repeat(2).as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Fixture {
    name: String,
    input: String,
    expected: String,
}

impl Fixture {
    pub fn format(&self) -> String {
        let res = vec![
            self.format_name(),
            "\n\n".to_string(),
            self.format_input(),
            "\n\n".to_string(),
            "---".to_string(),
            "\n\n".to_string(),
            self.format_expected(),
        ];

        res.join("")
    }

    fn format_name(&self) -> String {
        let mut res = Vec::new();
        let n = max(3, self.name.trim().len());
        let sep = "=".to_string().repeat(n);
        res.push(sep.clone());
        res.push(self.name.trim().to_string());
        res.push(sep.clone());

        res.join("\n")
    }

    fn format_input(&self) -> String {
        self.input.trim().to_string()
    }

    fn format_expected(&self) -> String {
        s_exp_formatter::format_s_expr(&self.expected)
    }
}

fn walk_tests_dir(path: &Path) -> Vec<TestFile> {
    let mut res: Vec<TestFile> = Vec::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            res.push(TestFile::from_file(&path));
        }
    }

    res
}
