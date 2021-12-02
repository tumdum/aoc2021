package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func failOnErr(e error) {
	if e != nil {
		panic(e)
	}
}

func count(s []int) int {
	c := 0
	for i := 1; i != len(s); i++ {
		if s[i-1] < s[i] {
			c += 1
		}
	}
	return c
}

func sum3(s []int) []int {
	ret := []int{}
	for i := 2; i != len(s); i++ {
		ret = append(ret, s[i]+s[i-1]+s[i-2])
	}
	return ret
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	input := []int{}
	for scanner.Scan() {
		n, err := strconv.Atoi(scanner.Text())
		failOnErr(err)
		input = append(input, n)
	}
	fmt.Println(count(input))
	fmt.Println(count(sum3(input)))
}
