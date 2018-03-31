#include <algorithm>
#include <iostream>
#include <vector>

typedef unsigned long long ll;

using namespace std;

int main() {
  ll n;
  cin >> n;

  vector<ll> v;
  for (int i = 0; i < n; i++) {
    ll t;
    cin >> t;
    v.push_back(t);
  }

  // すべて同じ値だった時
  sort(v.begin(), v.end());
  v.erase(unique(v.begin(), v.end()), v.end());
  if ( v.size() == 1 ) {
    cout << v.front() << endl;
    return 0;
  }

  // Core
  ll maxval = v.back();
  ll org = maxval;
  // ll maxval = *max_element(v.begin(), v.end());
  while (true) {
    bool ng = false;
    for (int i = n-1; i >= 0; i--) {
      if (maxval % v[i] != 0) {
        ng = true;
        break;
      }
    }
    if (!ng) {
      break;
    }
    maxval += org; // 針を一周させる
  }

  cout << maxval << endl;

  return 0;
}

// 9,223,372,036,854,775,807