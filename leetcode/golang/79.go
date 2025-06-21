package main

import (
	"fmt"
)

func exist(board [][]byte, word string) bool {
	if len(board) == 0 || len(board[0]) == 0 {
		return false
	}

	if len(word) == 0 {
		return true
	}

	rows := len(board)
	cols := len(board[0])

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if board[i][j] == word[0] {
				if dfs(board, word, i, j, 0, rows, cols) {
					return true
				}
			}
		}
	}

	return false
}

func dfs(board [][]byte, word string, i, j, wordIdx, rows, cols int) bool {
	if i < 0 || i >= rows || j < 0 || j >= cols {
		return false
	}

	if board[i][j] != word[wordIdx] || board[i][j] == '*' {
		return false
	}

	if wordIdx == len(word)-1 {
		return true
	}

	originalChar := board[i][j]
	board[i][j] = '*'

	found := dfs(board, word, i+1, j, wordIdx+1, rows, cols) ||
		dfs(board, word, i-1, j, wordIdx+1, rows, cols) ||
		dfs(board, word, i, j+1, wordIdx+1, rows, cols) ||
		dfs(board, word, i, j-1, wordIdx+1, rows, cols)

	board[i][j] = originalChar

	return found
}

func main() {
	board1 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	word1 := "ABCCED"
	fmt.Printf("%t\n", exist(board1, word1))

	board2 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	word2 := "SEE"
	fmt.Printf("%t\n", exist(board2, word2))

	board3 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	word3 := "ABCB"
	fmt.Printf("%t\n", exist(board3, word3))
}
