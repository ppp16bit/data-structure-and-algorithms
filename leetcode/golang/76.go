package main

import (
	"math"
)

func minWindow(s string, t string) string {
	count := [128]int{}
	required := len(t)
	var bestLeft int = -1
	minLength := math.MaxInt32

	for _, chartT := range t {
		count[chartT]++
	}

	for left, right := 0, 0; right < len(s); right++ {
		charRight := rune(s[right])
		count[charRight]--

		if count[charRight] >= 0 {
			required--
		}

		for required == 0 {
			currentWindowLength := right - left + 1

			if currentWindowLength < minLength {
				minLength = currentWindowLength
				bestLeft = left
			}
			charLeft := rune(s[left])

			count[charLeft]++

			if count[charLeft] > 0 {
				required++
			}
			left++
		}
	}

	if bestLeft == -1 {
		return ""
	}

	return s[bestLeft : bestLeft+minLength]
}

/*func main() {
	s1 := "ADOBECODEBANC"
	t1 := "ABC"

	result1 := minWindow(s1, t1)
	s2 := "a"
	t2 := "a"
	result2 := minWindow(s2, t2)

	s3 := "a"
	t3 := "aa"
	result3 := minWindow(s3, t3)

	fmt.Printf("Input: s=\"%s\", t=\"%s\" -> Output: \"%s\"\n", s1, t1, result1)
	fmt.Printf("Input: s=\"%s\", t=\"%s\" -> Output: \"%s\"\n", s2, t2, result2)
	fmt.Printf("Input: s=\"%s\", t=\"%s\" -> Output: \"%s\"\n", s3, t3, result3)
}*/
