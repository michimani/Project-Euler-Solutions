//
// Solution for Project Euler problem 22
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=22
//
package solution

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

const nameFile = "./solution/0022_names.txt"

var fix = 1 - int([]rune("A")[0])

// Solve0022 is function to solve the problem.
func Solve0022() {
	var answer int

	data, err := ioutil.ReadFile(nameFile)
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	names := string(data)
	names = names[1 : len(names)-1]
	nameList := strings.Split(names, "\",\"")
	sort.Slice(nameList, func(i, j int) bool { return nameList[i] < nameList[j] })

	for i, name := range nameList {
		lineScore := i + 1
		nameScore := calcNameScore(name)
		answer = answer + lineScore*nameScore
	}

	fmt.Println(answer)
}

func calcNameScore(name string) int {
	score := 0
	nameUpper := strings.ToUpper(name)
	for _, r := range nameUpper {
		score = score + int(r) + fix
	}

	return score
}
