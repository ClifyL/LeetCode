package main

import (
	"fmt"
)

func main() {

	fmt.Println(findMedianSortedArrays([]int{1, 2, 3, 4}, []int{1, 2, 3, 4}))
}

/*
*
[4]寻找中位数
*/
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	var (
		pointOne = 0
		pointTwo = 0
	)

	list := make([]int, len(nums1)+len(nums2))
	for i := 0; i < len(nums1)+len(nums2); i++ {

		if pointOne >= len(nums1) {
			list[i] = nums2[pointTwo]
			pointTwo++
			continue
		}
		if pointTwo >= len(nums2) {
			list[i] = nums1[pointOne]
			pointOne++
			continue
		}
		if nums1[pointOne] < nums2[pointTwo] {
			list[i] = nums1[pointOne]
			pointOne++
		} else {
			list[i] = nums2[pointTwo]
			pointTwo++
		}
	}
	return float64(list[int((len(nums1)+len(nums2)+1)/2)-1]+list[int((len(nums1)+len(nums2)+2)/2)-1]) / 2
}
