class Solution:
    def isValid(self, s: str) -> bool:
        stk = []
        
        for c in s:
            if c == '(':
                stk.append(')')
            elif c == '{':
                stk.append('}')
            elif c == '[':
                stk.append(']')
            elif not stk or stk.pop() != c:
                return False
        
        return not stk