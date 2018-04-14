// package MyFuncs

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

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
