#[macro_use]
extern crate nickel;
extern crate clog;

use nickel::Nickel;
use nickel::router::http_router::HttpRouter;

mod git;
mod clog_interopt;

fn main() {
    let mut server = Nickel::new();

    let repo_name = "some-unique-id";
    let repo_uri = "https://github.com/angular/angular";

    git::clone(repo_uri, repo_name).ok();

    let changelog = clog_interopt::generate_changelog(repo_name, repo_uri);

    server.get("**", middleware!(&changelog as &str));
    server.listen("127.0.0.1:6767");
}
