use chrono::{offset::Utc, DateTime};
use gloo_utils::document;
use reqwasm::http::Request;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;
use yew::prelude::*;

const USERS_URL: &str = "http://127.0.0.1:8000/api/users";

struct DisplayOption<T>(pub Option<T>);

impl<T: Display> Display for DisplayOption<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            Some(ref v) => write!(f, "{}", v),
            None => write!(f, ""),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq)]
enum UserStatus {
    Inactive = 0,
    Active = 1,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UserStatus::Inactive => write!(f, "Inactive"),
            UserStatus::Active => write!(f, "Active"),
        }
    }
}

#[derive(Copy, Clone, Deserialize, PartialEq)]
struct OurDateTime(DateTime<Utc>);

impl fmt::Display for OurDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Clone, PartialEq)]
struct User {
    uuid: Uuid,
    username: String,
    email: String,
    description: Option<String>,
    status: UserStatus,
    created_at: OurDateTime,
    updated_at: OurDateTime,
}

#[derive(Clone, Copy, Deserialize, PartialEq)]
struct Pagination {
    next: OurDateTime,
    limit: usize,
}

#[derive(Deserialize, Default, Properties, PartialEq)]
struct UsersWrapper {
    users: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pagination: Option<Pagination>,
}

#[function_component(UsersList)]
fn users_list(UsersWrapper { users, .. }: &UsersWrapper) -> Html {
    users.iter()
        .enumerate().map(|user| html! {
        <div class="container">
            <div><mark class="tag">{ format!("{}", user.0) }</mark></div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "UUID:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", user.1.uuid) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Username:" }</mark></div>
                <div class="col-sm-9">{ format!("{}", user.1.username) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Email:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", user.1.email) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Description:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", DisplayOption(user.1.description.as_ref())) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Status:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", user.1.status) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Created At:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", user.1.created_at) }</div>
            </div>
            <div class="row">
                <div class="col-sm-3"><mark>{ "Updated At:" }</mark></div>
                <div class="col-sm-9"> { format!("{}", user.1.updated_at) }</div>
            </div>
            <a href={format!("/users/{}", user.1.uuid)} class="button">{ "See user" }</a>
        </div>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let users_wrapper = use_state(|| UsersWrapper::default());
    {
        let users_wrapper = users_wrapper.clone();
        use_effect_with_deps(
            move |_| {
                let users_wrapper = users_wrapper.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users_wrapper: UsersWrapper = Request::get(USERS_URL)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    users_wrapper.set(fetched_users_wrapper);
                });
                || ()
            },
            (),
        );
    }
    let (next, limit): (Option<OurDateTime>, Option<usize>) = if users_wrapper.pagination.is_some()
    {
        let pagination = users_wrapper.pagination.as_ref().unwrap();
        (Some(pagination.next), Some(pagination.limit))
    } else {
        (None, None)
    };
    html! {
        <>
            <UsersList users = {users_wrapper.users.clone()}/>
            if next.is_some() {
                <a href={ format!("/users?pagination.next={}&pagination.limit={}", DisplayOption(next), DisplayOption(limit)) } class="button">
                    { "Next" }
                </a>
            }
        </>
    }
}

fn main() {
    let document = document();
    let main_container = document.query_selector("#main_container").unwrap().unwrap();

    yew::start_app_in_element::<App>(main_container);
}
