-- Add up migration script here
INSERT INTO
	app.users (username, password, rol_id)
VALUES
	('elpropio@example.com', '$2y$10$iehE5KT5eQ0Nx8TOZCCPpOh6H61zG7O1cUzOSkWD8TWp7hA4E1Hd.', (SELECT id FROM app.roles WHERE "name" = 'READER')), -- pass1
	('lafirma@example.com', '$2y$08$XuxC4rYOjulX4nY3tlljee1puUDp927TDJx3iwybK6XzKmssqs0eK', (SELECT id FROM app.roles WHERE "name" = 'READER')), -- pass2
	('elprofe@example.com', '$2y$08$m5/s8mipmSjjUtJJ7pWo4.uItNryaqNU5ioc2zf1VhV5NxyreeRk.', (SELECT id FROM app.roles WHERE "name" = 'ADMIN')) -- pass3
ON CONFLICT DO NOTHING;