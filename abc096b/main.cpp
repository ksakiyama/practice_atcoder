#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

int main()
{
  vector<int> vec(3);
  int  k;
  cin >> vec[0], cin >> vec[1], cin >> vec[2], cin >> k;

  sort(vec.begin(), vec.end());
  for (int i = 0; i < k; i++) {
    vec[2] *= 2;
  }

  cout << accumulate(vec.begin(), vec.end(), 0) << endl;

}