import re

class Solution:
    def isPalindrome(self, s: str) -> bool:
        clean = re.sub('[^a-zA-Z0-9]', '', s.lower())
        return clean == clean[::-1]

if __name__ == '__main__':
    sol = Solution()
    #sample = 'A man, a plan, a canal: Panama'
    #sample = 'race a car'
    sample = ' '
    result = sol.isPalindrome(sample)
    print('result is', result)
