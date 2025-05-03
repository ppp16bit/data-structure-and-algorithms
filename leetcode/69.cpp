class Solution {
public:
    int mySqrt(int x) {
        unsigned left = 1;
        unsigned right = x + 1u;

        while (left < right) {
            const unsigned m = (left + right) / 2;

            if (m > x / m)
                right = m;
            else
                left = m + 1;
        }

        return left - 1;
    }
};