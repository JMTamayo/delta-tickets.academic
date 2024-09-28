-- Add up migration script here
INSERT INTO
	app.roles (name, description)
VALUES
	('ADMIN', 'Full access'),
	('READER', 'Only read access')
ON CONFLICT DO NOTHING;