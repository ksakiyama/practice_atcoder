package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	n := readInt64()

	if n == 0 {
		fmt.Println(0)
		os.Exit(0)
	}
	if n < 10 {
		fmt.Println(n)
		os.Exit(0)
	}

	N := getNumberOfDigits(n)
	maxnum := int64(0)
	for i := 1; i < N; i++ {
		digitVal := getDigitValue(n, i)
		if digitVal != 9 {
			n = n - ((int64(digitVal) + 1) * int64(math.Pow(10.0, float64(i-1))))
		}

		// fmt.Println(i, ":", n)
		maxnum = max(maxnum, calcDigitsSum(n))
	}

	fmt.Println(maxnum)
}

func getNumberOfDigits(n int64) int {
	cnt := 0
	for {
		cnt++
		n = int64(n / 10)
		if n == 0 {
			break
		}
	}
	return cnt
}

func getDigitValue(n int64, idx int) int {
	cnt := 0
	for {
		cnt++
		ret := n % 10
		n = int64(n / 10)
		if idx == cnt {
			return int(ret)
		}
		if n == 0 {
			fmt.Println("Error@getDigitValue")
			break
		}
	}
	return -1 // invalid value
}

func calcDigitsSum(n int64) int64 {
	sum := int64(0)
	for {
		sum += n % 10
		n = int64(n / 10)
		if n == 0 {
			break
		}
	}
	return sum
}

func max(a, b int64) int64 {
	if a > b {
		return a
	}
	return b
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

// 3141592653589793
