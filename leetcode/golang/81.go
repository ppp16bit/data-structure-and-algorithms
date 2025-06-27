package main

func search(nums []int, target int) bool {
	left, right := 0, len(nums)-1

	for left <= right {
		mid := (left + right) / 2

		if nums[mid] == target {
			return true
		}

		if nums[left] == nums[mid] && nums[mid] == nums[right] {
			left++
			right--
		} else if nums[left] <= nums[mid] {
			if nums[left] <= target && target < nums[mid] {
				right = mid - 1
			} else {
				left = mid + 1
			}
		} else {
			if nums[mid] < target && target <= nums[right] {
				left = mid + 1
			} else {
				right = mid - 1
			}
		}
	}

	return false
}

/*func main() {
	nums1 := []int{2, 5, 6, 0, 0, 1, 2}
	nums2 := []int{2, 5, 6, 0, 0, 1, 2}

	t1 := 0
	t2 := 3

	outp1 := search(nums1, t1)
	fmt.Println(outp1)

	outP2 := search(nums2, t2)
	fmt.Println(outP2)
}*/
