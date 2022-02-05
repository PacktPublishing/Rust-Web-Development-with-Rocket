pub struct RawContentType<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for RawContentType<'r> {
    type Error = ();

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let header = req.headers().get_one("Content-Type").or(Some("")).unwrap();
        rocket::request::Outcome::Success(RawContentType(header))
    }
}
