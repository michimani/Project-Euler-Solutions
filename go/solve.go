package main

import (
	"fmt"

	"github.com/michimani/project-euler-go/solution"
)

func solveProblem(problemNo int) {
	switch problemNo {
	case 1:
		solution.Solve0001()
	case 2:
		solution.Solve0002()
	case 3:
		solution.Solve0003()
	case 4:
		solution.Solve0004()
	case 5:
		solution.Solve0005()
	case 6:
		solution.Solve0006()
	case 7:
		solution.Solve0007()
	case 8:
		solution.Solve0008()
	case 10:
		solution.Solve0010()
	case 12:
		solution.Solve0012()
	case 13:
		solution.Solve0013()
	case 15:
		solution.Solve0015()
	case 16:
		solution.Solve0016()
	case 20:
		solution.Solve0020()
	case 25:
		solution.Solve0025()
	case 26:
		solution.Solve0026()
	default:
		fmt.Printf("Solution for problem number %d does not exists.", problemNo)
	}
}
