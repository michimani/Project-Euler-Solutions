NO?=0

.PHONY: new-python new-go

new-python:
	python python/new_solution.py ${NO}

new-go:
	go run go/new_solution.go ${NO}
