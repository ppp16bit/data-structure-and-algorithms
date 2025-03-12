class Solution {
    public:
        vector<int> searchRange(vector<int>& nums, int target) {
            const int left = ranges::lower_bound(nums, target) - nums.begin();

            if (left == nums.size() || nums[left] != target)
              return {-1, -1};

            const int right = ranges::upper_bound(nums, target) - nums.begin() - 1;
            return {left, right};
        }
    };