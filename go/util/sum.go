package util

func Sum(nums []int) int {
	s := 0
	for _, n := range nums {
		s = s + n
	}

	return s
}
