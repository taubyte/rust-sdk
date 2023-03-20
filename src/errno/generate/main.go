package main

import (
	"bufio"
	"fmt"
	"os"
	"path"
	"strings"
)

func main() {
	wd, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	readFile, err := os.Open(path.Join(wd, "../mod.rs"))
	if err != nil {
		panic(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var checkVars bool
	var errnoCount int
	var errorStrings = `[
`
	for fileScanner.Scan() {
		if checkVars {
			if fileScanner.Text() == "}" {
				break
			}

			errorStrings = errorStrings + fmt.Sprintf(`	"%s",
`, strings.TrimSpace(strings.TrimSuffix(fileScanner.Text(), ",")))
			errnoCount++
		}
		if fileScanner.Text() == "pub enum Errno {" {
			checkVars = true
		}
	}

	toWrite := fmt.Sprintf(`// This file was generated, do not manually edit. 

pub const ERROR_STRINGS:[&str;%d] = %s];
`, errnoCount, errorStrings)

	err = os.WriteFile(path.Join(wd, "../error_strings.rs"), []byte(toWrite), 0755)
	if err != nil {
		panic(err)
	}
}
