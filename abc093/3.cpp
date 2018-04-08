#include <algorithm>
#include <cmath>
#include <iostream>
#include <map>
#include <utility>
#include <vector>

#define REP(i, n) for (int i = 0; i < n; i++)

typedef long long ll;
using namespace std;

int main() {
  vector<ll> vec(3);
  cin >> vec[0], cin >> vec[1], cin >> vec[2];
  sort(vec.begin(), vec.end());

  ll A, B, C;
  A = vec[0];
  B = vec[1];
  C = vec[2];

  if (A == B && B == C) {
    cout << 0 << endl;
    return 0;
  }
  if (A == B && B < C) {
    cout << vec[2] - vec[0] << endl;
    return 0;
  }

  // B < Cの場合は処理をスキップ
  ll ans = 0;
  while (B < C) {
    A++;
    B++;
    ans++;
  }

  // ここでB==Cになっているはず
  // abs(A-B)のぶんだけ2を加算する
  ll diff = abs(A - B);
  if (diff % 2 == 0) {
    ans += diff / 2;
  } else {
    ans += (ll)(diff / 2) + 2;
  }

  cout << ans << endl;

  return 0;
}