package main

import "fmt"

func combine(n int, k int) [][]int {
	var ans [][]int
	dfs(n, k, 1, []int{}, &ans)
	return ans
}

func dfs(n int, k int, start int, path []int, ans *[][]int) {
	if len(path) == k {
		temp := make([]int, len(path))
		copy(temp, path)
		*ans = append(*ans, temp)
		return
	}

	for i := start; i <= n; i++ {
		if k-len(path) > n-i+1 {
			continue
		}

		path = append(path, i)
		dfs(n, k, i+1, path, ans)
		path = path[:len(path)-1]
	}
}

func main() {
	n := 4
	k := 2
	result := combine(n, k)
	fmt.Println(result)

	n2 := 5
	k2 := 3
	result2 := combine(n2, k2)
	fmt.Println(result2)
}

