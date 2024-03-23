pull:
	docker pull postgres:latest

run-db:
	docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres

remove-db:
	docker stop some-postgres && docker rm some-postgres

bash-in:
	docker exec -it some-postgres psql -U postgres
