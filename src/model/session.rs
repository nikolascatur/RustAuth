use uuid::Uuid;

pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub refresh_token: String,
    pub timeout: i64,
}

pub struct CreateSession {
    pub user_id: Uuid,
}
