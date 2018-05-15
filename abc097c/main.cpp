#include <iostream>
#include <algorithm>
#include <set>
using namespace std;

int main() {
  string s;
  getline(cin, s);
  int K;
  cin >> K;

  set<string> S;
  int l = s.size();

  for (int i = 0; i <= l-1; i++) {
    for(int j = 1; j <= K; j++) {
      S.insert(s.substr(i, j));
    }
  }

  for (int i = 0; i < K-1; i++) {
    S.erase(S.begin());
  }

  cout << *S.begin() << endl;

  return 0;
}