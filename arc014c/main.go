package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

const NMAX = 15
const INF = 10000000

var balls [NMAX]int
var lines = make([]int, 1, NMAX)
var used [NMAX]bool

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func dfs(pos, n int) int {
	if pos == n {
		data := make([]int, n)
		copy(data, lines)
		// fmt.Print("", lines)
		// fmt.Print(":", data)
		for {
			flag := true
			for i := 1; i < len(data); i++ {
				if data[i] == data[i-1] {
					data = append(data[:i-1], data[i+1:]...)
					flag = false
					break
				}
			}
			if flag {
				break
			}
		}
		// fmt.Print(":", data)
		// fmt.Println(":", len(data))
		return len(data)
	}

	ans := INF
	if !used[pos] {
		used[pos] = true

		// 後につける場合
		lines = append(lines, balls[pos])
		ans = min(ans, dfs(pos+1, n))

		// もとに戻す
		lines = lines[:len(lines)-1]

		// 前につける場合
		// slice, slice[0] = append(slice[0:1], slice[0:]...)
		lines, lines[0] = append(lines[0:1], lines[0:]...), balls[pos]
		ans = min(ans, dfs(pos+1, n))

		// 元に戻す
		lines = lines[1:]

		used[pos] = false
	}
	return ans
}

func main() {
	sc.Split(bufio.ScanWords)
	n, err := strconv.Atoi(nextLine())
	if err != nil {
		panic(err)
	}

	if n > NMAX {
		return
	}

	sBalls := nextLine()
	for i, v := range sBalls {
		if v == 'R' {
			balls[i] = 1
		} else if v == 'G' {
			balls[i] = 2
		} else {
			balls[i] = 3
		}
	}

	used[0] = true
	lines[0] = balls[0]

	fmt.Println(dfs(1, n))
}

var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}
