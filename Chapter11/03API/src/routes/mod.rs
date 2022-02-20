use rocket::http::Status;
use rocket::response::Redirect;
use rocket::Shutdown;
use rocket_dyn_templates::Template;

pub mod post;
pub mod session;
pub mod user;
pub mod api;

type HtmlResponse = Result<Template, Status>;

#[get("/")]
pub async fn home() -> Redirect {
    Redirect::to("/users")
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
