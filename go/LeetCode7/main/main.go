package main

import (
	"fmt"
	"math"
)

/*
*
[7]整数反转
*/
func main() {
	fmt.Println(reverse(22221111131319))
}

func reverse(x int) int {
	var temp int
	var res int
	for x != 0 {
		temp = x % 10
		x = x / 10
		// 由于x为int类型，限制temp的最大值只能是2，所以可以不用判断temp
		if res > math.MaxInt32/10 || res < math.MinInt32/10 {
			return 0
		}
		res = res*10 + temp
	}
	return res
}
