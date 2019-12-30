build:
	docker build -t=tnt .

run: stop build
	docker run --rm -it --name tnt -p3301:3301 -d tnt

logs:
	docker logs -f tnt

stop:
	docker stop tnt || true

run-test:
	cd test && go run test
