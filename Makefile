build:
	docker-compose -p delta-tickets build

create:
	docker-compose -p delta-tickets create

deploy:
	docker-compose -p delta-tickets up -d

down:
	docker-compose -p delta-tickets down

start:
	docker-compose -p delta-tickets start

stop:
	docker-compose -p delta-tickets stop