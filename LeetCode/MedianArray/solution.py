class Solution(object):
    def findMedianSortedArrays(self, nums1, nums2):
        merged = sorted(nums1 + nums2)
        middle = len(merged) // 2
        return float(merged[middle]) if len(merged) % 2 != 0 else (merged[middle-1] + merged[middle])/2.0

if __name__ == '__main__':
    solution = Solution()
    result = solution.findMedianSortedArrays([1, 2], [3, 4])
    print('result is', result)
