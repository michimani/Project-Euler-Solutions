package util_test

import (
	"testing"

	"github.com/michimani/project-euler-go/util"
	"github.com/stretchr/testify/assert"
)

func TestIsPalindrome(t *testing.T) {
	cases := []struct {
		name    string
		num     int
		expect  bool
		wantErr bool
	}{
		{
			name:    "true 1",
			num:     12321,
			expect:  true,
			wantErr: false,
		},
		{
			name:    "true 2",
			num:     1221,
			expect:  true,
			wantErr: false,
		},
		{
			name:    "true 3",
			num:     33,
			expect:  true,
			wantErr: false,
		},
		{
			name:    "false 1",
			num:     12345,
			expect:  false,
			wantErr: false,
		},
		{
			name:    "false 2",
			num:     1222,
			expect:  false,
			wantErr: false,
		},
		{
			name:    "false 3",
			num:     31,
			expect:  false,
			wantErr: false,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			isP, err := util.IsPalindrome(c.num)

			if c.wantErr {
				assert.Error(tt, err)
				return
			}

			assert.NoError(tt, err)
			assert.Equal(tt, c.expect, isP)
		})
	}
}

func TestSliceToNumber(t *testing.T) {
	cases := []struct {
		name    string
		nums    []int
		expect  int
		wantErr bool
	}{
		{
			name:    "ok 1",
			nums:    []int{1},
			expect:  1,
			wantErr: false,
		},
		{
			name:    "ok 2",
			nums:    []int{1, 2, 3, 4},
			expect:  1234,
			wantErr: false,
		},
		{
			name:    "ok 3",
			nums:    []int{0, 0, 2, 3},
			expect:  23,
			wantErr: false,
		},
		{
			name:    "ng 1",
			nums:    []int{-1},
			expect:  0,
			wantErr: true,
		},
		{
			name:    "ng 2",
			nums:    []int{1, 2, 3, -4},
			expect:  0,
			wantErr: true,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(tt *testing.T) {
			num, err := util.SliceToNumber(c.nums)

			if c.wantErr {
				assert.Error(tt, err)
				return
			}

			assert.NoError(tt, err)
			assert.Equal(tt, c.expect, num)
		})
	}
}
