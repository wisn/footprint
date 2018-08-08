package main

import (
	"bufio"
	"fmt"
	"os"
)

func Sqrt(x float64) float64 {
	z := 1.0

	for i := 0; i < 50; i++ {
		z -= ((z * z) - x) / (2 * z)
	}

	return z
}

func main() {
	fmt.Println("SQRT")
	fmt.Println("Input n < 0 to quit from this program.")
	fmt.Println()

	stdin := bufio.NewReader(os.Stdin)
	n := 1

	for ; n > 0; {
		for {
			fmt.Print("Sqrt: ")
			_, err := fmt.Fscan(stdin, &n)

			if err == nil {
				break
			}

			stdin.ReadString('\n')
			fmt.Println("Please input a valid number.")
			fmt.Println()
		}

		if n < 1 {
			fmt.Println("Good bye!")
			break
		}

		fmt.Println(Sqrt(float64(n)))
		fmt.Println()
	}
}

