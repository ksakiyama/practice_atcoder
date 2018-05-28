#include <iostream>
#include <map>

using namespace std;

int main() {
  int N;
  cin >> N;
  string S;
  cin >> S;

  int ans = 0;
  string alphabet = "abcdefghijklmnopqrstuvwxyz";

  for(int i = 1; i < S.length() - 1; i++) {
    string a = S.substr(0, i);
    string b = S.substr(i);
    map<char, int> ma, mb;
    for (int j = 0; j < a.length(); j++) {
      ma[a[j]] = 0;
    }
    for (int j = 0; j < b.length(); j++) {
      mb[b[j]] = 0;
    }

    int cnt = 0;
    for (int j = 0; j < alphabet.length(); j++) {
      auto ita = ma.find(alphabet[j]);
      auto itb = mb.find(alphabet[j]);
      if (ita != ma.end() && itb != mb.end()) {
        cnt++;
      }
    }
    
    ans = max(ans, cnt);
  }
  cout << ans << endl;
}