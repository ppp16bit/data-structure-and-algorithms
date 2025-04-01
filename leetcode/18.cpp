class Solution {
public:
  vector<vector<int>> fourSum(vector<int> &nums, int target) {
    vector<vector<int>> ans;
    vector<int> path;
    ranges::sort(nums);
    nSum(nums, 4, target, 0, nums.size() - 1, path, ans);
    return ans;
  }

private:
  void nSum(const vector<int> &nums, long n, long target, int left, int right,
            vector<int> &path, vector<vector<int>> &ans) {
    if (right - left + 1 < n || target < nums[left] * n || target > nums[right] * n)
      return;
    if (n == 2) {
      while (left < right) {
        const int sum = nums[left] + nums[right];
        if (sum == target) {
          path.push_back(nums[left]);
          path.push_back(nums[right]);
          ans.push_back(path);
          path.pop_back();
          path.pop_back();
          ++left;
          --right;
          while (left < right && nums[left] == nums[left - 1])
            ++left;
          while (left < right && nums[right] == nums[right + 1])
            --right;
        } else if (sum < target) {
          ++left;
        } else {
          --right;
        }
      }
      return;
    }

    for (int i = left; i <= right; ++i) {
      if (i > left && nums[i] == nums[i - 1])
        continue;
      path.push_back(nums[i]);
      nSum(nums, n - 1, target - nums[i], i + 1, right, path, ans);
      path.pop_back();
    }
  }
};