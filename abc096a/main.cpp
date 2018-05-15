#include <iostream>

using namespace std;

int main() {
  int a, b;
  cin >> a, cin >> b;

  int cnt = a;
  if (a > b) {
    cnt--;
  }

  cout << cnt << endl;
}