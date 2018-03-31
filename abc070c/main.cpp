#include <algorithm>
#include <iostream>
#include <vector>

typedef long long ll;

using namespace std;

// 最大公約数
ll gcd(ll a, ll b) {
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

// 最小公倍数
ll lcm(ll a, ll b) {
  ll g = gcd(a, b);
  return a / g * b; // Overflowに注意
}

int main() {
  ll n;
  cin >> n;

  ll ans = 1;
  for (int i = 0; i < n; i++) {
    ll t;
    cin >> t;
    ans = lcm(ans, t);
  }

  cout << ans << endl;

  return 0;
}
