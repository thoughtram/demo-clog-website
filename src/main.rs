#[macro_use]
extern crate nickel;
extern crate clog;
extern crate rustc_serialize;
extern crate uuid;
extern crate regex;

use nickel::{ Nickel, JsonBody, HttpRouter, StaticFilesHandler };
use clog_config::ClogConfig;
use clog_result::ClogResult;
use rustc_serialize::json;
use uuid::Uuid;
use std::error::Error;
use std::env;

mod git;
mod clog_interop;
mod clog_config;
mod clog_result;
mod url_parser;

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets"));
    server.utilize(StaticFilesHandler::new("assets/templates"));

    server.post("/generate", middleware! { |request, response|

        let clog_config = request.json_as::<ClogConfig>().unwrap();

        let repo_name = Uuid::new_v4().to_string();
        let repo_url = url_parser::parse(&clog_config.repository);

        let result = if let Err(err) = git::clone(&repo_url, &repo_name) {
            ClogResult {
                changelog: "".to_owned(),
                error: err.description().to_owned(),
            }
        } else {
            let changelog = clog_interop::generate_changelog(&repo_name, &repo_url);

            ClogResult {
                changelog: changelog,
                error: "".to_owned()
            }
        };

        json::encode(&result).unwrap()
    });

    // Look up our server port number in PORT, for compatibility with Heroku.
    fn get_server_port() -> u16 {
        env::var("PORT").unwrap_or("6767".to_string()).parse().unwrap()
    }

    server.listen(("0.0.0.0", get_server_port()));
}
