package main

import (
	"fmt"
	"os"
)

// n:S
// m:c
// Scc
func main() {
	n, m := readInt64(), readInt64()

	var cnt int64
	if 2*n <= m {
		cnt = n // add # of S(n)
		m -= 2 * n
		cnt += int64(m / 4)
		fmt.Println(cnt)
		os.Exit(0)
	} else {
		cnt = int64(m / 2)
		fmt.Println(cnt)
		os.Exit(0)
	}
}

func readInt64() int64 {
	var a int64
	fmt.Scan(&a)
	return a
}
