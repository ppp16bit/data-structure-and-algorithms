class Solution {
    public:
        int searchInsert(vector<int>& nums, int target) {
            int left = 0;
            int right = nums.size();

            while (left < right) {
                const int m = (left + right) / 2;
                if (nums[m] == target)
                  return m;
                if (nums[m] < target)
                  left = m + 1;
                else
                  right = m;
            }
            return left;
        }
    };