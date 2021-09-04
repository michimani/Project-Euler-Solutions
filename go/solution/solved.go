package solution

import "fmt"

var solvedProblem map[int]func() = map[int]func(){
	1:  Solve0001,
	2:  Solve0002,
	3:  Solve0003,
	4:  Solve0004,
	5:  Solve0005,
	6:  Solve0006,
	7:  Solve0007,
	8:  Solve0008,
	10: Solve0010,
	12: Solve0012,
	13: Solve0013,
	15: Solve0015,
	16: Solve0016,
	20: Solve0020,
	25: Solve0025,
	26: Solve0026,
	28: Solve0028,
}

func Solve(problemNo int) error {
	solveFn, ok := solvedProblem[problemNo]
	if !ok {
		return fmt.Errorf("There is no solution for problem No.%d yet.", problemNo)
	}

	fmt.Printf("The answer for the problem No.%d is ... ", problemNo)
	solveFn()
	return nil
}