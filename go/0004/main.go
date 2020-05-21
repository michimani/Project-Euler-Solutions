//
// Solution for Project Euler problem 4
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=4
//
package main

import (
	"fmt"
	"strconv"
	"strings"
)

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var palindomes []int = []int{}

	for a := 100; a < 1000; a++ {
		for b := 100; b < 1000; b++ {
			times := a * b
			isPalindrome := IsPalindrome(times)
			if isPalindrome == true {
				palindomes = append(palindomes, times)
			}
		}
	}

	var palindomeTmp int = 0
	for _, p := range palindomes {
		if p > palindomeTmp {
			palindomeTmp = p
		}
	}

	answer = palindomeTmp

	return answer
}

// IsPalindrome is function to check a number is palindrome.
func IsPalindrome(num int) bool {
	var isPalindrome bool = false

	numList := strings.Split(strconv.Itoa(num), "")
	var reverse []string = []string{}
	for n := len(numList) - 1; n >= 0; n-- {
		reverse = append(reverse, numList[n])
	}
	reverseNum, _ := strconv.Atoi(strings.Join(reverse, ""))

	if reverseNum == num {
		isPalindrome = true
	}

	return isPalindrome
}

func main() {
	fmt.Println(Compute())
}
