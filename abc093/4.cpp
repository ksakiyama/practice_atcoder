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
  ll Q;
  cin >> Q;
  vector<ll> vA, vB;
  REP(i, Q) {
    ll _A, _B;
    cin >> _A, cin >> _B;
    if (_A > _B) {
      swap(_A, _B);
    }
    vA.push_back(_A);
    vB.push_back(_B);
  }

  REP(j, Q) {
    ll A = vA[j];
    ll B = vB[j];
    ll AB = A * B;
    ll ans = 0;

    // if (A == B && A == 1) {
    //   cout << 0 << endl;
    //   continue;
    // }
    if (AB <= 2) {
      cout << 0 << endl;
      continue;
    }

    ll i_A = 1;
    ll i_B = 1;
    while (i_A <= A - 1) {
      if (i_A * i_B < AB) {
        ans+=2;
        i_A++;
        i_B++;
      } else {
        break;
      }
    }

    if (A != B) {
      i_A = A + 1;
      i_B = A;
      while (i_B <= B - 1) {
        if (i_A * i_B < AB) {
          ans+=2;
          i_A++;
          i_B++;
        } else {
          break;
        }
      }

      if ((i_A-1)*i_B >= AB) {
        ans--;
      }

      i_A = B + 1;
      i_B = B + 1;
      while (true) {
        if (i_A * i_B < AB) {
          ans++;
          i_A++;
          i_B++;
        } else {
          break;
        }
      }
    }

    cout << ans << endl;
  }

  return 0;
}