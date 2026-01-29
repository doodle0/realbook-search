use std::path::Path;
use rocket::{fs::NamedFile};

#[get("/")]
pub fn index() -> &'static str {
    "This is the API root address."
}

#[get("/rickroll")]
pub async fn rickroll() -> Option<NamedFile> {
    println!("{:?}", Path::new("resources/rickroll.gif").canonicalize());
    NamedFile::open(Path::new("api/resources/rickroll.gif")).await.ok()
}
