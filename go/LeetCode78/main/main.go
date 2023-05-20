package main

import (
	"fmt"
)

/*
*
[78]子集
*/
func main() {
	fmt.Println(subsets([]int{1, 2, 3}))
}

func subsets(nums []int) [][]int {
	var result [][]int
	var trace []int
	var traceBack func(i int)
	traceBack = func(i int) {
		temp := make([]int, len(trace))
		copy(temp, trace)
		result = append(result, temp)
		if i > len(nums)-1 {
			return
		}
		for index := i; index < len(nums); index++ {
			trace = append(trace, nums[index])
			traceBack(index + 1)
			trace = trace[:len(trace)-1]
		}
	}
	traceBack(0)
	return result
}
