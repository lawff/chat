-- Add migration script here
-- if chat changed, notify with chat data
CREATE OR REPLACE FUNCTION notify_chat_change()
RETURNS TRIGGER AS $$
BEGIN
  RAISE NOTICE 'chat changed: %', NEW;
  PERFORM pg_notify('chat_change', json_build_object(
    'op', TG_OP,
    'old', OLD,
    'new', NEW
  )::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER chat_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON chats
FOR EACH ROW
EXECUTE FUNCTION notify_chat_change();

-- if new message added, notify with message data
CREATE OR REPLACE FUNCTION notify_message_added()
RETURNS TRIGGER AS $$
BEGIN
  IF TG_OP = 'INSERT' THEN
    RAISE NOTICE 'message added: %', NEW;
    PERFORM pg_notify('message_added', row_to_json(NEW)::text);
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER message_added_trigger
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION notify_message_added();
