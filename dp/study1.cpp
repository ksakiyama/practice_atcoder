#include <algorithm>
#include <cmath>
#include <iostream>
#include <utility>
#include <vector>

#define REP(i, n) for (int i = 0; i < n; i++)

typedef long long ll;
using namespace std;

int n, W;
vector<int> w;
vector<int> v;
vector<vector<int>> dp;

int rec(int i, int j) {
  if (dp[i][j] >= 0) {
    return dp[i][j];
  }

  int res;
  if (i == n) {
    // 残っていない
    res = 0;
  } else if (j < w[i]) {
    // w[i]は入らないので、i+1を見に行く
    res = rec(i + 1, j);
  } else {
    // i+1を見に行ったとき or iを選択してi+1を見に行ったとき
    // どちらかで最大の方を取得する
    res = max(rec(i + 1, j), rec(i + 1, j - w[i]) + v[i]);
  }

  dp[i][j] = res;
  return res;
}

// void solve() { cout << rec(0, W) << endl; }

// void solve() {
//   for (int i = n - 1; i >= 0; i--) {
//     for (int j = 0; j <= W; j++) {
//       if (j < w[i]) {
//         dp[i][j] = dp[i + 1][j];
//       } else {
//         dp[i][j] = max(dp[i + 1][j], dp[i + 1][j - w[i]] + v[i]);
//       }
//     }
//   }
//   cout << dp[0][W] << endl;
// }

// dp[N+1][W+1]を考える

void solve() {
  for (int i = 0; i < n; i++) { // すべての品物
    for (int j = 0; j <= W; j++) { // 許容できる重さを増やしていくループ（W以下まで)
      if (j < w[i]) {
        // 積み込めない
        dp[i+1][j] = dp[i][j];
      } else {
        // 積み込める場合
        // 現在の価値 or 積み込んだ場合を比較して最大の方
        dp[i+1][j] = max(dp[i][j], dp[i][j - w[i]] + v[i]);
      }
    }
  }
  cout << dp[n][W] << endl;
}

int main() {
  cin >> n, cin >> W;

  REP(i, n) {
    int t;
    cin >> t;
    w.push_back(t);
    cin >> t;
    v.push_back(t);
  }

  REP(i, n + 1) {
    // vector<int> t(W + 1, -1);
    vector<int> t(W + 1); // zeros
    dp.push_back(t);
  }

  solve();

  return 0;
}
/*
4 5
2 3
1 2
3 4
2 2
*/