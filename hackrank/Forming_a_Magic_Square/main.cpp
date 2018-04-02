#include <algorithm>
#include <cmath>
#include <cstdio>
#include <iostream>
#include <vector>
using namespace std;

const int INF = 100000;
const int N = 3;
int cell[3][3];
int palette[3][3];
bool used[N * N];

/*
アイデア
すべてが15になるようにセットする
*/

bool check_magic() {
  for (int i = 0; i < N; i++){
    int sum = 0;
    for (int j = 0; j < N; j++) {
      sum += palette[i][j];
    }
    if (sum != 15) {
      return false;
    }
  }
  
  for (int i = 0; i < N; i++){
    int sum = 0;
    for (int j = 0; j < N; j++) {
      sum += palette[j][i];
    }
    if (sum != 15) {
      return false;
    }
  }

  // 斜めも15でないといけない…（知らんかった）
  if (palette[0][0] + palette[1][1] + palette[2][2] != 15) {
    return false;
  }
  if (palette[2][0] + palette[1][1] + palette[0][2] != 15) {
    return false;
  }

  return true;
}

int calc_cost() {
  int sum = 0;
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      sum += abs(cell[i][j] - palette[i][j]);
    }
  }
  return sum;
}

int solve(int pos) {
  if (pos == N * N) {
    // 魔法陣でない
    if (!check_magic()) {
      return INF;
    }
    // for (int i = 0; i < N * N; i++) {
    //   int y = i / N, x = i % N;
    //   cout << palette[y][x] << " ";
    // }
    // cout << ":" << calc_cost();
    // cout << endl;
    return calc_cost();
  }

  int cost = INF;
  for (int i = 0; i < N * N; i++) {
    if (!used[i]) {
      int y = pos / N, x = pos % N;
      used[i] = true;
      palette[y][x] = i+1;
      cost = min(solve(pos + 1), cost);
      used[i] = false;
      palette[y][x] = 0;
    }
  }
  return cost;
}

int main() {
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      int t;
      cin >> t;
      cell[i][j] = t;
    }
  }

  cout << solve(0) << endl;

  return 0;
}

/*
4 8 2
4 5 7
6 1 6

4 9 2
3 5 7
8 1 5
*/