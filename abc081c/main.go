package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

// const INF int = 1000000

var sc = bufio.NewScanner(os.Stdin)

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
	k := nextInt()
	list := make([]int, n)
	for i := 0; i < n; i++ {
		list[i] = nextInt()
	}

	// TODO
	dict := map[int]int{}

	// 種類ごとに個数をカウント
	for _, v := range list {
		_, hasKey := dict[v]
		if hasKey {
			dict[v]++
		} else {
			dict[v] = 1
		}
	}

	// 種類がk以下なら0
	l := len(keys(dict))
	if l <= k {
		fmt.Println(0)
		return
	}

	numReduce := l - k

	// kより多い場合、少ないものから消していく
	// ソートする
	vals := values(dict)
	sort.Ints(vals)

	idx := 0
	ans := 0
	for numReduce > 0 {
		ans += vals[idx]
		idx++
		numReduce--
	}

	// ソートしないで毎回探しているとすごく遅い
	// for {
	// 	var minKey int
	// 	minVal := INF
	// 	for key, val := range dict {
	// 		if min(minVal, val) == val {
	// 			minKey = key
	// 			minVal = val
	// 		}
	// 	}

	// 	ans += minVal
	// 	delete(dict, minKey)
	// 	numReduce--
	// 	if numReduce == 0 {
	// 		break
	// 	}
	// }

	fmt.Println(ans)
}

func keys(m map[int]int) []int {
	ks := []int{}
	for k := range m {
		ks = append(ks, k)
	}
	return ks
}

func values(m map[int]int) []int {
	vs := []int{}
	for _, v := range m {
		vs = append(vs, v)
	}
	return vs
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}
