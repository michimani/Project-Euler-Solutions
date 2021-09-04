//
// Solution for Project Euler problem 4
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=4
//
package solution

import (
	"fmt"
	"strconv"
	"strings"
)

// Solve0004 is function to solve the problem.
func Solve0004() {
	var answer int
	var palindomes []int = []int{}

	for a := 100; a < 1000; a++ {
		for b := 100; b < 1000; b++ {
			times := a * b
			isP := isPalindrome(times)
			if isP == true {
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

	fmt.Println(answer)
}

// isPalindrome is function to check a number is palindrome.
func isPalindrome(num int) bool {
	var isP bool = false

	numList := strings.Split(strconv.Itoa(num), "")
	var reverse []string = []string{}
	for n := len(numList) - 1; n >= 0; n-- {
		reverse = append(reverse, numList[n])
	}
	reverseNum, _ := strconv.Atoi(strings.Join(reverse, ""))

	if reverseNum == num {
		isP = true
	}

	return isP
}
