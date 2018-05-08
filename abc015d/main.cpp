#include <algorithm>
#include <cmath>
#include <iostream>
#include <utility>
#include <vector>

#define REP(i, n) for (int i = 0; i < n; i++)

typedef long long ll;
using namespace std;

// w以下
// 選択可能な枚数 k以下
// aコスト
// b価値

// http://kenkoooo.hatenablog.com/entry/2015/12/16/000721
// https://www.slideshare.net/chokudai/abc015
int main() {
  int W, N, K;
  cin >> W >> N >> K;
  vector<int> A(N), B(N);
  for (int i = 0; i < N; ++i)
    cin >> A[i] >> B[i];

  // dp[N+1][K+1][W+1]
  // -1で初期化
  vector<vector<vector<int>>> dp(
      N + 1, vector<vector<int>>(K + 1, vector<int>(W + 1, -1)));
  dp[0][0][0] = 0;
  int ans = 0;
  for (int i = 0; i < N; ++i) {
    for (int k = 0; k < K; ++k) {
      for (int w = 0; w <= W; ++w) {
        if (dp[i][k][w] < 0) {
          // まだ調べていない
          continue;
        }

        dp[i + 1][k][w] = max(dp[i + 1][k][w], dp[i][k][w]);
        
        // Wの許容範囲内のとき
        if (w + A[i] <= W) {
          // 幅i+1, 使用枚数k+1, w+A[i]の重量になったとき
          // どちらかの最大値 ⇒ i+1,k+1ですでに登録されている値 or 現時点にB[i]を加算した値
          dp[i + 1][k + 1][w + A[i]] =
              max(dp[i + 1][k + 1][w + A[i]], dp[i][k][w] + B[i]);
          ans = max(ans, dp[i + 1][k + 1][w + A[i]]);
        }
      }
    }
  }
  cout << ans << endl;

  return 0;
}

/*
DPだが、ここにkの配慮も必要。

int main() {
  int w;
  cin >> w;
  int n, k;
  cin >> n, cin >> k;

  vector<int> a;
  vector<int> b;
  REP(i, n) {
    int t;
    cin >> t;
    a.push_back(t);
    cin >> t;
    b.push_back(t);
  }

  // memo table
  vector<vector<int>> dp;
  REP(i, n+1) {
    vector<int> t(w+1);
    dp.push_back(t);
  }

  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= w; j++) {
      if (j < a[i]) {
        dp[i+1][j] = dp[i][j];
      } else {
        dp[i+1][j] = max(dp[i][j - a[i]] + b[i], dp[i][j]);
      }
    }
  }

  cout << dp[n][w] << endl;

  return 0;
}
*/