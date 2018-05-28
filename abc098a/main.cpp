#include <iostream>
using namespace std;

int main()
{
  int a, b;
  cin >> a, cin >> b;

  cout << max(max(a + b, a - b), a * b) << endl;
}