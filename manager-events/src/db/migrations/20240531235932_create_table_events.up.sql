-- Add up migration script here
CREATE TABLE
	IF NOT EXISTS events.events (
		id UUID PRIMARY KEY DEFAULT uuid_generate_v1 (),
		name VARCHAR(20) UNIQUE NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);

COMMENT ON TABLE events.events IS 'Table for storing events';

COMMENT ON COLUMN events.events.id IS 'Unique identifier for the event';

COMMENT ON COLUMN events.events.name IS 'Name of the event';

COMMENT ON COLUMN events.events.created_at IS 'Timestamp of when the event was created';