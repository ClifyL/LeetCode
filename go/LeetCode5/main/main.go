package main

import (
	"fmt"
)

/*
*
[5]最大回文字串
*/
func main() {

	fmt.Println(longestPalindrome("bbbb"))
}

func longestPalindrome(s string) string {
	var (
		start = 0
		end   = 0
		max   = 0
		sp    = 0
		ep    = 0
	)
	for i := 1; i < len(s); i++ {
		sp = i - 1
		if i < len(s)-1 && s[i-1] == s[i+1] {
			sp = i - 1
			ep = i + 1
			for sp >= 0 && ep < len(s) && s[sp] == s[ep] {
				if ep-sp+1 > max {
					max = ep - sp + 1
					start = sp
					end = ep
				}
				ep++
				sp--
			}
		}
		if s[i-1] == s[i] {
			sp = i - 1
			ep = i
			for sp >= 0 && ep < len(s) && s[sp] == s[ep] {
				if ep-sp+1 > max {
					max = ep - sp + 1
					start = sp
					end = ep
				}
				ep++
				sp--
			}
		}

	}
	return s[start : end+1]
}
