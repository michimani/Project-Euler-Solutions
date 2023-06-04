NO?=0

.PHONY: new-python new-go

new-python:
	python3 python/new_solution.py ${NO}

new-go:
	cd go && go run . -c new -p ${NO}

new-rust:
	cd rust && cargo run --bin new ${NO}
