package main

import (
	"fmt"
)

func day8_A(nodelist []int) {
	var idx int
	fmt.Println("Day 8, part 1", day8_reduce(nodelist, &idx))
}

func day8_B(nodelist []int) {
	var idx int
	fmt.Println("Day 8, part 2", day8_value(nodelist, &idx))
}

func day8_reduce(nodelist []int, idx *int) int {
	children := nodelist[*idx]
	*idx += 1

	metasum := 0
	metacount := nodelist[*idx]
	*idx += 1

	for i := 0; i < children; i++ {
		metasum += day8_reduce(nodelist, idx)
	}

	for i := 0; i < metacount; i++ {
		metasum += nodelist[*idx]
		*idx += 1
	}

	return metasum
}

func day8_value(nodelist []int, idx *int) int {
	children := nodelist[*idx]
	metacount := nodelist[*idx+1]
	nodeValue := 0
	childValue := make([]int, children)
	*idx += 2

	for i := 0; i < children; i++ {
		childValue[i] = day8_value(nodelist, idx)
	}

	for i := 0; i < metacount; i++ {
		v := nodelist[*idx]
		if children == 0 {
			nodeValue += v
		} else if v > 0 && v <= children {
			nodeValue += childValue[v-1]
		}
		*idx += 1
	}

	return nodeValue
}

func day8() {
	values := readInts("input/08")
	day8_A(values)
	day8_B(values)
}
