package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}


// leetcode submit region begin(Prohibit modification and deletion)
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if left == 1 {
		return reverseN(head, right)
	}
	head.Next = reverseBetween(head.Next, left-1, right-1)
	return head
}

var nextNode *ListNode
func reverseN(head *ListNode, n int) *ListNode {
	if n == 1 {
		nextNode = head.Next
		return head
	}
	last := reverseN(head.Next, n-1)
	head.Next.Next = head
	head.Next = nextNode

	return last
}

func main() {
	head := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 4,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	}
	result := reverseBetween(head, 2, 4)
	for {
		if result == nil {
			break
		}
		fmt.Println(result.Val)
		result = result.Next
	}
}
