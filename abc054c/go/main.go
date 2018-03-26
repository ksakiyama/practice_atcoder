package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

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
	m := nextInt()

	fmt.Println(n)
	fmt.Println(m)

	a := make([]int, 0)
	b := make([]int, 0)

	for i := 0; i < m; i++ {
		// sc.Split(bufio.ScanWords)
		a = append(a, nextInt())
		b = append(b, nextInt())
	}

	fmt.Println(a)
	fmt.Println(b)
}
