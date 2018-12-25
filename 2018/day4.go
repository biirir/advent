package main

import (
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"
)

type Guard map[int]int

func day4() (string, string) {
	lines := readLines("input/04")
	sort.Strings(lines)

	guard := -1
	begin := -1
	totals := make(map[int]int)
	guards := make(map[int]Guard)

	for _, l := range lines {
		var _z, min int
		var _x, action string

		if strings.Contains(l, "Guard") {
			_, err := fmt.Sscanf(l, "%s %s %s #%d", &_x, &_x, &_x, &guard)
			check(err)
			if guards[guard] == nil {
				guards[guard] = make(Guard)
			}
			continue
		}

		_, err := fmt.Sscanf(l, "%s %d:%d] %s", &_x, &_z, &min, &action)
		check(err)

		if action == "falls" {
			begin = min
		} else if action == "wakes" {
			for m := begin; m < min; m++ {
				totals[guard] += 1
				guards[guard][m] += 1
			}
		} else {
			log.Fatalf("unknown action: %s", action)
		}
	}

	// Part A
	topGuard := maxByVal(totals)
	topMinute := maxByVal(guards[topGuard])

	// Part B
	mostGuard := -1
	mostMinute := -1
	minuteFreq := 0

	for guard, dict := range guards {
		minute := maxByVal(dict)
		if dict[minute] > minuteFreq {
			mostGuard = guard
			mostMinute = minute
			minuteFreq = dict[minute]
		}
	}

	// Debug
	// fmt.Printf("               guard=%d minute=%d\n", topGuard, topMinute)
	// fmt.Printf("               guard=%d minute=%d\n", mostGuard, mostMinute)

	return strconv.Itoa(topGuard * topMinute), strconv.Itoa(mostGuard * mostMinute)
}

func maxByVal(mapping map[int]int) int {
	topKey := -1
	maxVal := 0

	for k, v := range mapping {
		if v > maxVal {
			topKey = k
			maxVal = v
		}
	}

	return topKey
}
