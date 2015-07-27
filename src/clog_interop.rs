use std::fs::{self, File};
use std::path::Path;
use std::io::{Read};
use clog::Clog;

pub fn generate_changelog (repository: &str, repo_url: &str) -> String {
    let mut clog = Clog::with_dir(repository).unwrap_or_else(|e| {
        fs::remove_dir_all(repository).ok();
        panic!("Failed to clone repository: {}", e);
    });
    let changelog_file_name = format!("changelog_{}.md", repository);
    clog.repository(repo_url);
    clog.write_changelog_to(Path::new(repository).join(&changelog_file_name));

    let mut contents = String::new();

    File::open(&Path::new(repository).join(&changelog_file_name))
        .map(|mut f| f.read_to_string(&mut contents).ok()).ok();

    fs::remove_dir_all(repository).ok();

    contents
}
