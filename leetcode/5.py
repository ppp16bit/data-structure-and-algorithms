class Solution:
    def longestPalindrome(self, s: str) -> str:
        if not s:
            return ""
        
        start = 0
        end = 0
        n = len(s)
        
        
        # no excessive func calls 
        for i in range(n):
            for left, right in [(i, i), (i, i + 1)]:
                while left >= 0 and right < n and s[left] == s[right]:
                    left -= 1
                    right += 1
                
                if right - left - 1 > end - start:
                    start, end = left + 1, right - 1
        return s[start:end + 1]