class Solution {
public:
  int threeSumClosest(vector<int> &nums, int target) {
    int ans = nums[0] + nums[1] + nums[2];

    ranges::sort(nums);

    for (int i = 0; i + 2 < nums.size(); ++i) {
      if (i > 0 && nums[i] == nums[i - 1])
        continue;

      int left = i + 1;
      int right = nums.size() - 1;

      while (left < right) {
        const int sum = nums[i] + nums[left] + nums[right];

        if (sum == target)
          return sum;
        if (abs(sum - target) < abs(ans - target))
          ans = sum;
        if (sum < target)
          ++left;
        else
          --right;
      }
    }

    return ans;
  }
};