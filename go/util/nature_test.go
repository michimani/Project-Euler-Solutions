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
