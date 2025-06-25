package main

import "fmt"

func removeDuplicates(nums []int) int {
	if len(nums) <= 2 {
		return len(nums)
	}

	i := 2

	for num := 2; num < len(nums); num++ {
		if nums[num] != nums[i-2] {
			nums[i] = nums[num]
			i++
		}
	}

	return i
}

func main() {
	nums1 := []int{1, 1, 1, 2, 2, 3}
	outp1 := removeDuplicates(nums1)
	fmt.Printf("%v\n", nums1[:outp1])

	nums2 := []int{0, 0, 1, 1, 1, 1, 2, 3, 3}
	outp2 := removeDuplicates(nums2)
	fmt.Printf("%v\n", nums2[:outp2])
}
