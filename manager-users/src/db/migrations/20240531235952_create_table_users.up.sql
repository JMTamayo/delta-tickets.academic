-- Add up migration script here
CREATE TABLE
	IF NOT EXISTS app.users (
		id UUID PRIMARY KEY DEFAULT uuid_generate_v1 (),
		username VARCHAR(50) UNIQUE NOT NULL,
		password VARCHAR(255) NOT NULL,
		rol_id UUID REFERENCES app.roles (id),
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ
	);

COMMENT ON TABLE app.users IS 'Table for storing user account information';

COMMENT ON COLUMN app.users.id IS 'Unique identifier for the user';

COMMENT ON COLUMN app.users.username IS 'Username of the user';

COMMENT ON COLUMN app.users.password IS 'Hashed password for the user';

COMMENT ON COLUMN app.users.rol_id IS 'Role of the user';

COMMENT ON COLUMN app.users.created_at IS 'Timestamp of when the user was created';

COMMENT ON COLUMN app.users.updated_at IS 'Timestamp of the last update to the user';