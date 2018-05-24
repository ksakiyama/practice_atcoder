#include <iostream>
#include <vector>

using namespace std;
using ll = long long;

int main()
{
  int n;
  cin >> n;

  bool invalid = false;

  vector<ll> vec(n);
  cin >> vec[0];

  if (vec[0] != 0)
    invalid = true;

  for (int i = 1; i < n; i++)
  {
    cin >> vec[i];
    if (vec[i] > vec[i - 1] + 1)
      invalid = true;
  }

  if (invalid)
  {
    cout << -1 << endl;
    return 0;
  }

  ll cnt = 0;
  ll ans = vec[n - 1];
  for (int i = n - 2; i > 0; i--)
  {
    // 階段になっている場合は無視
    if (vec[i] == vec[i + 1] - 1)
    {
      continue;
    }
    // それ以外はその数値がコスト
    ans += vec[i];
  }

  cout << ans << endl;

  return 0;
}