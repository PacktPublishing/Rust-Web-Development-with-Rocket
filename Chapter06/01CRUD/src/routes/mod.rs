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

const USER_HTML_PREFIX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>Blank HTML 5 Template</title>
</head>
<body>"#;

const USER_HTML_SUFFIX: &str = r#"</body>
</html>"#;
