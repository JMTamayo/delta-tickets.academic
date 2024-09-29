-- Add up migration script here
CREATE TABLE
	IF NOT EXISTS events.event_user_relationship (
		id UUID PRIMARY KEY DEFAULT uuid_generate_v1 (),
		event_id UUID REFERENCES events.events (id),
		user_id UUID NOT NULL,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ
	);

COMMENT ON TABLE events.event_user_relationship IS 'Table for storing even-user relationships';

COMMENT ON COLUMN events.event_user_relationship.id IS 'Unique identifier for the event-user relationship';

COMMENT ON COLUMN events.event_user_relationship.event_id IS 'Foreign key to the event';

COMMENT ON COLUMN events.event_user_relationship.user_id IS 'Foreign key to the user';

COMMENT ON COLUMN events.event_user_relationship.created_at IS 'Timestamp when the event-user relationship was created';

COMMENT ON COLUMN events.event_user_relationship.updated_at IS 'Timestamp when the event-user relationship was last updated';