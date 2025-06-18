package main

import "fmt"

func sortColors(nums []int) {
	red, white, blue := 0, 0, 0
	idx := 0

	for _, num := range nums {
		if num == 0 {
			red++
		} else if num == 1 {
			white++
		} else {
			blue++
		}
	}

	for i := 0; i < red; i++ {
		nums[idx] = 0
		idx++
	}

	for i := 0; i < white; i++ {
		nums[idx] = 1
		idx++
	}

	for i := 0; i < blue; i++ {
		nums[idx] = 2
		idx++
	}
}

func main() {
	nums := []int{2, 0, 1, 1, 0}

	sortColors(nums)
	fmt.Println(nums)
}
