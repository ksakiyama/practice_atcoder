package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	n := readInt64()
	a := readInt64Line()
	b := readInt64Line()

	sumA := sumInt64Array(a)
	sumB := sumInt64Array(b)
	// if sumA > sumB {
	// 	fmt.Println("No")
	// 	os.Exit(0)
	// } else if sumA == sumB {
	// 	fmt.Println("Yes")
	// 	os.Exit(0)
	// }

	cnt := int64(0)
	for i := 0; i < int(n); i++ {
		if a[i] > b[i] {
			cnt += a[i] - b[i]
		} else if a[i] > b[i] {
			cnt += int64((b[i] - a[i]) / 2)
		}
	}

	if sumB-sumA >= cnt {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}

	os.Exit(0)
}

func readInt64() int64 {
	var a int64
	fmt.Scan(&a)
	return a
}

func readInt64Line() []int64 {
	var sc = bufio.NewScanner(os.Stdin)
	sc.Scan()
	line := sc.Text()
	vals := strings.Split(line, " ")
	var vec []int64
	for _, v := range vals {
		n, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			panic(err)
		}
		vec = append(vec, n)
	}
	return vec
}

func sumInt64Array(arr []int64) int64 {
	var sum int64
	for _, val := range arr {
		sum += val
	}
	return sum
}
