package main

import "fmt"

//leetcode submit region begin(Prohibit modification and deletion)
type NumMatrix struct {
	sum [][]int
	m   int
	n   int
}

func Constructor(matrix [][]int) NumMatrix {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return NumMatrix{sum: matrix}
	}
	m := len(matrix) + 1
	n := len(matrix[0]) + 1
	f := make([][]int, m)
	for i := 0; i < len(f); i++ {
		f[i] = make([]int, n)
	}
	sumMatrix := NumMatrix{
		sum: f,
		m:   m,
		n:   n,
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			sumMatrix.sum[i][j] = matrix[i-1][j-1] + sumMatrix.sum[i-1][j] + sumMatrix.sum[i][j-1] - sumMatrix.sum[i-1][j-1]
		}
	}
	return sumMatrix
}

func (this *NumMatrix) SumRegion(row1 int, col1 int, row2 int, col2 int) int {
	return this.sum[row2+1][col2+1] - this.sum[row2+1][col1] - this.sum[row1][col2+1] + this.sum[row1][col1]
}

func main() {
	constructor := Constructor([][]int{{1, 1, 1}, {1, 1, 1}})
	fmt.Println(constructor.SumRegion(0, 0, 1, 1))
}
