#include <iostream>
using namespace std;
int main()
{
  int _a, b, _c, d;
  cin >> _a, cin >> b, cin >> _c, cin >> d;

  int a = min(_a, _c);
  int c = max(_a, _c);

  if (d >= abs(a - c))
  {
    cout << "Yes" << endl;
    return 0;
  }

  // if ((b >= c) && (b <= a))
  // {
  //   cout << "No" << endl;
  //   return 0;
  // }

  if ((d >= abs(a - b) && (d >= abs(b - c))))
  {
    cout << "Yes" << endl;
    return 0;
  }

  cout << "No" << endl;
  return 0;
}