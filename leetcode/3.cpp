class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    int ans = 0;
    vector<int> count(128);

    for (int left = 0, right = 0; right < s.length(); ++right) {
      ++count[s[right]];
      while (count[s[right]] > 1)
        --count[s[left++]];
      ans = max(ans, right - left + 1);
    }
    return ans;
  }
};