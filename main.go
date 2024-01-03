package main

import (
	"citrine/cli"
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: citrine <command>")
		os.Exit(1)
	}

	switch cmd := os.Args[1]; cmd {
	case "add":
		cli.AddTask()
	case "list":
		cli.ListTasks()
	case "delete":
		cli.DeleteTask()
	case "update":
		cli.UpdateTask()
	default:
		fmt.Println("Unrecognized command:", cmd)
		fmt.Println("Supported Commands: add, list, delete, update")
	}
}
