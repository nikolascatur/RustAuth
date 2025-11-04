use anyhow::anyhow;
use sqlx::PgPool;

use crate::{
    model::{
        session::UserSession,
        users::{CreateUser, LoginUser},
    },
    repository::user_repo::UserRepo,
    util::validation::is_valid_email,
};

pub struct UserService {
    user_repo: UserRepo,
}

impl UserService {
    pub fn new(db: &PgPool) -> Self {
        Self {
            user_repo: UserRepo::new(db),
        }
    }

    pub async fn create_user(&self, input: CreateUser) -> Result<UserSession, anyhow::Error> {
        let user = self.user_repo.create_user(input).await;
        user
    }

    pub async fn is_user_exist(&self, email: &String) -> Result<bool, anyhow::Error> {
        if !is_valid_email(email) {
            return Err(anyhow!("Invalid email format"));
        }
        let count = self.user_repo.check_user(email).await;
        count
    }

    pub async fn login(&self, login: &LoginUser) -> Result<UserSession, anyhow::Error> {
        let user_session = self.user_repo.login(login).await;
        user_session
    }
}
