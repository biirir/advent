package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

func part1(nums []int) {
	freq := 0
	for _, n := range nums {
		freq += n
	}
	fmt.Printf("Result part 1: %d\n", freq)
}

func part2(nums []int) {
	freq := 0
	seen := map[int]bool{
		0: true,
	}
	for i := 0; true; i++ {
		freq += nums[i%len(nums)]
		if seen[freq] {
			fmt.Printf("Result part 2: %d\n", freq)
			break
		} else {
			seen[freq] = true
		}
	}
}

func main() {
	f, err := os.Open("input")
	check(err)
	defer f.Close()

	nums := make([]int, 0)
	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		n, _ := strconv.Atoi(scanner.Text())
		nums = append(nums, n)
	}

	check(scanner.Err())
	part1(nums)
	part2(nums)
}
