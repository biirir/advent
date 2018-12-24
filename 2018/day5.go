package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
)

func polymer_match(a, b byte) bool {
	return a != b && a%32 == b%32
}

func polymer_reduce(polymer []byte) int {
	//
	// I'm sad because I seem unable to come up with a better algorithm
	// than brute force. :-(
	//
	count := 0
	match := true

	for match {
		l := -1
		r := -1
		match = false
		for i := range polymer {
			if polymer[i] == 0 {
				continue
			}
			if l < 0 {
				l = i
				continue
			}
			r = i
			a := polymer[l]
			b := polymer[r]
			if polymer_match(a, b) {
				polymer[l] = 0
				polymer[r] = 0
				l = -1
				match = true
			} else {
				l = r
			}
		}
	}

	for _, b := range polymer {
		if b > 0 {
			count++
		}
	}

	return count
}

func day5_A(polymer []byte) {
	fmt.Println("Day 5, part 1:", polymer_reduce(polymer))
}

func day5_B(polymer []byte) {
	done := make([]bool, 27)
	minCount := len(polymer)
	workspace := make([]byte, len(polymer))

	for _, b := range polymer {
		if done[b%32] {
			continue
		}
		done[b%32] = true
		for i, orig := range polymer {
			if b == orig || b%32 == orig%32 {
				workspace[i] = 0
			} else {
				workspace[i] = orig
			}
		}
		minCount = min(minCount, polymer_reduce(workspace))
	}

	fmt.Println("Day 5, part 2:", minCount)
}

func day5() {
	polymer, err := ioutil.ReadFile("input/05")
	check(err)
	polymer = bytes.TrimSpace(polymer)

	day5_A(polymer)
	day5_B(polymer)
}
