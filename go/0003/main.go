//
// Solution for Project Euler problem 3
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=3
//
package main

import (
	"fmt"
)

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var primesMap map[int]int = map[int]int{}
	var isPrime bool = true

	var targetNumber int = 600851475143
	var targetNumberDiv int = targetNumber

	for num := 2; num < targetNumber; num++ {
		isPrime = true
		for _, val := range primesMap {
			if num%val == 0 {
				isPrime = false
				break
			}
		}

		if isPrime == true {
			primesMap[num] = num

			for targetNumberDiv%num == 0 {
				targetNumberDiv = targetNumberDiv / num
			}

			if targetNumberDiv <= 1 {
				answer = num
				break
			}
		}

	}

	return answer
}

func main() {
	fmt.Println(Compute())
}
