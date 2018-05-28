#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

int main()
{
  ll N;
  cin >> N;
  string S;
  cin >> S;

  ll tmp = 0;
  vector<ll> buf1(N);
  for (int i = 0; i < N; i++)
  {
    buf1[i] = tmp;
    if (S[i] == 'W')
      tmp++;
  }

  tmp = 0;
  vector<ll> buf2(N);
  for (int i = N - 1; i >= 0; i--)
  {
    buf2[i] = tmp;
    if (S[i] == 'E')
      tmp++;
  }

  ll ans = N;
  for (int i = 0; i < N; i++) {
    ans = min(ans, buf1[i] + buf2[i]);
  }

  cout << ans << endl;
}