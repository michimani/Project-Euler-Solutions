//
// Solution for Project Euler problem 27
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=27
//
package solution

import (
	"fmt"

	"github.com/michimani/project-euler-go/util"
)

type tmpAnswer struct {
	primes int
	a      int
	b      int
}

func (ta tmpAnswer) isSmaller(other tmpAnswer) bool {
	return ta.primes < other.primes
}

var primes map[int]struct{} = map[int]struct{}{}

// Solve0027 is function to solve the problem.
func Solve0027() {
	var answer int
	ta := tmpAnswer{}

	for a := -999; a < 1000; a++ {
		for b := -1000; b <= 1000; b++ {
			tmpA := calc(a, b)
			if ta.isSmaller(tmpA) {
				ta = tmpA
			}
		}
	}

	answer = ta.a * ta.b

	fmt.Println(answer)
}

func calc(a, b int) tmpAnswer {
	ta := tmpAnswer{a: a, b: b}
	n := 0
	isPrime := true
	primeCnt := 0
	for isPrime {
		res := n*n + a*n + b
		if res > 0 {
			isPrime = util.IsPrime(res, primes)
			if isPrime {
				primes[res] = struct{}{}
				primeCnt++
			}
		}
		n++
	}

	ta.primes = primeCnt
	return ta
}
