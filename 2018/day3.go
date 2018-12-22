package main

import (
	"fmt"
)

type claim struct {
	id, x, y, w, h int
}

type cell struct {
	x, y int
}

func day3_claim(c *claim, cells map[cell]int) {
	for i := 0; i < c.w; i++ {
		for j := 0; j < c.h; j++ {
			cell := cell{c.x + i, c.y + j}
			cells[cell] += 1
		}
	}
}

func day3_A(claims []claim) {
	cells := make(map[cell]int)
	overlap := 0

	for i := range claims {
		day3_claim(&claims[i], cells)
	}

	for _, v := range cells {
		if v > 1 {
			overlap++
		}
	}
	fmt.Printf("Day 3, part 1: %d\n", overlap)

	day3_B(claims, cells)
}

func day3_B(claims []claim, cells map[cell]int) {
outer:
	for i := range claims {
		c := &claims[i]
		for i := 0; i < c.w; i++ {
			for j := 0; j < c.h; j++ {
				cell := cell{c.x + i, c.y + j}
				if cells[cell] > 1 {
					continue outer
				}
			}
		}
		fmt.Printf("Day 3, part 2: %d\n", c.id)
		break
	}
}

func day3() {
	lines := readLines("input/03")
	claims := make([]claim, 0, len(lines))

	for _, l := range lines {
		var p claim
		_, err := fmt.Sscanf(l, "#%d @ %d,%d: %dx%d", &p.id, &p.x, &p.y, &p.w, &p.h)
		check(err)
		claims = append(claims, p)
	}

	day3_A(claims) // Calls day3_B() with intermediate state.
}
