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
