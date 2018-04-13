package main

import (
	"fmt"
	"os"
)

func main() {
	x := readInt64()
	cnt := int64(x/11) * 2
	if x%11 == 0 {
		fmt.Println(x / 11 * 2)
		os.Exit(0)
	}

	x = x % 11
	if x <= 6 {
		fmt.Println(cnt + 1)
	} else {
		fmt.Println(cnt + 2)
	}
}

func readInt64() int64 {
	var a int64
	fmt.Scan(&a)
	return a
}
