import re

class Solution(object):
    def isMatch(self, s, p):
        return re.match('^{}$'.format(re.sub('\*{2,%d}' % len(p), '*', p) if len(p) >= 2 else p), s) != None

if __name__ == '__main__':
    solution = Solution()
    result = solution.isMatch('abc', 'a***abc')
    print('result is', result)
