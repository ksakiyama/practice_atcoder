#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <utility>

typedef long long ll;
using namespace std;

const int W = 0;
const int B = 1;
/*
W:0
B:1
*/
int main() {
  string s;
  cin >> s;

  vector<int> vec;
  for (auto c : s) {
    if (c == 'W') {
      vec.push_back(W);
    } else {
      vec.push_back(B);
    }
  }

  int cnt = 0;
  int before = vec[0];
  for (int i = 1; i < vec.size(); i++) {
    if (before == vec[i]) {
      continue;
    } else {
      cnt++;
      before = vec[i];
    }
  }

  cout << cnt << endl;
  return 0;
}