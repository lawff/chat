-- Add migration script here
ALTER TABLE chats ALTER COLUMN deleted_at TYPE TIMESTAMPTZ;
