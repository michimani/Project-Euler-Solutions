package util

import "math"

func IsSquareOfNatureNumber(square int) bool {
	if square <= 0 {
		return false
	}

	return square == int(math.Pow(math.Floor(math.Pow(float64(square), 0.5)), 2))
}

// Divisors returns divisors of the number.
func Divisors(num int) []int {
	if num <= 0 {
		return []int{}
	}
	if num == 1 {
		return []int{1}
	}

	var divisors []int = []int{}
	var quot int
	for n := 1; n < num; n++ {
		if num%n == 0 {
			quot = num / n
			if quot < n {
				break
			}

			divisors = append(divisors, n)
			if quot != n {
				divisors = append(divisors, quot)
			}
		}
	}

	return divisors
}

// ProperDivisors returns prop divisors of the number.
func ProperDivisors(num int) []int {
	if num <= 1 {
		return []int{}
	}

	var propDivisors []int = []int{}
	var quot int
	for n := 1; n < num; n++ {
		if num%n == 0 {
			quot = num / n
			if quot < n {
				break
			}

			if n != num {
				propDivisors = append(propDivisors, n)
			}
			if quot != n && quot != num {
				propDivisors = append(propDivisors, quot)
			}
		}
	}

	return propDivisors
}

func Factorial(num int) uint64 {
	if num < 0 {
		return 0
	}

	var a uint64 = 1
	for n := num; n > 0; n-- {
		a = a * uint64(n)
	}

	return a
}
