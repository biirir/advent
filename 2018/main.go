package main

import (
	"log"
	"os"
	"strconv"
)

type DayFunc func()

var (
	dayfunc = [...]DayFunc{day1}
)

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
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
