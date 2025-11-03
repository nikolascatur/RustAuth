use anyhow::anyhow;
use rand::Error;
use sqlx::PgPool;

use crate::{
    model::{
        session::UserSession,
        users::{CreateUser, LoginUser, User},
    },
    repository::user_repo::UserRepo,
    util::validation::is_valid_email,
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

    pub async fn create_user(&self, input: CreateUser) -> Result<UserSession, anyhow::Error> {
        let user = self.userRepo.create_user(input).await;
        user
    }

    pub async fn is_user_exist(&self, email: &String) -> Result<bool, anyhow::Error> {
        if !is_valid_email(email) {
            return Err(anyhow!("Invalid email format"));
        }
        let count = self.userRepo.check_user(email).await;
        count
    }

    pub async fn login(&self, login: &LoginUser) -> Result<UserSession, anyhow::Error> {
        let user_session = self.userRepo.login(login).await;
        user_session
    }
}
