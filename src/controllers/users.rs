use services::users;
use db::models::{UserView, NewUser};
use rocket_contrib::{JSON};
use rocket::http::Status;
use rocket::response::status;

#[get("/users/<username>")]
fn get_user(username: &str) -> Option<JSON<UserView>> {
    let user: Result<UserView, &str> = users::get_user(username.to_string());

    match user {
        Ok(user_res) => Some(JSON(user_res)),
        Err(_) => None
    }
}

#[post("/register", data = "<json_user>")]
fn register<'a>(json_user: JSON<NewUser>) -> Result<status::Created<JSON<UserView>>, Status> {
    let new_user: NewUser = json_user.into_inner();
    let user = users::create_user(&new_user);
    match user {
        Ok(user_created) => {
            let response = status::Created("/register".to_string(), Some(JSON(user_created)));
            Ok(response)
        },
        Err(_) => Err(Status::InternalServerError)
    }
}
