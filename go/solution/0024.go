//
// Solution for Project Euler problem 24
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=24
//
package solution

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/michimani/project-euler-go/util"
)

const TARGET uint64 = 1000000

var NUMS []int = []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}

// Solve0024 is function to solve the problem.
func Solve0024() {
	var answer string
	var nums = NUMS
	var answerSequence []string = []string{}
	var selected map[int]struct{} = map[int]struct{}{}
	var now uint64 = 0
	var digit int

	for pos := 1; pos <= len(NUMS); pos++ {
		patternCount := util.Factorial(len(NUMS) - pos)
		for idx, n := range nums {
			if _, ok := selected[n]; ok {
				continue
			}

			if now+patternCount >= TARGET {
				digit = nums[idx]

				answerSequence = append(answerSequence, strconv.Itoa(digit))
				selected[digit] = struct{}{}
				break
			}
			now = now + patternCount
		}
	}

	answer = strings.Join(answerSequence, "")

	fmt.Println(answer)
}
