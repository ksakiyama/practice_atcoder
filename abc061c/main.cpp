#include <iostream>
#include <vector>
#include <algorithm>
#include <utility>

using namespace std;

typedef long long ll;

int main() {
  ll n, k;
  cin >> n;
  cin >> k;

  vector< pair<int, ll> > v;
  for (int i = 0; i < n; i++) {
    pair<int, ll> p;
    cin >> p.first, cin >> p.second;
    v.push_back(p);
  }

  sort(v.begin(), v.end());

  ll start = 0;
  for (int i = 0; i < n; i++) {
    ll end = start + v[i].second;
    if (start <= k - 1 && k - 1 < end) {
      // cout << start << endl;
      // cout << end << endl;
      cout << v[i].first << endl;
      return 0;
    }

    start = end;
  }

  return 0;
}