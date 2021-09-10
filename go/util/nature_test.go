package util_test

import (
	"testing"

	"github.com/michimani/project-euler-go/util"
	"github.com/stretchr/testify/assert"
)

func TestIsSquareOfNatureNumber(t *testing.T) {
	cases := []struct {
		name   string
		square int
		expect bool
	}{
		{
			name:   "true 1",
			square: 1,
			expect: true,
		},
		{
			name:   "true 2",
			square: 4,
			expect: true,
		},
		{
			name:   "true 3",
			square: 144,
			expect: true,
		},
		{
			name:   "false 1",
			square: -1,
			expect: false,
		},
		{
			name:   "false 2",
			square: 0,
			expect: false,
		},
		{
			name:   "false 3",
			square: 5,
			expect: false,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			b := util.IsSquareOfNatureNumber(c.square)

			assert.Equal(tt, c.expect, b)
		})
	}
}

func TestDivisors(t *testing.T) {
	cases := []struct {
		name   string
		num    int
		expect []int
	}{
		{
			name:   "ok 1",
			num:    1,
			expect: []int{1},
		},
		{
			name:   "ok 2",
			num:    12,
			expect: []int{1, 2, 3, 4, 6, 12},
		},
		{
			name:   "ok 3",
			num:    9,
			expect: []int{1, 3, 9},
		},
		{
			name:   "ng 1",
			num:    0,
			expect: []int{},
		},
		{
			name:   "ng 2",
			num:    -100,
			expect: []int{},
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			divs := util.Divisors(c.num)
			assert.Equal(tt, len(c.expect), len(divs))
			for _, e := range c.expect {
				exists := false
				for _, d := range divs {
					if e == d {
						exists = true
						break
					}
				}
				assert.True(tt, exists)
			}
		})
	}
}

func TestPropDivisors(t *testing.T) {
	cases := []struct {
		name   string
		num    int
		expect []int
	}{
		{
			name:   "ok 1",
			num:    8,
			expect: []int{1, 2, 4},
		},
		{
			name:   "ok 2",
			num:    12,
			expect: []int{1, 2, 3, 4, 6},
		},
		{
			name:   "ok 3",
			num:    9,
			expect: []int{1, 3},
		},
		{
			name:   "ng 1",
			num:    1,
			expect: []int{},
		},
		{
			name:   "ng 2",
			num:    0,
			expect: []int{},
		},
		{
			name:   "ng 3",
			num:    -100,
			expect: []int{},
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			divs := util.ProperDivisors(c.num)
			assert.Equal(tt, len(c.expect), len(divs))
			for _, e := range c.expect {
				exists := false
				for _, d := range divs {
					if e == d {
						exists = true
						break
					}
				}
				assert.True(tt, exists)
			}
		})
	}
}

func TestFactorial(t *testing.T) {
	cases := []struct {
		name   string
		num    int
		expect uint64
	}{
		{
			name:   "ok 1",
			num:    0,
			expect: 1,
		},
		{
			name:   "ok 2",
			num:    1,
			expect: 1,
		},
		{
			name:   "ok 3",
			num:    3,
			expect: 6,
		},
		{
			name:   "ok 4",
			num:    8,
			expect: 40320,
		},
		{
			name:   "ng 1",
			num:    -1,
			expect: 0,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			f := util.Factorial(c.num)
			assert.Equal(tt, c.expect, f)
		})
	}
}
