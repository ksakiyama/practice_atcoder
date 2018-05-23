#include <iostream>
#include <vector>

using namespace std;

int main()
{
  int n;
  cin >> n;

  vector<int> vec(n + 1);
  for (int i = 0; i < n; i++)
  {
    int a;
    cin >> a;
    vec[a] = i;
  }

  int ans = 1;
  int i = 2, cur = 1;
  while (i <= n)
  {
    while (vec[i] > vec[i - 1] && i <= n)
    {
      cur++, i++;
    }
    ans = max(ans, cur);
    cur = 1;
    i++;
  }
  cout << n - ans << endl;
  return 0;
}