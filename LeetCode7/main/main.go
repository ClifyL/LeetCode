package main

import (
	"fmt"
	"math"
	"strconv"
)

/*
*
[6]整数反转
*/
func main() {
	fmt.Println(reverse(-130))
}

func reverse(x int) int {
	if x == 0 {
		return 0
	}
	num := strconv.Itoa(x)
	var str string
	end := len(num) - 1
	for {
		if num[end] != 48 {
			break
		}
		end--
	}
	for i := end; i >= 0; i-- {
		if num[i] != 45 {
			str += string(num[i])
		}
	}
	result, _ := strconv.Atoi(str)
	if result > math.MaxInt32 || result < math.MinInt32 {
		return 0
	} else if num[0] == 45 {
		return -result
	} else {
		return result
	}
}
