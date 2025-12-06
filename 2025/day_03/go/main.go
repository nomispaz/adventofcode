package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func read_file_to_string(inputfile string) []string {
	contents, err := os.ReadFile(inputfile)
	contents_string := strings.Split(string(contents), "\n")
	if err != nil {
		panic(fmt.Sprintf("Couldn't read inputfile: %s", inputfile))
	}
	return contents_string
}

func find_battery(sequence string, start_idx int, end_index int) (int, int) {
	max_joltage := 0
	max_joltage_idx := 0
	fmt.Printf("%s, %d, %d\n", sequence[start_idx:end_index], start_idx, end_index)
	for idx, battery := range sequence[start_idx:end_index] {
		if int(battery-'0') > max_joltage {
			max_joltage = int(battery - '0')
			max_joltage_idx = idx
		}
	}
	return max_joltage, max_joltage_idx + start_idx
}

func main() {
	contents := read_file_to_string("../testinput.input")

	// part 1
	// number_batteries := 2
	// part 2
	number_batteries := 12
	total_output := 0
	for _, line := range contents {
		var battery_array []int
		combined_joltage := ""
		battery_value := 0
		battery_idx := -1

		length := len(line)
		for idx := range number_batteries {
			battery_value, battery_idx = find_battery(line, battery_idx+1, (length - (number_batteries - 1 - idx)))
			battery_array = append(battery_array, battery_value)
		}
		for _, battery := range battery_array {
			combined_joltage += strconv.Itoa(battery)
		}
		combined_joltage_int, err := strconv.Atoi(combined_joltage)
		if err != nil {
			panic("Couldn't convert string to int")
		}
		total_output += combined_joltage_int
		fmt.Printf("%d, %d, %s\n", combined_joltage_int, total_output, line)
	}
	fmt.Println(total_output)

}
