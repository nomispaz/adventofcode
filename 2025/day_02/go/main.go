package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func read_file_to_string(inputfile string) []string {
	contents, err := os.ReadFile(inputfile)
	contents_string := strings.Split(string(contents), ",")
	if err != nil {
		panic(fmt.Sprintf("Couldn't read inputfile: %s", inputfile))
	}
	return contents_string
}

func check_repetitions_brute_force(start string, end string, repetitions int) int {
	start_int, err := strconv.Atoi(start)
	if err != nil {
		panic("Couldn't convert str to int")
	}
	end_int, err := strconv.Atoi(end)
	if err != nil {
		panic("Couldn't convert str to int")
	}

	for idx := range end_int - start_int + 1 {
		current_int := start_int + idx
		current := strconv.Itoa(current_int)
		length := len(current)
		first_part := current[:length/2]
		first_part_int, _ := strconv.Atoi(first_part)
		second_part := current[length/2:]
		second_part_int, _ := strconv.Atoi(second_part)
		if first_part_int == second_part_int && length%2 == 0 {
			repetitions += current_int
		}

	}
	return repetitions
}

func match_id(regex string, text string) bool {
	// ^: start of string
	// (?:   ) exact substring
	// +: one or more repetitions
	// $: end of string
	match, _ := regexp.MatchString(fmt.Sprintf("^(?:%s)+$", regex), text)
	return match
}

func check_repetitions_brute_force_regex(start string, end string, part int, repetitions int) int {
	start_int, err := strconv.Atoi(start)
	if err != nil {
		panic("Couldn't convert str to int")
	}
	end_int, err := strconv.Atoi(end)
	if err != nil {
		panic("Couldn't convert str to int")
	}

	// loop throuth all numbers in interval
	for idx := range end_int - start_int + 1 {
		current_int := start_int + idx
		current := strconv.Itoa(current_int)
		length := len(current)

		// part 1: only check for half of the string
		// part 2: check all substrings of the string as long as smaller or equal half of the length
		switch part {
		case 1:
			if length%2 == 0 {
				if match_id(current[:length/2], current) {
					repetitions += current_int
				}
			}
		case 2:
			for cur_length := range length/2 + 1 {
				//fmt.Println(fmt.Sprintf("Checking: %s, %d, %d", current, cur_length, length))
				if match_id(current[:cur_length], current) {
					//fmt.Println(fmt.Sprintf("Found: %s", current))
					repetitions += current_int
					break
				}
			}
		}
	}

	return repetitions
}

func main() {
	filecontents := read_file_to_string("../input.input")
	repetitions := 0
	//repetitions_2 := 0
	for _, entry := range filecontents {
		println(fmt.Sprintf("New interval: %s", entry))
		start_end := strings.Split(entry, "-")
		//repetitions = check_repetitions_brute_force(start_end[0], start_end[1], repetitions)
		repetitions = check_repetitions_brute_force_regex(start_end[0], start_end[1], 2, repetitions)
	}
	println(repetitions)
}
