package main

import (
	"container/ring"
	"fmt"
)

func makeRing(marble int) *ring.Ring {
	r := ring.New(1)
	r.Value = marble
	return r
}

func day9_score(players, lastval int) int {
	var n *ring.Ring
	p := 0
	r := makeRing(0)
	scores := make([]int, players)

	for i := 1; i <= lastval; i++ {
		if i%23 != 0 {
			n = r.Next()
			r = makeRing(i)
			n.Link(r)
		} else {
			r = r.Move(-8) // 7+1 because x.Unlink() starts at x.Next()
			n = r.Unlink(1)
			r = r.Next()
			scores[p] += i + n.Value.(int)
		}
		p = (p + 1) % players
	}

	max := 0

	for _, v := range scores {
		if v > max {
			max = v
		}
	}

	return max
}

func day9() {
	lines := readLines("input/09")
	var players, lastval int
	fmt.Sscanf(
		lines[0], "%d players; last marble is worth %d points", &players, &lastval)

	fmt.Println("Day 9, part 1:", day9_score(players, lastval))
	fmt.Println("Day 9, part 2:", day9_score(players, lastval*100))
}
