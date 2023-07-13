package main

import (
	"github.com/ryanccn/nyoom/cmd"
	"github.com/ryanccn/nyoom/config"
)

func main() {
	config.Init()
	cmd.Execute()
}
