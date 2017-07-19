use services::users;
use db::models::{UserView, NewUser, UserLogin};
use rocket_contrib::{Json};
use rocket::http::Status;
use rocket::response::status;
use rocket::http::{Cookies, Cookie};

#[get("/users/<username>")]
fn get_user(username: String) -> Option<Json<UserView>> {
    let user: Result<UserView, &str> = users::get_user(username.to_string());

    match user {
        Ok(user_res) => Some(Json(user_res)),
        Err(_) => None
    }
}

#[post("/register", data = "<json_user>")]
fn register<'a>(json_user: Json<NewUser>) -> Result<status::Created<Json<UserView>>, Status> {
    let new_user: NewUser = json_user.into_inner();
    let user = users::create_user(&new_user);
    match user {
        Ok(user_created) => {
            let response = status::Created("/register".to_string(), Some(Json(user_created)));
            Ok(response)
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/login", data = "<user_login>")]
fn login<'a>(mut cookies: Cookies, user_login: Json<UserLogin>) -> Result<Json<UserView>, Status> {
    match users::authenitcate_user(&user_login.username, &user_login.password) {
        Ok(username) => {
            let user: Result<UserView, &str> = users::get_user(username.to_string());

            match user {
                Ok(user_res) => {
                    cookies.add_private(Cookie::new("user_id", user_res.id.clone().to_string()));
                    Ok(Json(user_res))
                },
                Err(_) => Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::BadRequest)
    }
}
