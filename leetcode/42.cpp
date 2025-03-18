class Solution {
public:
  int trap(vector<int> &height) {
    if (height.empty())
      return 0;

    int ans = 0;
    int left = 0;
    int right = height.size() - 1;
    int maxLeft = height[left];
    int maxRight = height[right];

    while (left < right)
      if (maxLeft < maxRight) {
        ans += maxLeft - height[left];
        maxLeft = max(maxLeft, height[++left]);
      } else {
        ans += maxRight - height[right];
        maxRight = max(maxRight, height[--right]);
      }
    return ans;
  }
};