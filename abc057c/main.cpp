#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

// 素因数分解する関数
// 不要だった…
vector<long> trial_division(long n) {
  vector<long> a;
  for (long i = 2; i < long(sqrt(n) + 2); i++) {
    while (n % i == 0) {
      n /= i;
      a.push_back(i);
    }
  }
  if (n > 1) {
    a.push_back(n);
  }
  return a;
}

long num_digits(long n) {
  if (n < 0) {
    return -1; // invalid
  } else if (n < 10) {
    return 1;
  }

  long cnt = 1;
  while (n > 0) {
    cnt++;
    n /= 10;
    if (n < 10) {
      return cnt;
    }
  }
  return -1; // invalid
}

int main() {
  long n;
  cin >> n;

  long ans = num_digits(n);

  for (long a = 1; a*a <= n; a++) {
    if (n % a != 0) continue;
    const long b = n / a;
    ans = min(ans, max(num_digits(a), num_digits(b)));
  }

  cout << ans << endl;
  return 0;
}
