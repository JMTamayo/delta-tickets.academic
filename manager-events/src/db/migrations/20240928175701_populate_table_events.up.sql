-- Add up migration script here
INSERT INTO
	events.events (name)
VALUES
	('SUPERBOWL_2024')
ON CONFLICT DO NOTHING;