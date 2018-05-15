#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

using namespace std;

int main()
{
  int x;
  cin >> x;

  vector<int> vec(1000 + 1, 0);
  vec[1] = 1;

  for (int i = 2; i <= x; i++)
  {
    int res = i * i;
    if (x >= res)
    {
      while (x >= res)
      {
        res *= i;
      }
      vec[i] = res / i;
    }
  }

  decltype(vec)::iterator ret = max_element(vec.begin(), vec.end());
  // auto ret = max_element(vec.begin(), vec.end());
  cout << *ret << endl;
  // for (auto v : vec)
  // {
  //   cout << v << ",";
  // }
}