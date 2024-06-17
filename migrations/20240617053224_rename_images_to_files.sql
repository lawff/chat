-- Add migration script here
ALTER TABLE messages RENAME COLUMN images TO files;
