package main

import (
	"bytes"
	"io/ioutil"
	"strconv"
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

func day5_A(polymer []byte) int {
	return polymer_reduce(polymer)
}

func day5_B(polymer []byte) int {
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

	return minCount
}

func day5() (string, string) {
	polymer, err := ioutil.ReadFile("input/05")
	check(err)
	polymer = bytes.TrimSpace(polymer)

	a := day5_A(polymer)
	b := day5_B(polymer)
	return strconv.Itoa(a), strconv.Itoa(b)
}
