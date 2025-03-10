NO?=0

new-python:
	python3 python/new_solution.py ${NO}

new-go:
	cd go && go run . -c new -p ${NO}

new-rust:
	cd rust && cargo run --bin new ${NO}

new-java:
	cd java && java src/New.java ${NO}
