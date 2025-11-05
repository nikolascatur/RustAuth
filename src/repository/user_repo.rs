use crate::model::session::{CreateSession, UserSession};
use crate::model::users::{CountUser, CreateUser, InfoUser, LoginUser, Logout, Session, User};
use crate::util::security::{hash_password, verify_passwrod};
use anyhow::{Ok, Result, anyhow};
use secrecy::SecretString;
use sqlx::PgPool;
use sqlx::types::{chrono, uuid};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserRepo {
    db: PgPool,
}

impl UserRepo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create_user(&self, create: CreateUser) -> Result<UserSession, anyhow::Error> {
        let password = SecretString::new(create.password);
        let password_hash = hash_password(&password).unwrap_or_else(|e| "".to_string()); //.map_err(|e| e.to_string());
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password
        "#,
            create.name,
            create.email,
            password_hash
        )
        .fetch_one(&self.db)
        .await?;
        let session = self
            .create_session(CreateSession { user_id: user.id })
            .await
            .map_err(|err| anyhow!("Session create failed{}", err));
        session
    }

    pub async fn check_user(&self, email: &String) -> Result<bool, anyhow::Error> {
        let count_user = sqlx::query_as!(
            CountUser,
            "SELECT COUNT(email) as count FROM users WHERE email LIKE $1",
            email
        )
        .fetch_one(&self.db)
        .await?;

        return Ok(count_user.count.unwrap() > 0);
    }

    pub async fn login(&self, login: &LoginUser) -> Result<UserSession, anyhow::Error> {
        let password = SecretString::new(login.password.clone());
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email, password FROM users WHERE email LIKE $1",
            login.email
        )
        .fetch_one(&self.db)
        .await?;

        let is_valid = verify_passwrod(&password, &user.password)
            .map_err(|err| anyhow!("Password invalid {}", err))?;
        if is_valid == true {
            let session = self
                .create_session(CreateSession { user_id: user.id })
                .await
                .map_err(|err| anyhow!(err));
            session
        } else {
            Err(anyhow!("Wrong password"))
        }
    }

    pub async fn delete_session(&self, logout: &Logout) -> Result<bool, anyhow::Error> {
        let del = sqlx::query!(
            "DELETE FROM user_sessions WHERE session_token = $1",
            &logout.session_id
        )
        .execute(&self.db)
        .await?
        .rows_affected();
        Ok(del > 0)
    }

    pub async fn create_session(
        &self,
        session: CreateSession,
    ) -> Result<UserSession, anyhow::Error> {
        let token = Uuid::new_v4();
        let refresh_token = Uuid::new_v4();
        let age = chrono::Utc::now().timestamp() + 108000;
        let session = sqlx::query_as!(
            UserSession,
            r#"INSERT INTO user_sessions(user_id, session_token, refresh_token, timeout) VALUES ($1, $2, $3, $4) 
            RETURNING id, user_id,session_token,refresh_token,timeout"#,
            session.user_id,
            token.to_string(),
            refresh_token.to_string(),
            age
        ).fetch_one(&self.db).await?;

        Ok(session)
    }

    pub async fn is_session_active(&self, session_id: &String) -> Result<bool, anyhow::Error> {
        let session = sqlx::query_as!(
            Session,
            "SELECT timeout FROM user_sessions WHERE session_token = $1",
            session_id
        )
        .fetch_one(&self.db)
        .await?;
        let check_active = &session.timeout - &chrono::Utc::now().timestamp();
        Ok(check_active > 0)
    }

    pub async fn info_user(&self, id: Uuid) -> Result<InfoUser, anyhow::Error> {
        let user = sqlx::query_as!(
            InfoUser,
            "SELECT id, name, email FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }
}
