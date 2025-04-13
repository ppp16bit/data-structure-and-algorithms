#include <vector>
#include <string>
#include <utility>
using namespace std;

class Solution {
    vector<vector<string>> res;

public:
    bool isPromising(vector<pair<int, int>> memo, int r, int c, int n) {
        for (auto& p : memo) {
            if (c == p.second)
                return false;
            for (int i = p.first + 1, j = 1; i < n; i++, j++) {
                if (r == i && c == p.second + j)
                    return false;
                if (r == i && c == p.second - j)
                    return false;
            }
        }

        return true;
    }

    void traverse(vector<string> board, int row, int col, int n,
                  vector<pair<int, int>> memo) {
        if (!isPromising(memo, row, col, n))
            return;
        memo.push_back(make_pair(row, col)); 

        if (row == n - 1) {
            vector<string> partialRes;
            for (const auto& p : memo) {
                string step(n, '.');
                step[p.second] = 'Q';
                partialRes.push_back(step);
            }
            res.push_back(partialRes);
            return;
        }

        for (int i = 0; i < n; i++) {
            traverse(board, row + 1, i, n, memo);
        }
    }

    vector<vector<string>> solveNQueens(int n) {
        vector<string> board(n);

        for (int i = 0; i < n; i++) {
            vector<pair<int, int>> memo;
            traverse(board, 0, i, n, memo);
        }

        return res;
    }
};