//
// Solution for Project Euler problem 14
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=14
//
package solution

import (
	"fmt"
	"math"
)

const MAX_NUMBER = 1000000

type startNumber int
type sequenceLength int

var checked = map[startNumber]sequenceLength{}

// Solve0014 is function to solve the problem.
func Solve0014() {
	var answer int

	for n := 1; n <= MAX_NUMBER; n++ {
		calcCollatzSequence(n)
	}

	tmpLen := 0
	for startNum, seqLen := range checked {
		if tmpLen < int(seqLen) {
			tmpLen = int(seqLen)
			answer = int(startNum)
		}
	}

	fmt.Println(answer)
}

func calcCollatzSequence(startNum int) {
	next := startNum
	length := 1
	for next != 1 {
		if checkedSeq, ok := checked[startNumber(next)]; ok {
			length = length + int(checkedSeq)
			break
		}

		if next%2 == 0 {
			next = int(math.Floor(float64(next) / 2))
		} else {
			next = 3*next + 1
		}
		length++
	}

	checked[startNumber(startNum)] = sequenceLength(length)
}
