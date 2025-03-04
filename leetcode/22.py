class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        ans = []
        
        def dfs(left: int, right: int, s: list[str]) -> None:
            if left == 0 and right == 0:
                ans.append(''.join(s))
            if left > 0:
                s.append('(')
                dfs(left - 1, right, s)
                s.pop()
            if left < right:
                s.append(')')
                dfs(left, right - 1, s)
                s.pop()
        dfs(n, n, [])
        return ans