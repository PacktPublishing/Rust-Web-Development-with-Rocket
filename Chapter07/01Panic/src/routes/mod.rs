use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::content::RawHtml;

pub mod post;
pub mod user;

type HtmlResponse = Result<RawHtml<String>, Status>;

#[get("/<_filename>")]
pub async fn assets(_filename: &str) -> NamedFile {
    todo!("will implement later")
}
