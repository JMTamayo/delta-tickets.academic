-- Add up migration script here
INSERT INTO
	events.event_user_relationship (event_id, user_id)
VALUES
	((SELECT id FROM events.events WHERE "name" = 'SUPERBOWL_2024'), 'a728d7dc-5d73-4a05-a960-baa3a7705335')
ON CONFLICT DO NOTHING;