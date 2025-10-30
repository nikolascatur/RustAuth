use rand::Error;
use sqlx::PgPool;

use crate::{
    model::users::{CreateUser, User},
    repository::user_repo::UserRepo,
};

pub struct UserService {
    userRepo: UserRepo,
}

impl UserService {
    pub fn new(db: &PgPool) -> Self {
        Self {
            userRepo: UserRepo::new(db),
        }
    }

    pub async fn create_user(&self, input: CreateUser) -> Result<User, anyhow::Error> {
        let user = self.userRepo.create_user(input).await;
        return user;
    }

    pub async fn is_user_exist(&self, email: String) -> Result<bool, anyhow::Error> {
        let count = self.userRepo.check_user(email).await;
        return count;
    }
}
