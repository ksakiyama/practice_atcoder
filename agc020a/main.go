package main

import (
	"fmt"
	"os"
)

func main() {
	Alice := "Alice"
	Borys := "Borys"

	n := readInt32()
	a := readInt32()
	b := readInt32()
	if n <= 2 {
		fmt.Println(Borys)
		os.Exit(0)
	}

	if absInt32(a, b)%2 == 0 {
		fmt.Println(Alice)
	} else {
		fmt.Println(Borys)
	}
}

func readInt32() int32 {
	var a int32
	fmt.Scan(&a)
	return a
}

func absInt32(a, b int32) int32 {
	ret := a - b
	if ret >= 0 {
		return ret
	}
	return ret * -1
}
