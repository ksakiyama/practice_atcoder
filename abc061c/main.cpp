#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
  int n, k;
  cin >> n;
  cin >> k;

  vector<int> a, b;
  for (int i = 0; i < n; i++) {
    int _a, _b;
    cin >> _a, cin >> _b;
    a.push_back(_a);
    b.push_back(_b);
  }

  vector<int> stack;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < b[i]; j++) {
      stack.push_back(a[i]);
    }
  }

  sort(stack.begin(), stack.end());

  cout << stack[k-1] << endl;

  return 0;
}