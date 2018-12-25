package main

import (
	"fmt"
	"math"
	"strconv"
)

type coord struct {
	x, y int
}

func manhattan_dist(a, b coord) int {
	return abs(a.x-b.x) + abs(a.y-b.y)
}

func day6_A(points []coord, x, y int) int {
	// Compute the required map: for each point P, store the number of
	// locations which have P as their closest point. This is done by
	// visiting each location and calculating the distance with every
	// single point (!).
	areas := make(map[coord]int)
	killed := make(map[coord]bool)

	for i := 0; i <= x; i++ {
		for j := 0; j <= y; j++ {
			cur := coord{i, j}
			best := math.MaxInt32
			tied := false
			var top coord
			for _, c := range points {
				if d := manhattan_dist(c, cur); d < best {
					top = c
					best = d
					tied = false
				} else if d == best {
					tied = true
				}
			}
			// These locations are in the edge, whichever points they're
			// closest to will have infinite closest locations. Kill those.
			if i == 0 || j == 0 || i == x || j == y {
				killed[top] = true
			}
			if !tied {
				areas[top] += 1
			}
		}
	}

	// Return point who's most closest location.
	maxVal := 0

	for k, v := range areas {
		if v > maxVal && !killed[k] {
			maxVal = v
		}
	}

	return maxVal
}

func day6_B(points []coord, x, y int) int {
	//
	// Use a grid to store distances.
	//
	grid := make([][]int, x+1)

	for i := 0; i <= x; i++ {
		grid[i] = make([]int, y+1)
	}

	//
	// Compute.
	//
	for i := 0; i <= x; i++ {
		for j := 0; j <= y; j++ {
			cur := coord{i, j}
			dist := 0
			for _, c := range points {
				dist += manhattan_dist(c, cur)
			}
			grid[i][j] = dist
		}
	}

	//
	// Filter.
	//
	tot := 0

	for row := range grid {
		for _, val := range grid[row] {
			if val < 10000 {
				tot += 1
			}
		}
	}

	return tot
}

func day6() (string, string) {
	lines := readLines("input/06")
	points := make([]coord, 0, len(lines))

	for _, l := range lines {
		var c coord
		fmt.Sscanf(l, "%d, %d", &c.x, &c.y)
		points = append(points, c)
	}

	// Start by finding the bottom-rightmost point.
	x := 0
	y := 0
	for _, c := range points {
		x = max(x, c.x)
		y = max(y, c.y)
	}

	a := day6_A(points, x, y)
	b := day6_B(points, x, y)
	return strconv.Itoa(a), strconv.Itoa(b)
}
