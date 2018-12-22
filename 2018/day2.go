package main

import (
	"fmt"
)

// Returns two values:
//   - number of letters that appear exactly twice in ‘s’
//   - number of letters that appear exactly thrice in ‘s’
func dupecounts(s string) (int, int) {
	two := 0
	three := 0
	counts := make(map[rune]int)

	for _, c := range s {
		counts[c] += 1
	}

	for _, v := range counts {
		if v == 2 {
			two++
		} else if v == 3 {
			three++
		}
	}

	return two, three
}

func day2_A(lines []string) {
	mul1 := 0
	mul2 := 0
	for _, s := range lines {
		two, three := dupecounts(s)
		if two > 0 {
			mul1++
		}
		if three > 0 {
			mul2++
		}
	}
	fmt.Printf("Day 2, part 1: %d\n", mul1*mul2)
}

func day2_B(lines []string) {
	//
	// Let’s try bruteforce!
	//
	for i := range lines {
		for j := i + 1; j < len(lines); j++ {
			if s := day2_match(lines[i], lines[j]); s != "" {
				fmt.Printf("Day 2, part 2: %s\n", s)
				return
			}
		}
	}
}

func day2_match(a, b string) string {
	//
	// Moar bruteforce.
	//
	pos := -1

	if len(a) != len(b) {
		return ""
	}

	for i := range a {
		if a[i] == b[i] {
			continue
		}
		if pos < 0 {
			pos = i
		} else {
			return ""
		}
	}

	return a[:pos] + b[pos+1:]
}

func day2() {
	lines := readLines("input/02")
	day2_A(lines)
	day2_B(lines)
}
