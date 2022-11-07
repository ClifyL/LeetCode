package main

import "fmt"

type NumArray struct {
	sum []int
}

/*
*
[303]区域和检索- 数组不可变
*/
func main() {
	input := []int{1, 2, 3, 1, 4}
	obj := Constructor(input)
	fmt.Println(obj.sum)
	fmt.Println(obj.SumRange(1, 2))
}

func Constructor(nums []int) NumArray {
	obj := NumArray{
		sum: make([]int, len(nums)+1),
	}
	obj.sum[0] = 0
	for i := 0; i < len(nums); i++ {
		obj.sum[i+1] = obj.sum[i] + nums[i]
	}
	return obj
}

func (this *NumArray) SumRange(left int, right int) int {
	return this.sum[right+1] - this.sum[left]
}
