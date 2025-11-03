-- Add migration script here
CREATE TABLE user_sessions(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE NOT NULL,
    timeout BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    update_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT fk_user_sessions 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);