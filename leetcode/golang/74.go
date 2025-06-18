package main

func searchMatrix(matrix [][]int, target int) bool {
	if matrix == nil || len(matrix) == 0 || len(matrix[0]) == 0 {
		return false
	}

	m := len(matrix)
	n := len(matrix[0])
	var left int = 0
	var right int = m * n

	for left < right {
		mid := (left + right) / 2
		i := mid / n
		j := mid % n

		if matrix[i][j] == target {
			return true
		}

		if matrix[i][j] < target {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return false
}

/*func main() {
	m1 := [][]int{
		{1, 3, 5, 7},
		{10, 11, 16, 20},
		{23, 30, 34, 60},
	}
	t1 := 3

	m2 := [][]int{
		{1, 3, 5, 7},
		{10, 11, 16, 20},
		{23, 30, 34, 60},
	}
	t2 := 13

	found1 := searchMatrix(m1, t1)
	found2 := searchMatrix(m2, t2)
	fmt.Println(found1, found2)

}*/
