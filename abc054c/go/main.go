package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var sc = bufio.NewScanner(os.Stdin)

const nmax = 8

var graph = [nmax][nmax]bool{}

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func main() {
	sc.Split(bufio.ScanWords)

	n := nextInt()
	m := nextInt()

	for i := 0; i < m; i++ {
		a := nextInt()
		b := nextInt()
		graph[a-1][b-1] = true
		graph[b-1][a-1] = true
	}

	var visited = [nmax]bool{}
	visited[0] = true

	ans := dfs(0, n, visited[:])
	fmt.Println(ans)
}

func dfs(v, N int, visited []bool) int {
	allVisited := true
	for i := 0; i < N; i++ {
		if !visited[i] {
			allVisited = false
		}
	}

	if allVisited {
		return 1
	}

	ret := 0

	for i := 0; i < N; i++ {
		if !graph[v][i] {
			// 頂点がつながってないので無視
			continue
		}
		if visited[i] {
			// すでに訪れたので無視（今回の問題は一筆書き）
			continue
		}

		// 上記のチェックに引っかからなかった
		// 到達可能なためtrue
		visited[i] = true
		// さらに深いポイントを探索していく
		ret += dfs(i, N, visited)
		// このルートは探索が終わったのでfalseに戻す
		visited[i] = false
	}

	return ret
}
