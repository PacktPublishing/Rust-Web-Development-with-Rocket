use rocket::request::Request;
use rocket::response::content::RawHtml;

const ERROR_HTML_PREFIX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>Error</title>
</head>
<body>"#;

const ERROR_HTML_SUFFIX: &str = r#"</body>
</html>"#;

#[catch(400)]
pub fn bad_request(_: &Request) -> RawHtml<String> {
    RawHtml(format!(
        "{}{}{}",
        ERROR_HTML_PREFIX, "Invalid input", ERROR_HTML_SUFFIX
    ))
}

#[catch(404)]
pub fn not_found(_: &Request) -> RawHtml<String> {
    RawHtml(format!(
        "{}{}{}",
        ERROR_HTML_PREFIX, "We cannot found that resource", ERROR_HTML_SUFFIX
    ))
}

#[catch(422)]
pub fn unprocessable_entity(r: &Request) -> RawHtml<String> {
    bad_request(r)
}

#[catch(500)]
pub fn internal_server_error(_: &Request) -> RawHtml<String> {
    RawHtml(format!(
        "{}{}{}",
        ERROR_HTML_PREFIX, "Something went wrong", ERROR_HTML_SUFFIX
    ))
}
