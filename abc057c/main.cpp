#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;
// vector<long> trial_division(long n) {
//   vector<long> a;
//   long f = 2;
//   while (n > 1) {
//     if (n % f == 0) {
//       a.push_back(f);
//       n /= f;
//     } else {
//       f += 1;
//     }
//   }
//   return a;
// }
vector<long> trial_division(long n) {
  vector<long> a;
  a.push_back(1); // 後で
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

  // for (auto v : trial_division(n)) {
  //   cout << v << endl;
  // }

  vector<long> vec = trial_division(n);

  const long m = vec.size();
  long ans = 10000000000; // INF
  do {
    for (long i = 0; i < m-1; i++) {
      long a = 1, b = 1;
      for (long j = 0; j < i + 1; j++) {
        a *= vec[j];
      }
      for (long j = i + 1; j < m; j++) {
        b *= vec[j];
      }
      ans = min(ans, max(num_digits(a), num_digits(b)));
    }
  } while (next_permutation(vec.begin(), vec.end()));
  cout << ans << endl;
}
