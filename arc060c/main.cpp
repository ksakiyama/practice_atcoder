#include <iostream>
#include <vector>

typedef long long ll;
using namespace std;

// global
vector<ll> vec;
vector<ll> used;

ll dfs(ll pos, ll n, ll a) {
  if (pos == n) {
    // calc ave
    ll sum = 0, cnt = 0;
    for (ll i = 0; i < n; i++) {
      if (used[i]) {
        sum += vec[i];
        cnt++;
      }
    }
    // 平均値がkなので1
    if ((double)a == ((double)sum / cnt)) {
      return 1;
    }
    return 0;
  }

  ll ans = 0;
  if (used[pos] < 0) {
    // 使う
    used[pos] = 1;
    ans += dfs(pos+1, n, a);

    // 使わない場合
    used[pos] = 0;
    ans += dfs(pos+1, n, a);

    used[pos] = -1;
  }

  return ans;
}

int main() {
  ll n, a;
  cin >> n, cin >> a;
  
  for (ll i = 0; i < n; i++) {
    ll t;
    cin >> t;
    vec.push_back(t);
  }

  // debug
  if (n >= 17) {
    return 1;
  }

  vector<ll> _used(n, -1);
  used = _used; // copy

  cout << dfs(0, n, a) << endl;

  return 0;
}