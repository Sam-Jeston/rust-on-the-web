use db::models::*;
use diesel::prelude::*;
use db;
use pwhash::bcrypt;
use helpers;
use diesel::insert;

pub fn get_user<'a>(target_username: String) -> Result<UserView, &'a str> {
    use db::schema::users::dsl::*;

    let connection = db::connection::establish_connection();
    let results = users.select((id, username, created_at, updated_at))
        .filter(username.eq(target_username))
        .limit(1)
        .load::<UserView>(&connection)
        .expect("Error loading Users");

    if results.len() == 0 {
        Err("This user does not exist")
    } else {
        Ok(results[0].clone())
    }
}

pub fn create_user<'a>(new_user: &'a NewUser) -> Result<UserView, &'a str> {
    use db::schema::users;
    let connection = db::connection::establish_connection();

    match new_user.password == new_user.confirm_password {
        true => {
            let created_at = helpers::current_time();
            let updated_at = helpers::current_time();
            let hashed_pasword = bcrypt::hash(&new_user.password).unwrap();

            let constructed_user = ConstructedUser {
                username: new_user.username.clone(),
                password: hashed_pasword,
                created_at: created_at,
                updated_at: updated_at
            };

            let _ = insert(&constructed_user).into(users::table).execute(&connection);

            get_user(new_user.username.clone())
        },
        false => Err("Passwords do not match")
    }
}
