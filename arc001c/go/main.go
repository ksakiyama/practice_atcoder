package main

import (
	"bufio"
	"fmt"
	"os"
)

// N : width, height
const N int = 8

var cell [N * N]bool

func checkBorder(x, y int) bool {
	if x < 0 || x > N-1 || y < 0 || y > N-1 {
		return false
	}
	return true
}

// 8方向にQが存在するとfalse（安全じゃない）
func checkSafe(x, y int) bool {
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				continue
			}

			step := 1
			for {
				nx := x + dx*step
				ny := y + dy*step
				if !checkBorder(nx, ny) {
					break
				} else if cell[ny*N+nx] {
					return false
				}
				step++
			}
		}
	}
	return true
}

func dfs(pos, n int) bool {
	if n == 0 {
		return true
	}
	if pos == N*N {
		return false
	}

	y := pos / N
	x := pos % N

	if cell[pos] {
		if checkSafe(x, y) {
			if dfs(pos+1, n-1) {
				return true
			}
		}
	} else {
		if checkSafe(x, y) {
			cell[pos] = true
			if dfs(pos+1, n-1) {
				return true
			}
			cell[pos] = false
		}
		if dfs(pos+1, n) {
			return true
		}
	}
	return false
}

func main() {
	sc.Split(bufio.ScanWords)

	for i := 0; i < N; i++ {
		s := nextLine()
		for j, c := range s {
			idx := i*N + j
			if c == '.' {
				cell[idx] = false
			} else {
				cell[idx] = true
			}
		}
	}
	// fmt.Println(cell)

	if dfs(0, 8) {
		for y := 0; y < N; y++ {
			for x := 0; x < N; x++ {
				if cell[y*N+x] {
					fmt.Print("Q")
				} else {
					fmt.Print(".")
				}
			}
			fmt.Println("")
		}
	} else {
		fmt.Println("No Answer")
	}
}

var sc = bufio.NewScanner(os.Stdin)

func nextLine() string {
	sc.Scan()
	return sc.Text()
}
