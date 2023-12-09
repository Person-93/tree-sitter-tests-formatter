use std::{fs, path::PathBuf};

use clap::Parser;
use tree_sitter_tests_formatter::{format_test_file, format_tests_dir};

/// A tool for formatting tree-sitter test files.
#[derive(Debug, Parser, Clone)]
#[clap(version)]
pub struct Opts {
    /// Path to the directory/file to format.
    /// When a directory is given, all files in the directory will be formatted.
    #[clap(short, long, value_parser)]
    path: PathBuf,
}

fn main() {
    let opts = Opts::parse();
    if !opts.path.exists() {
        panic!("Path does not exist: {:?}", opts.path);
    }
    if opts.path.is_file() {
        println!("Formatting test file: {:?}", opts.path);
        format_test_file(&opts.path);
        return;
    }
    if opts.path.is_dir() {
        let n = fs::read_dir(&opts.path).unwrap().count();
        println!("Formatting {} tests in directory: {:?}", n, opts.path);
        format_tests_dir(&opts.path);
        return;
    }
    panic!("Path is not a file or directory: {:?}", opts.path)
}
