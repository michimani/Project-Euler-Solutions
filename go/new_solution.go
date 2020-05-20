package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// CreateNewSolutionScript is a function to create new solution script.
func CreateNewSolutionScript(problemNo int) {
	var scriptTemplate string = `
//
// Solution for Project Euler problem ##PROBLEM_NO##
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=##PROBLEM_NO##
//
package main

import "fmt"

// Compute is function to solve the problem.
func Compute() string {
	var answer string

	return answer
}

func main() {
	fmt.Println(Compute())
}`

	currDir, _ := os.Getwd()
	var solutionDir string = fmt.Sprintf("%s/go/%04d", currDir, problemNo)
	var solutionFileName string = fmt.Sprintf("%s/main.go", solutionDir)

	os.Mkdir(solutionDir, 0777)
	file, _ := os.Create(solutionFileName)
	defer file.Close()

	writer := bufio.NewWriter(file)
	writer.Write([]byte(strings.Replace(scriptTemplate, "##PROBLEM_NO##", strconv.Itoa(problemNo), 2)))
	writer.Flush()
}

func main() {
	flag.Parse()
	args := flag.Args()
	if len(args) == 0 {
		fmt.Println("The first parameter is required.")
	} else {
		problemNo, err := strconv.Atoi(args[0])
		if err != nil {
			fmt.Println("The first parameter is must be int type.")
		} else {
			CreateNewSolutionScript(problemNo)
		}
	}
}
