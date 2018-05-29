#include <iostream>
#include <vector>
#include <algorithm>
#include <set>
#include <map>
#include <cmath>
#include <utility>

using ll = long long;
using namespace std;

int main() {
  ll N, T;
  cin >> N, cin >> T;

  ll ans = 0;
  ll sec = 0;

  // i=0
  cin >> sec;
  ans += T;

  for (ll i = 1; i < N; i++) {
    ll now;
    cin >> now;
    if (T < now - sec) {
      ans += T;
    } else {
      ans += now - sec;
    }

    sec = now;
  }

  cout << ans << endl;

  return 0;
}