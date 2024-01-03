package cli

import (
	"citrine/utils"
	"flag"
	"fmt"
	"os"
	"time"
)

func AddTask() {
	// Register the add subcommand
	addCmd := flag.NewFlagSet("add", flag.ExitOnError)

	// Register the flags
	title := addCmd.String("title", "", "Title of the task")
	description := addCmd.String("description", "", "Description of the task")
	due := addCmd.String("due", "", "Due date of the task in RFC3339 format")

	// Parse the flags passed to the add subcommand
	if err := addCmd.Parse(os.Args[2:]); err != nil {
		fmt.Println("Failed to parse add command flags:", err)
		os.Exit(1)
	}
	// Check if the flags were provided, if not, print usage information
	if *title == "" || *description == "" || *due == "" {
		addCmd.PrintDefaults() // This will print default usage information for missing flags
		os.Exit(1)
	}
	// grab the due flag and attempt to parse it as a time.rfc3339 value
	// if it fails, print an error message and exit
	t, err := time.Parse(time.RFC3339, *due)
	if err != nil {
		fmt.Println("Failed to parse due date:", err)
		os.Exit(1)
	}
	if t.Before(time.Now()) {
		fmt.Println("Due date cannot be in the past")
		os.Exit(1)
	}

	// Create a new task with the provided information
	task := utils.Task{
		ID:          utils.GenerateID(),
		Title:       *title,
		Description: *description,
		Status:      "open", // Assuming new tasks always start with status "open"
		CreatedAt:   time.Now().Format(time.RFC3339),
		UpdatedAt:   time.Now().Format(time.RFC3339),
		DueAt:       *due,
	}

	// Use the `utils.AddTask` function to save the new task
	if err := utils.AddTask(task); err != nil {
		fmt.Println("Error adding task:", err)
		os.Exit(1)
	} else {
		fmt.Println("Task added successfully:", task.ID)
	}
}

func ListTasks() {
	// the list subcommand will list all the tasks in the .citrine.yml file
	// flags: none

	// use the utils.GetTasks function to get all the tasks
	tasks, err := utils.GetTasks()
	if err != nil {
		fmt.Println("No tasks found")
		os.Exit(1)
	}
	const format = "%-20s %-20s %-30s %-30s %-30s %-30s %-30s\n"
	fmt.Printf(format, "ID", "Title", "Description", "Status", "Created At", "Updated At", "Due At")
	fmt.Printf(format, "--", "-----", "-----------", "------", "----------", "----------", "------")
	// print the tasks
	for _, task := range tasks {
		// print all task fields separated by a tab character, ensure the values are aligned regardless of the length of the field, print a table header
		fmt.Printf(format, task.ID, task.Title, task.Description, task.Status, task.CreatedAt, task.UpdatedAt, task.DueAt)

	}
}

func UpdateTask() {
	// the update subcommand will update a task in the .citrine.yml file
	// flags: id, title (optional), description (optional), status (optional), due (optional)

	// Register the update subcommand
	updateCmd := flag.NewFlagSet("update", flag.ExitOnError)

	// Register the flags
	id := updateCmd.String("id", "", "ID of the task")
	title := updateCmd.String("title", "", "Title of the task")
	description := updateCmd.String("description", "", "Description of the task")
	status := updateCmd.String("status", "", "Status of the task")
	due := updateCmd.String("due", "", "Due date of the task in RFC3339 format")

	// Parse the flags passed to the update subcommand
	if err := updateCmd.Parse(os.Args[2:]); err != nil {
		fmt.Println("Failed to parse update command flags:", err)
		os.Exit(1)
	}

	// Check if the flags were provided, if not, print usage information
	if *id == "" {
		updateCmd.PrintDefaults() // This will print default usage information for missing flags
		os.Exit(1)
	}

	var newTask utils.Task

	if *title != "" {
		newTask.Title = *title
	}
	if *description != "" {
		newTask.Description = *description
	}
	if *status != "" {
		if *status != "open" && *status != "in_progress" && *status != "completed" {
			fmt.Println("Invalid status provided")
			os.Exit(1)
		}
		newTask.Status = *status
	}
	if *due != "" {
		t, err := time.Parse(time.RFC3339, *due)
		if err != nil {
			fmt.Println("Failed to parse due date:", err)
			os.Exit(1)
		}
		if t.Before(time.Now()) {
			fmt.Println("Due date cannot be in the past")
			os.Exit(1)
		}
		newTask.DueAt = *due
	}

	if err := utils.UpdateTask(*id, newTask); err != nil {
		fmt.Println("Error updating task:", err)
		os.Exit(1)
	} else {
		fmt.Println("Task updated successfully:", *id)
	}
}

func DeleteTask() {
	// the delete subcommand will delete a task in the .citrine.yml file
	// flags: id

	// Register the delete subcommand
	deleteCmd := flag.NewFlagSet("delete", flag.ExitOnError)

	// Register the flags
	id := deleteCmd.String("id", "", "ID of the task")

	// Parse the flags passed to the delete subcommand
	if err := deleteCmd.Parse(os.Args[2:]); err != nil {
		fmt.Println("Failed to parse delete command flags:", err)
		os.Exit(1)
	}

	// Check if the flags were provided, if not, print usage information
	if *id == "" {
		deleteCmd.PrintDefaults() // This will print default usage information for missing flags
		os.Exit(1)
	}

	if err := utils.RemoveTask(*id); err != nil {
		fmt.Println("Error deleting task:", err)
		os.Exit(1)
	} else {
		fmt.Println("Task deleted successfully:", *id)
	}

}
