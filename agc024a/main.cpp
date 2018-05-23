#include <iostream>

using ll = long long;
using namespace std;

const ll LIMIT = 1000000000000000000;

int main() {
  ll a, b, c, k;
  cin >> a, cin >> b, cin >> c, cin >> k;

  ll ans = a - b;
  if (k % 2 == 1) {
    ans *= -1;
  }

  if (abs(ans) > LIMIT) {
    cout << "Unfair" << endl;
  } else {
    cout << ans << endl;
  }

  return 0;
}