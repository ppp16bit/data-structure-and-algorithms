class Solution {
public:
  vector<int> findSubstring(string s, vector<string> &words) {
    unordered_map<string, int> wordCount;

    for (auto &word : words) {
      ++wordCount[word];
    }

    int stringSize = s.size(), wordCountSize = words.size(),
        wordSize = words[0].size();

    vector<int> substrIndices;

    for (int i = 0; i < wordSize; ++i) {
      unordered_map<string, int> windowCount;

      int left = i, right = i;
      int totalCount = 0;

      while (right + wordSize <= stringSize) {
        string currentWord = s.substr(right, wordSize);
        right += wordSize;

        if (!wordCount.count(currentWord)) {
          windowCount.clear();
          left = right;
          totalCount = 0;
          continue;
        }

        ++windowCount[currentWord];
        ++totalCount;

        while (windowCount[currentWord] > wordCount[currentWord]) {
          string wordToRemove = s.substr(left, wordSize);
          left += wordSize;
          --windowCount[wordToRemove];
          --totalCount;
        }

        if (totalCount == wordCountSize) {
          substrIndices.push_back(left);
        }
      }
    }

    return substrIndices;
  }
};