#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <set>
#include <utility>
#include <vector>

using ll = long long;
using ull = unsigned long long;
using namespace std;

int main()
{
  ll N, W;
  cin >> N, cin >> W;

  vector<ll> w1s;
  vector<ll> w2s;
  vector<ll> w3s;
  vector<ll> w4s;

  ll w1, v;
  cin >> w1, cin >> v;
  w1s.push_back(v);

  ll w2 = w1 + 1;
  ll w3 = w1 + 2;
  ll w4 = w1 + 3;

  for (ll i = 1; i < N; i++) {
    ll _w;
    cin >> _w, cin >> v;
    if (_w == w1)
      w1s.push_back(v);
    else if (_w == w2)
      w2s.push_back(v);
    else if (_w == w3)
      w3s.push_back(v);
    else // w4
      w4s.push_back(v);
  }

  if (w1 > W) {
    cout << 0 << endl;
    return 0;
  }

  // sort
  sort(w1s.begin(), w1s.end(), std::greater<ll>());
  sort(w2s.begin(), w2s.end(), std::greater<ll>());
  sort(w3s.begin(), w3s.end(), std::greater<ll>());
  sort(w4s.begin(), w4s.end(), std::greater<ll>());

  // i j k l
  ll ans = 0;
  for (ll i = 0; i <= (ll)(W / w1); i++) {
    ll W1 = W - (i * w1);
    for (ll j = 0; j <= (ll)(W1 / w2); j++) {
      ll W2 = W1 - (j * w2);
      for (ll k = 0; k <= (ll)(W2 / w3); k++) {
        ll W3 = W2 - (k * w3);
        for (ll l = 0; l <= (ll)(W3 / w4); l++) {
          ll a = 0;
          for (ll m = 0; m < min(i, (ll)w1s.size()); m++)
            a += w1s[m];
          for (ll m = 0; m < min(j, (ll)w2s.size()); m++)
            a += w2s[m];
          for (ll m = 0; m < min(k, (ll)w3s.size()); m++)
            a += w3s[m];
          for (ll m = 0; m < min(l, (ll)w4s.size()); m++)
            a += w4s[m];

          ans = max(ans, a);
        }
      }
    }
  }

  cout << ans << endl;

  return 0;
}