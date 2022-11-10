package main

import (
	"fmt"
	"strings"
)

/*
*
[6]Z形变换
*/
func main() {
	fmt.Println(convert("PAYPALISHIRING", 1))
}

func convert(s string, numRows int) string {
	if numRows < 2 {
		return s
	}
	list := make([]string, numRows)
	index := 0
	factor := -1
	for i := 0; i < len(s); i++ {
		if index == numRows-1 || index == 0 {
			factor = -factor
		}
		list[index] += string(s[i])
		index += factor
	}
	return strings.Join(list, "")
}
