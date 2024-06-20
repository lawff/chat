-- Add migration script here
-- if new message added, notify with message data
CREATE OR REPLACE FUNCTION notify_message_added()
  RETURNS TRIGGER
  AS $$
DECLARE
  USERS bigint[];
BEGIN
  IF TG_OP = 'INSERT' THEN
    RAISE NOTICE 'message_added: %', NEW;
    -- select chat with chat_id in NEW
    SELECT
      members INTO USERS
    FROM
      chats
    WHERE
      id = NEW.chat_id;
    PERFORM
      pg_notify('message_added', json_build_object('message', NEW, 'members', USERS)::text);
  END IF;
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;
