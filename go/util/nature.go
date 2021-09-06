package util

import "math"

func IsSquareOfNatureNumber(square int) bool {
	if square <= 0 {
		return false
	}

	return square == int(math.Pow(math.Floor(math.Pow(float64(square), 0.5)), 2))
}
