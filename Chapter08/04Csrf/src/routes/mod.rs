use rocket::fs::{relative, NamedFile};
use rocket::http::Status;
use rocket::Shutdown;
use rocket_dyn_templates::Template;
use std::path::{Path, PathBuf};

pub mod post;
pub mod user;

type HtmlResponse = Result<Template, Status>;

#[get("/<filename..>")]
pub async fn assets(filename: PathBuf) -> Option<NamedFile> {
    let mut filename = Path::new(relative!("static")).join(filename);
    if filename.is_dir() {
        filename.push("index.html");
    }

    NamedFile::open(filename).await.ok()
}

#[get("/shutdown")]
pub async fn shutdown(shutdown: Shutdown) -> &'static str {
    // suppose this variable is from function which produces irrecoverable error
    let result: Result<&str, &str> = Err("err");
    if result.is_err() {
        shutdown.notify();
        return "Shuting down the application.";
    }
    "Not doing anything."
}
