#[macro_use]
extern crate rocket;

use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::fs::{relative, NamedFile};
use rocket::http::{ContentType, Header, Status};
use rocket::request::{FromParam, Request};
use rocket::response::{self, Responder, Response};
use rocket::{Build, Data, Orbit, Rocket};
use rocket_db_pools::{
    sqlx,
    sqlx::{FromRow, PgPool},
    Connection, Database,
};
use std::io::Cursor;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::vec::Vec;
use uuid::Uuid;

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

struct NameGrade<'r> {
    name: &'r str,
    grade: u8,
}

impl<'r> FromParam<'r> for NameGrade<'r> {
    type Error = &'static str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        const ERROR_MESSAGE: Result<NameGrade, &'static str> = Err("Error parsing user parameter");

        let name_grade_vec: Vec<&'r str> = param.split('_').collect();
        match name_grade_vec.len() {
            2 => match name_grade_vec[1].parse::<u8>() {
                Ok(n) => Ok(Self {
                    name: name_grade_vec[0],
                    grade: n,
                }),
                Err(_) => ERROR_MESSAGE,
            },
            _ => ERROR_MESSAGE,
        }
    }
}

fn default_response<'r>() -> response::Response<'r> {
    Response::build()
        .header(ContentType::Plain)
        .raw_header("X-CUSTOM-ID", "CUSTOM")
        .finalize()
}

#[derive(Debug, FromRow)]
struct User {
    uuid: Uuid,
    name: String,
    age: i16,
    grade: i16,
    active: bool,
}

impl<'r> Responder<'r, 'r> for User {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user = format!("Found user: {:?}", self);
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-USER-ID", self.uuid.to_string())
            .merge(base_response)
            .ok()
    }
}

struct NewUser(Vec<User>);

impl<'r> Responder<'r, 'r> for NewUser {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user = self
            .0
            .iter()
            .map(|u| format!("{:?}", u))
            .collect::<Vec<String>>()
            .join(",");
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-CUSTOM-ID", "USERS")
            .join(base_response)
            .ok()
    }
}

struct VisitorCounter {
    visitor: AtomicU64,
}

impl VisitorCounter {
    fn increment_counter(&self) {
        self.visitor.fetch_add(1, Ordering::Relaxed);
        println!(
            "The number of visitor is: {}",
            self.visitor.load(Ordering::Relaxed)
        );
    }
}

#[rocket::async_trait]
impl Fairing for VisitorCounter {
    fn info(&self) -> Info {
        Info {
            name: "Visitor Counter",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Request,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        println!("Setting up visitor counter");
        Ok(rocket)
    }

    async fn on_liftoff(&self, _: &Rocket<Orbit>) {
        println!("Finish setting up visitor counter");
    }

    async fn on_request(&self, _: &mut Request<'_>, _: &mut Data<'_>) {
        self.increment_counter();
    }
}

const X_TRACE_ID: &str = "X-TRACE-ID";
struct XTraceId {}

#[rocket::async_trait]
impl Fairing for XTraceId {
    fn info(&self) -> Info {
        Info {
            name: "X-TRACE-ID Injector",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let header = Header::new(X_TRACE_ID, Uuid::new_v4().to_hyphenated().to_string());
        req.add_header(header);
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let header = req.headers().get_one(X_TRACE_ID).unwrap();
        res.set_header(Header::new(X_TRACE_ID, header));
    }
}

#[derive(Database)]
#[database("main_connection")]
struct DBConnection(PgPool);

#[get("/user/<uuid>", rank = 1, format = "text/plain")]
async fn user(mut db: Connection<DBConnection>, uuid: &str) -> Result<User, Status> {
    let parsed_uuid = Uuid::parse_str(uuid).map_err(|_| Status::BadRequest)?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE uuid = $1", parsed_uuid)
        .fetch_one(&mut *db)
        .await;
    user.map_err(|_| Status::NotFound)
}

#[get("/users/<name_grade>?<filters..>")]
async fn users(
    mut db: Connection<DBConnection>,
    name_grade: NameGrade<'_>,
    filters: Option<Filters>,
) -> Result<NewUser, Status> {
    let mut query_str = String::from("SELECT * FROM users WHERE name LIKE $1 AND grade = $2");
    if filters.is_some() {
        query_str.push_str(" AND age = $3 AND active = $4");
    }
    let mut query = sqlx::query_as::<_, User>(&query_str)
        .bind(format!("%{}%", &name_grade.name))
        .bind(name_grade.grade as i16);
    if let Some(fts) = &filters {
        query = query.bind(fts.age as i16).bind(fts.active);
    }
    let unwrapped_users = query.fetch_all(&mut *db).await;
    let users: Vec<User> = unwrapped_users.map_err(|_| Status::InternalServerError)?;
    if users.is_empty() {
        Err(Status::NotFound)
    } else {
        Ok(NewUser(users))
    }
}

#[get("/favicon.png")]
async fn favicon() -> NamedFile {
    NamedFile::open(Path::new(relative!("static")).join("favicon.png"))
        .await
        .unwrap()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("We cannot find this page {}.", req.uri())
}

#[catch(403)]
fn forbidden(req: &Request) -> String {
    format!("Access forbidden {}.", req.uri())
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let visitor_counter = VisitorCounter {
        visitor: AtomicU64::new(0),
    };

    let x_trace_id = XTraceId {};

    rocket::build()
        .attach(DBConnection::init())
        .attach(visitor_counter)
        .attach(x_trace_id)
        .mount("/", routes![user, users, favicon])
        .register("/", catchers![not_found, forbidden])
}
