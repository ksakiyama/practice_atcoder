#include <algorithm>
#include <cmath>
#include <iostream>
#include <utility>
#include <vector>

#define REP(i, n) for (int i = 0; i < n; i++)

typedef long long ll;
using namespace std;

const int n = 4;
const int m = 4;
string s = "abcd";
string t = "becd";

vector<vector<int>> dp;

void solve() {
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      if (s[i] == t[j]) {
        // 文字がマッチしたら
        dp[i+1][j+1] = dp[i][j] + 1; // カウント
      } else {
        dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
      }
    }
  }
  cout << dp[n][m] << endl;
}

int main() {
  REP(i, n+1) {
    vector<int> t(m+1);
    dp.push_back(t);
  }

  solve();

  return 0;
}
