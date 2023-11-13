#include<iostream>
#include<vector>

using std::cout;
using std::endl;
using std::vector;

class Solution {
public:
	vector<int> twoSum(vector<int>& nums, int target) {
		size_t i, j;
		for (i = 0; i < nums.size(); i++) {
			for (j = i + 1; j < nums.size(); j++) {
				if (nums[i] + nums[j] == target) {
					return vector<int>{static_cast<int>(i), static_cast<int>(j)};
				}
			}
		}
		return vector<int>{static_cast<int>(-1), static_cast<int>(-1)};
	}
};

int main(int argc, char* argv) {
	Solution sol;
	vector<int> nums{2, 7, 11, 15};
	int target(9);
	for (int result : sol.twoSum(nums, target)) {
		cout << result << endl;
	}
	return 0;
}
