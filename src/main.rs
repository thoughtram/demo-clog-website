#[macro_use]
extern crate nickel;
extern crate clog;
extern crate rustc_serialize;
extern crate uuid;

use nickel::{ Nickel, JsonBody, HttpRouter, StaticFilesHandler };
use clog_config::ClogConfig;
use clog_result::ClogResult;
use rustc_serialize::json;
use uuid::Uuid;
use std::error::Error;


mod git;
mod clog_interop;
mod clog_config;
mod clog_result;

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets"));
    server.utilize(StaticFilesHandler::new("assets/templates"));

    server.post("/generate", middleware! { |request, response|

        let clog_config = request.json_as::<ClogConfig>().unwrap();

        let repo_name = Uuid::new_v4().to_string();

        let result = if let Err(err) = git::clone(&clog_config.repository, &repo_name) {
            ClogResult {
                changelog: "".to_owned(),
                error: err.description().to_owned(),
            }
        } else {
            let changelog = clog_interop::generate_changelog(&repo_name, &clog_config.repository);

            ClogResult {
                changelog: changelog,
                error: "".to_owned()
            }
        };

        json::encode(&result).unwrap()
    });

    server.listen("127.0.0.1:6767");
}
