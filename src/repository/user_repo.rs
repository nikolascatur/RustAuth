use crate::model::users::{CountUser, CreateUser, LoginUser, User};
use crate::util::security::{hash_password, verify_passwrod};
use anyhow::{Error, Ok, Result, anyhow};
use secrecy::SecretString;
use sqlx::PgPool;

pub struct UserRepo {
    db: PgPool,
}

impl UserRepo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create_user(&self, create: CreateUser) -> Result<User, anyhow::Error> {
        let password = SecretString::new(create.password);
        let password_hash = hash_password(&password).unwrap_or_else(|e| "".to_string()); //.map_err(|e| e.to_string());
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email
        "#,
            create.name,
            create.email,
            password_hash
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| anyhow!("{}", err));
        return user;
    }

    pub async fn check_user(&self, email: String) -> Result<bool, anyhow::Error> {
        let count_user = sqlx::query_as!(
            CountUser,
            "SELECT COUNT(email) as count FROM users WHERE email LIKE $1",
            &email
        )
        .fetch_one(&self.db)
        .await?;

        return Ok(count_user.count.unwrap() > 0);
    }

    pub async fn login(&self, login: LoginUser) {
        let password = SecretString::new(login.password);
        let user = sqlx::query_as!(
            LoginUser,
            "SELECT email, password FROM users WHERE email LIKE $1",
            login.email
        )
        .fetch_one(&self.db)
        .await
        .expect("");

        let result = verify_passwrod(&password, &user.password);
        // .unwrap_or_else(|e| e.to_string());
        // match User {
        //     Ok => verify_passwrod(&password, usr);
        //     _ => {}

        // }
    }
}
