class Solution {
public:
  vector<string> generateParenthesis(int n) {
    vector<string> ans;
    dfs(n, n, "", ans);
    return ans;
  }

private:
  void dfs(int left, int right, string &&path, vector<string> &ans) {
    if (left == 0 && right == 0) {
      ans.push_back(path);
      return;
    }

    if (left > 0) {
      path.push_back('(');
      dfs(left - 1, right, std::move(path), ans);
      path.pop_back();
    }

    if (left < right) {
      path.push_back(')');
      dfs(left, right - 1, std::move(path), ans);
      path.pop_back();
    }
  }
};