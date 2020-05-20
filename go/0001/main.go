//
// Solution for Project Euler problem 1
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=1
//
package main

import (
	"fmt"
	"strconv"
)

// Compute is function to solve the problem.
func Compute() string {
	var answer string
	var answerInt int = 0
	for num := 1; num < 1000; num++ {
		if num%3 == 0 || num%5 == 0 {
			answerInt = answerInt + num
		}
	}

	answer = strconv.Itoa(answerInt)

	return answer
}

func main() {
	fmt.Println(Compute())
}
