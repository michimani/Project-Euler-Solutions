NO?=0

.PHONY: new-python new-go

new-python:
	python python/new_solution.py ${NO}

new-go:
	cd go && go run . -c new -p ${NO}