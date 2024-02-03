
use teloxide::types::{User, UserId};
pub struct  UserDB {
    pub _id: UserId,
    pub name: String,
    pub username: String
}

impl UserDB {
    pub fn new(_id: UserId, name: String, username: Option<String>) -> UserDB {
        let uname = match username {
            Some(u) => u,
            None => "none".to_string()
        };
        
        UserDB {_id, name, username: uname}
    }

    pub fn from_user(user: &User) -> Self {
        Self::new(user.id, user.full_name(), user.username.clone())
    }
}