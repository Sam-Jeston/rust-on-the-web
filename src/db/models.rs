use super::schema::posts;
use super::schema::users;
use services;
use rocket::request::{self, FromRequest, Request};
use rocket::http::{Cookie, Cookies};
use rocket::outcome::Outcome::*;
use rocket::outcome::IntoOutcome;
use rocket::response::status;
use rocket::http::Status;

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub confirm_password: String
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct ConstructedUser {
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct PostShort {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Queryable, Serialize, Deserialize,  Debug, Clone)]
pub struct NewPost {
    pub title: String,
    pub caption: String,
    pub body: String
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct ConstructedPost {
    pub title: String,
    pub caption: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String
}

// TODO: Find a much better place for these
impl<'a, 'r> FromRequest<'a, 'r> for UserView {
    type Error = &'a str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, &'a str> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| {
                match services::users::get_user(id) {
                    Ok(user_view) => Success(user_view),
                    Err(_) => Failure((Status::Unauthorized, "Invalid User"))
                }
            }).unwrap()
    }
}
