class Solution:
    def reverse(self, x: int) -> int:
        reverse_n = 0
        sign = -1 if x < 0 else 1
        x *= sign
        
        while x:
            reverse_n = reverse_n * 10 + x % 10
            x //= 10
            
        return 0 if reverse_n < -2**31 or reverse_n > 2**31 - 1 else sign * reverse_n
