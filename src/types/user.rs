use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password_hash: String,
    pub phone_number: String,
}
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password_hash: String,
    pub phone_number: String,
}


#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UserId(pub i32);


#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}