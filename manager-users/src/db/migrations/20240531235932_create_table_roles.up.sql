-- Add up migration script here
CREATE TABLE
	IF NOT EXISTS app.roles (
		id UUID PRIMARY KEY DEFAULT uuid_generate_v1 (),
		name VARCHAR(20) UNIQUE NOT NULL,
		description VARCHAR
	);

COMMENT ON TABLE app.roles IS 'Table for storing user roles';

COMMENT ON COLUMN app.roles.id IS 'Unique identifier for the role';

COMMENT ON COLUMN app.roles.name IS 'Name of the role';

COMMENT ON COLUMN app.roles.description IS 'Description of the role';