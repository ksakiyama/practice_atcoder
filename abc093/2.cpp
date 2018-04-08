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
  ll A, B, K;
  cin >> A, cin >> B, cin >> K;

  if (abs(A - B) <= K) {
    
    for (ll i = A; i <= B; i++) {
      cout << i << endl;
    }
  } else {
    vector<ll> vec;
    for (ll i = A; i < A + K; i++) {
      vec.push_back(i);
    }
    for (ll i = B - K + 1; i <= B; i++) {
      vec.push_back(i);
    }
    sort( vec.begin(), vec.end() );
    vec.erase( unique( vec.begin(), vec.end() ), vec.end() );

    REP(i, vec.size()) {
      cout << vec[i] << endl;
    }
  }

  return 0;
}
