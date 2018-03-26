package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var sc = bufio.NewScanner(os.Stdin)

const nmax = 8

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

	var graph = [nmax][nmax]bool{}

	for i := 0; i < m; i++ {
		a := nextInt()
		b := nextInt()
		graph[a-1][b-1] = true
		graph[b-1][a-1] = true
	}

	var visited = [nmax]bool{}
	visited[0] = true

	ans := dfs(0, n, visited, graph)
	fmt.Println(ans)
}

func dfs(v, N int, visited [nmax]bool, graph [nmax][nmax]bool) int {
	allVisited := true
	for i := 0; i < N; i++ {
		if visited[i] == false {
			allVisited = true
		}
	}

	if allVisited {
		return 1
	}

	ret := 0

	for i := 0; i < N; i++ {
		if !graph[v][i] {
			continue
		}
		if visited[i] {
			continue
		}

		visited[i] = true
		ret += dfs(i, N, visited, graph)
		visited[i] = false
	}

	return ret
}
