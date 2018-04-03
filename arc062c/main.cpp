#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <utility>

#define REP(i,n) for(int i=0;i<n;i++)

typedef long long ll;
using namespace std;

int main() {
  int n;
  cin >> n;

  vector<ll> a;
  vector<ll> t;

  REP(i, n) {
    int _a, _t;
    cin >> _a, cin >> _t;
    a.push_back(_a);
    t.push_back(_t);
  }

  // 0 - (N-1)
  ll pa = 1, pt = 1; // 少なくとも1票
  REP(i, n) {
    if (i == 0) {
      pa = a[i];
      pt = t[i];
      continue;
    }

    // TLE, TODO
    ll cnt = 1;
    ll na = a[i], nt = t[i];
    if ( (na < pa) || (nt < pt) ) {
      ll x, y;
      if (pa % na == 0) {
        x = pa / na;
      } else {
        x = (ll)(pa / na) + 1;
      }
      if (pt % nt == 0) {
        y = pt / nt;
      } else {
        y = (ll)(pt / nt) + 1;
      }
      cnt = max(x, y);
    }

    pa = na * cnt;
    pt = nt * cnt;
  }

  cout << pa + pt << endl;

  return 0;
}
