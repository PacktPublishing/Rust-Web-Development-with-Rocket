use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::{content::RawHtml, Flash, Redirect};

pub mod post;
pub mod user;

type HtmlResponse = Result<RawHtml<String>, Status>;
type RedirectResponse = Result<RawHtml<String>, Flash<Redirect>>;

#[get("/<_filename>")]
pub async fn assets(_filename: &str) -> NamedFile {
    todo!("will implement later")
}
