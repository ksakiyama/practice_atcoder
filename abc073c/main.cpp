#include <iostream>
#include <vector>
#include <algorithm>
#include <map>

typedef long long ll;
using namespace std;

int main() {
  ll n;
  cin >> n;

  map<ll, ll> m;

  for (ll i = 0; i < n; i++) {
    ll t;
    cin >> t;

    // TODO
    auto itr = m.find(t);
    if (itr != m.end()) {
      // hasKey
      m.erase(itr);
    } else {
      m[t] = 1;
    }
  }

  cout << m.size() << endl;

  return 0;
}