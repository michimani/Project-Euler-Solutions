package main

import (
	"flag"
	"fmt"
	"os"
)

func usage() {
	u := `
Usage:
  go run . [flag] [value]
Flags:
  -c (required) Sub comand
							new   Create new solution
							solve Solve a problem
	-p (required) Number of problem
	`

	fmt.Fprintln(os.Stderr, u)
}

var subCmd = flag.String("c", "", "Sub Command: new, solve")
var problemNo = flag.Int("p", 0, "Number of problem")

func main() {
	flag.Usage = usage
	flag.Parse()

	os.Exit(run())
}

func run() int {
	if *subCmd != "new" && *subCmd != "solve" {
		fmt.Println("Invalid sub-command.")
		usage()
		return 1
	}

	if *problemNo <= 0 {
		fmt.Println("Invalid value for problem number.")
		fmt.Println(*problemNo)
		return 1
	}

	switch *subCmd {
	case "new":
		createNewSolutionScript(*problemNo)
	case "solve":
		solveProblem(*problemNo)
	}

	return 0
}
