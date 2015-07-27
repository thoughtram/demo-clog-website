#[derive(RustcDecodable, RustcEncodable)]
pub struct ClogResult {
    pub changelog: String,
    pub error: String
}
