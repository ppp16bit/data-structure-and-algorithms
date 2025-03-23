class Solution {
public:
  string longestPalindrome(string s) {
    const string t = join('@' + s + '$', '#');
    const vector<int> p = manacher(t);
    int maxPalindromeLength = 0;
    int bestCenter = -1;

    for (int i = 0; i < p.size(); ++i)
      if (p[i] > maxPalindromeLength) {
        maxPalindromeLength = p[i];
        bestCenter = i;
      }
    const int left = (bestCenter - maxPalindromeLength) / 2;
    const int right = (bestCenter + maxPalindromeLength) / 2;
    return s.substr(left, right - left);
  }

private:
  vector<int> manacher(const string &t) {
    vector<int> p(t.length());
    int center = 0;

    for (int i = 1; i < t.length() - 1; ++i) {
      const int rightBoundary = center + p[center];
      const int mirrorIdx = center - (i - center);

      if (rightBoundary > i)
        p[i] = min(rightBoundary - i, p[mirrorIdx]);

      while (t[i + 1 + p[i]] == t[i - 1 - p[i]])
        ++p[i];

      if (i + p[i] > rightBoundary)
        center = i;
    }
    return p;
  }

  string join(const string &s, char delimiter) {
    string joined;

    for (int i = 0; i < s.length() - 1; ++i) {
      joined += s[i];
      joined += delimiter;
    }
    joined += s.back();
    return joined;
  }
};