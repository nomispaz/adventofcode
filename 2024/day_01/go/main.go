package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func abs(x int, y int) int {
	if x < y {
		return y-x
	} else {
		return x-y
	}
}

func main()  {
	contents_bytes, _ := os.ReadFile("./input.txt")
	contents := string(contents_bytes)

	// get lines of file
	contents_split := strings.Split(contents, "\n")

	var col1 []int
	var col2 []int

	// remove last element since it is empty
	contents_split = contents_split[0:len(contents_split)-1]

	for _, line := range contents_split {
		line_split := strings.Split(line, "   ")
		convert_to_int, _ := strconv.Atoi(line_split[0])
		col1 = append(col1, convert_to_int)
		convert_to_int, _ = strconv.Atoi(line_split[1])
		col2 = append(col2, convert_to_int)
	}
	
	sort.Ints(col1)
	sort.Ints(col2)

	sum := 0
	for i := 0; i<=len(col1)-1; i++{
		sum += abs(col1[i],col2[i])
	}
	fmt.Print(sum)
}
