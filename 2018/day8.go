package main

import (
	"strconv"
)

func day8_A(nodelist []int) int {
	var idx int
	return day8_reduce(nodelist, &idx)
}

func day8_B(nodelist []int) int {
	var idx int
	return day8_value(nodelist, &idx)
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

func day8() (string, string) {
	values := readInts("input/08")
	a := day8_A(values)
	b := day8_B(values)
	return strconv.Itoa(a), strconv.Itoa(b)
}
