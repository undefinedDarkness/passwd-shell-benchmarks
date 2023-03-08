package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	shells := make(map[string]int)

	file, err := os.Open("passwd")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Split(line, ":")
		shell := fields[len(fields)-1]
		shells[shell]++
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	for shell, count := range shells {
		fmt.Printf("%-25s : %5d\n", shell, count)
	}
}
