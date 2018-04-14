package main

import (
	"fmt"
)

func main() {
	q := readInt64() // 1/4
	h := readInt64() // 1/2
	s := readInt64() // 1
	d := readInt64() // 2
	n := readInt64()

	e := s
	if e > q*4 {
		e = q * 4
	}
	if e > h*2 {
		e = h * 2
	}

	f := d
	if d > e*2 {
		f = e * 2
	}

	needYen := int64(n/2) * f
	if n%2 == 1 {
		needYen += e
	}

	fmt.Println(needYen)
}

func readInt64() int64 {
	var a int64
	fmt.Scan(&a)
	return a
}
