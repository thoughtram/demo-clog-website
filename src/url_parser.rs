use regex::Regex;

static GITHUB_PREFIX: &'static str = "https://github.com/";
static HTTPS_PREFIX: &'static str = "https://";

pub fn parse(url: &str) -> String {

    let user_repo_regex = Regex::new(r"^[\w\d.-]*/[\w\d.-]*$").unwrap();

    // in case user provided user/repo string
    if user_repo_regex.is_match(url) {
        return format!("{}{}", GITHUB_PREFIX, url);
    }

    let github_user_repo_regex = Regex::new(r"^github.com/[\w\d.-]*/[\w\d.-]*$").unwrap();

    // in case user provided github.com/user/repo string
    if github_user_repo_regex.is_match(url) {
        return format!("{}{}", HTTPS_PREFIX, url);
    }

    // anything else should just used as is and either works or fails later
    url.to_owned()
}
