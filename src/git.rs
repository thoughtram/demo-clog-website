use std::process::Command;
use std::io::{Error, ErrorKind};


pub fn clone (repo_url: &str, into_directory: &str) -> Result<String, Error> {
    let output = try!(Command::new("git")
                    .arg("clone")
                    .arg(repo_url)
                    .arg(into_directory)
                    .output());

    match output.status.success() {
        true => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
        false => Err(Error::new(ErrorKind::Other, format!("{}", String::from_utf8_lossy(&output.stderr))))
    }
}
