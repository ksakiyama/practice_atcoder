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
  ll a, b, c;
  cin >> a, cin >> b, cin >> c;

  for (ll i = 1; i <= b; i++) {
    if (c == i * a % b) {
      cout << "YES" << endl;
      return 0;
    }
  }

  cout << "NO" << endl;
  
  return 0;
}