-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS chat_rooms CASCADE;
DROP TABLE IF EXISTS user_chat_rooms CASCADE;
DROP TABLE IF EXISTS user_status CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS chat_room_invites CASCADE;

DROP FUNCTION IF EXISTS update_timestamp();

DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_chat_rooms_updated_at ON chat_rooms;
DROP TRIGGER IF EXISTS update_messages_updated_at ON messages;
