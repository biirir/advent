package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

type DayFunc func()

var (
	dayfunc = [...]DayFunc{day1, day2, day3, day4, day5, day6}
)

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

func abs(x int) int {
	if x >= 0 {
		return x
	}
	return -x
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func readLines(filename string) []string {
	f, err := os.Open(filename)
	check(err)
	defer f.Close()

	lines := make([]string, 0)
	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	check(scanner.Err())
	return lines
}

func main() {
	day := len(dayfunc)

	if len(os.Args) > 1 {
		var err error
		day, err = strconv.Atoi(os.Args[1])
		check(err)
	}

	dayfunc[day-1]()
}
