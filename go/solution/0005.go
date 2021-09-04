//
// Solution for Project Euler problem 5
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=5
//
package solution

import (
	"fmt"
)

// Solve0005 is function to solve the problem.
func Solve0005() {
	var answer int = 1
	var primesMap map[int]int = map[int]int{} // [prime]index
	var primeIndex int
	var numForDiv int
	var isPrime bool
	var max int = 20

	for num := 2; num <= max; num++ {
		isPrime = true
		for prime := range primesMap {
			if num%prime == 0 {
				isPrime = false
				break
			}
		}

		if isPrime {
			primesMap[num] = 1
		} else {
			numForDiv = num
			for primeTmp, index := range primesMap {
				primeIndex = 0
				for numForDiv%primeTmp == 0 {
					numForDiv = numForDiv / primeTmp
					primeIndex++
				}

				if primeIndex > index {
					primesMap[primeTmp] = primeIndex
				}
			}
		}
	}

	for prime, index := range primesMap {
		for t := 0; t < index; t++ {
			answer = answer * prime
		}
	}

	fmt.Println(answer)
}
