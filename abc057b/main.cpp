#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

const int INF = 1000000000;
const int INF_x = -1000000000;
const int INF_y = -1000000000;

struct Point {
  int x;
  int y;
  int num;
};

bool operator<(const Point &left, const Point &right) {
  int dl = abs(left.x - INF_x) + abs(left.y - INF_y);
  int dr = abs(right.x - INF_x) + abs(right.y - INF_y);
  if (dl == dr) {
    return left.num < right.num;
  }
  return dl < dr;
}

int calc(const Point &left, const Point &right) {
  return abs(left.x - right.x) + abs(left.y - right.y);
}

int main() {
  int n, m;
  cin >> n;
  cin >> m;

  vector<Point> students;
  vector<Point> points;
  for (int i = 0; i < n; i++) {
    Point p;
    cin >> p.x;
    cin >> p.y;
    p.num = i;
    students.push_back(p);
  }
  for (int i = 0; i < m; i++) {
    Point p;
    cin >> p.x;
    cin >> p.y;
    p.num = i;
    points.push_back(p);
  }
  
  /*
    アイデア
    INFを基準に、マンハッタン距離でソート
  */
  sort(points.begin(), points.end());

  for (int i = 0; i < n; i++) {
    Point best;
    int best_d = INF;
    for (int j = 0; j < m; j++) {
      int d = calc(students[i], points[j]);
      if (best_d > d) {
        best = points[j];
        best_d = d;
      }
    }
    cout << best.num+1 << endl;
  }
}

/*
解説をみると、こんなにめんどくさいループ回さなくていいっぽい。
N,Mを見て、計算量が小さそうだったら、素直に2重ループ回せばよい。
*/