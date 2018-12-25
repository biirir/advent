package main

import (
	"fmt"
)

func day1_A(nums []int) {
	freq := 0
	for _, n := range nums {
		freq += n
	}
	fmt.Printf("Day 1, part 1: %d\n", freq)
}

func day1_B(nums []int) {
	freq := 0
	seen := map[int]bool{
		0: true,
	}
	for i := 0; true; i++ {
		freq += nums[i%len(nums)]
		if seen[freq] {
			fmt.Printf("Day 1, part 2: %d\n", freq)
			break
		} else {
			seen[freq] = true
		}
	}
}

func day1() {
	nums := readInts("input/01")
	day1_A(nums)
	day1_B(nums)
}
