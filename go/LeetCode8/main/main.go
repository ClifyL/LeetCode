package main

import (
	"fmt"
	"math"
)

/*
*
[8]字符串转整数
*/
func main() {
	fmt.Println(myAtoi("2147483648"))
}

func myAtoi(s string) int {
	var result int
	var factor int
	var start int
	for start < len(s) {
		if s[start] != 32 {
			break
		}
		start++
	}
	for i := start; i < len(s); i++ {
		if s[i] == 45 && factor == 0 {
			factor = -1
			continue
		} else if s[i] == 43 && factor == 0 {
			factor = 1
			continue
		}
		if s[i] > 47 && s[i] < 58 {
			if factor == 0 {
				factor = 1
			}
			result = result*10 + int(s[i]-48)*factor
			if result > math.MaxInt32 {
				return math.MaxInt32
			}
			if result < math.MinInt32 {
				return math.MinInt32
			}
		} else {
			break
		}
	}
	return result
}
