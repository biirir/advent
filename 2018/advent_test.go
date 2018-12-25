package main

import (
	"fmt"
	"testing"
)

var (
	myResults = [...][]string{
		{"516", "71892"},
		{"5368", "cvgywxqubnuaefmsljdrpfzyi"},
		{"109785", "504"},
		{"65489", "3852"},
		{"11252", "6118"},
		{"3989", "49715"},
		nil,
		{"48260", "25981"},
		{"436720", "3527845091"},
	}
)

func TestAdvent(t *testing.T) {
	for d, f := range dayfunc {
		t.Run(fmt.Sprintf("Day=%d", d+1), func(t *testing.T) {
			if f == nil {
				t.Skipf("Day %d not implemented", d+1)
			}
			a, b := f()
			if val := myResults[d][0]; a != val {
				t.Errorf("Part 1: got %q, want %q", a, val)
			}
			if val := myResults[d][1]; b != val {
				t.Errorf("Part 2: got %q, want %q", b, val)
			}
		})
	}
}
