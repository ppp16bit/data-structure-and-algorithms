class Solution {
public:
  int maxArea(vector<int> &height) {
    int ans = 0;
    int left = 0;
    int right = height.size() - 1;

    while (left < right) {
      const int minHeight = min(height[left], height[right]);
      ans = max(ans, minHeight * (right - left));

      if (height[left] < height[right])
        ++left;
      else
        --right;
    }

    return ans;
  }
};