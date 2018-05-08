#include <iostream>
#include <vector>

typedef long long ll;
using namespace std;

int main() {
  ll n, a;
  cin >> n, cin >> a;
  vector<ll> vec;
  ll maxval = 0;
  for (ll i = 0; i < n; i++) {
    ll t;
    cin >> t;
    vec.push_back(t);
    maxval = max(maxval, t);
  }

  ll dp[n][n][n*maxval+1];

  // Core
  for (int s = 0; s < n*maxval; s++) {
    for (int k = 0; k < n; k++) {
      for (int j = 0; j < n; j++) {
        if(s == 0 && k == 0 && j==0) {
          dp[j][k][s] = 1;
        }
        if (j > 0)
          dp[j][k][s] = dp[j-1][k-1][s - vec[j]] + dp[j-1][k][s];
      }
    }
  }

  return 0;
}