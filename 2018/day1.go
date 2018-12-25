package main

import (
	"strconv"
)

func day1_A(nums []int) int {
	freq := 0
	for _, n := range nums {
		freq += n
	}
	return freq
}

func day1_B(nums []int) int {
	freq := 0
	seen := map[int]bool{
		0: true,
	}
	for i := 0; true; i++ {
		freq += nums[i%len(nums)]
		if seen[freq] {
			return freq
		} else {
			seen[freq] = true
		}
	}
	return -1
}

func day1() (string, string) {
	nums := readInts("input/01")
	a := day1_A(nums)
	b := day1_B(nums)
	return strconv.Itoa(a), strconv.Itoa(b)
}
