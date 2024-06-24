-- Add migration script here
ALTER TABLE chats ADD COLUMN deleted_at TIMESTAMP NULL DEFAULT NULL;
