package main

import (
	"fmt"
	"strconv"
)

/*
*
[9]回文数
*/
func main() {
	fmt.Println(isPalindrome(123211))
}

func isPalindrome(x int) bool {
	num := strconv.Itoa(x)
	var head = 0
	var tail = len(num) - 1
	for i := 0; i < len(num); i++ {
		if head >= tail {
			return true
		}

		if num[head] == num[tail] {
			head++
			tail--
			continue
		}
		break
	}
	return false
}
