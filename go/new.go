package main

import (
	"bufio"
	"fmt"
	"os"
)

// createNewSolutionScript is a function to create new solution script.
func createNewSolutionScript(problemNo int) {
	var scriptTemplate string = `
//
// Solution for Project Euler problem %d
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=%d
//
package solution

import "fmt"

// compute is function to solve the problem.
func Solve%04d() {
	var answer int

	fmt.Println(answer)
}
`

	currDir, _ := os.Getwd()
	var solutionDir string = fmt.Sprintf("%s/solution", currDir)
	var solutionFileName string = fmt.Sprintf("%s/%04d.go", solutionDir, problemNo)

	os.Mkdir(solutionDir, 0777)
	file, _ := os.Create(solutionFileName)
	defer file.Close()

	writer := bufio.NewWriter(file)
	writer.Write([]byte(fmt.Sprintf(scriptTemplate, problemNo, problemNo, problemNo)))
	writer.Flush()
}
