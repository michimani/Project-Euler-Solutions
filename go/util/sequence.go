package util

import (
	"strconv"
	"strings"
)

// IsPalindrome is function to check a number is palindrome.
// ex)
//   12321 ... true
//   22222 ... true
//   12222 ... false
func IsPalindrome(num int) (bool, error) {
	var isP bool

	numList := strings.Split(strconv.Itoa(num), "")
	var reverse []string = []string{}
	for n := len(numList) - 1; n >= 0; n-- {
		reverse = append(reverse, numList[n])
	}
	reverseNum, err := strconv.Atoi(strings.Join(reverse, ""))
	if err != nil {
		return false, err
	}

	if reverseNum == num {
		isP = true
	}

	return isP, nil
}
