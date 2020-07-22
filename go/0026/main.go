//
// Solution for Project Euler problem 26
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=26
//
package main

import (
	"fmt"
	"strconv"
)

var limitNum = 1000

// Compute is function to solve the problem.
func Compute() int {
	var answer int
	var tmpLength int = 0

	for natureNum := 1; natureNum < limitNum; natureNum++ {
		repeatPart := GetRepeatingPart(natureNum)
		if len(repeatPart) > tmpLength {
			answer = natureNum
			tmpLength = len(repeatPart)
		}
	}

	return answer
}

// GetRepeatingPart is function to get repeating part of reciprocal of nature number.
func GetRepeatingPart(natureNum int) string {
	var repeatPart string = ""
	var disits []string = []string{}
	divided := 1
	div := natureNum
	quot := 0
	remain := 0
	var remainMap map[int]int = map[int]int{1: 0}

	for true {
		if divided/div == 0 {
			divided = divided * 10
		}

		remain = divided % div
		quot = divided / div

		disits = append(disits, strconv.Itoa(quot))
		divided = remain

		_, remainExists := remainMap[remain]
		if remain == 0 || remainExists {
			break
		}

		remainMap[remain] = len(remainMap)
	}

	if remain == 0 {
		return repeatPart
	}

	firstAppearIdx := remainMap[remain]
	for idx := firstAppearIdx; idx < len(disits); idx++ {
		repeatPart = repeatPart + disits[idx]
	}
	return repeatPart
}

func main() {
	fmt.Println(Compute())
}
