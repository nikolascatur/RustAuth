-- Add migration script here
-- CREATE EXTENSION IF NOT EXISTS dblink;
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Buat database dbauth jika belum ada
-- DO
-- $$
-- BEGIN
--     IF NOT EXISTS (
--         SELECT FROM pg_database WHERE datname = 'dbauth'
--     ) THEN
--         PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE dbauth');
--     END IF;
-- END
-- $$;

CREATE Table users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    email VARCHAR(200) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    update_at TIMESTAMP DEFAULT NOW()
);