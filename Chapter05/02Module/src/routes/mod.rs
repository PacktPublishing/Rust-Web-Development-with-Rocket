use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::content::RawHtml;

pub mod posts;
pub mod users;

type HtmlResponse = Result<RawHtml<String>, Status>;

#[get("/<filename>")]
pub async fn assets(filename: &str) -> NamedFile {
    todo!("will implement later")
}
