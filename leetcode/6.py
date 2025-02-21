class Solution:
    def convert(self, s: str, numRows: int) -> str:
        rows = [''] * numRows
        currentRow = 0
        direction = (numRows == 1) - 1
        
        for char in s:
            rows[currentRow] += char
            if currentRow == 0 or currentRow == numRows - 1:
                direction *= -1
            currentRow += direction
        
        return ''.join(rows)