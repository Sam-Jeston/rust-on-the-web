use super::schema::posts;
use super::schema::users;

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
