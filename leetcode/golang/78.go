package main

func subsets(nums []int) [][]int {
	var ans [][]int
	dfs(nums, 0, []int{}, &ans)
	return ans
}

func dfs(nums []int, start int, path []int, ans *[][]int) {
	currentPath := make([]int, len(path))
	copy(currentPath, path)
	*ans = append(*ans, currentPath)

	for i := start; i < len(nums); i++ {
		path = append(path, nums[i])

		dfs(nums, i+1, path, ans)
		path = path[:len(path)-1]
	}
}

/*func main() {
	nums1 := []int{1, 2, 3}
	nums2 := []int{}
	fmt.Printf("%v\n", subsets(nums1))
	fmt.Printf("%v\n", subsets(nums2))
}*/
