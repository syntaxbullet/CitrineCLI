package utils

import (
	"fmt"
	"os"
	"path"
	"time"

	"gopkg.in/yaml.v2"
)

type Task struct {
	ID          string `yaml:"id"`
	Title       string `yaml:"title"`
	Description string `yaml:"description"`
	Status      string `yaml:"status"`
	CreatedAt   string `yaml:"created_at"`
	UpdatedAt   string `yaml:"updated_at"`
	DueAt       string `yaml:"due_at"`
}

func GetTasks() (tasks []Task, err error) {
	// this function looks for a .citrine.yml file in the current working directory and uses a yaml decoder to parse it and return a slice of Task structs
	workingDir, err := os.Getwd()
	if err != nil {
		fmt.Println("Error getting current working directory:", err)
		return nil, err
	}

	taskFilePath := path.Join(workingDir, ".citrine.yml")
	taskFile, err := os.Open(taskFilePath)
	// if the file doesn't exist, return an empty slice of tasks
	if os.IsNotExist(err) {
		return []Task{}, nil
	}
	// check for eof errors and return an empty slice of tasks
	if err != nil {
		return []Task{}, err
	}

	defer taskFile.Close()

	err = yaml.NewDecoder(taskFile).Decode(&tasks)
	if err != nil {
		defer taskFile.Close()
		return nil, err
	}
	return tasks, err
}

func WriteTasks(tasks []Task) (err error) {
	// this function writes a slice of Task structs to a .citrine.yml file in the current working directory
	workingDir, err := os.Getwd()
	if err != nil {
		return err
	}
	taskFilePath := path.Join(workingDir, ".citrine.yml")

	// Use os.Create to create or truncate the existing file
	taskFile, err := os.Create(taskFilePath)
	if err != nil {
		return err
	}
	defer taskFile.Close()

	// Encode and write the tasks to the file
	return yaml.NewEncoder(taskFile).Encode(tasks)
}

func AddTask(task Task) (err error) {
	// this function appends a task to the .citrine.yml file in the current working directory
	tasks, err := GetTasks()

	tasks = append(tasks, task)
	return WriteTasks(tasks)
}

func RemoveTask(taskID string) (err error) {
	// this function removes a task from the .citrine.yml file in the current working directory
	tasks, err := GetTasks()
	if err != nil {
		return err
	}

	found := false
	for i, task := range tasks {
		if task.ID == taskID {
			// Remove the task from the slice
			tasks = append(tasks[:i], tasks[i+1:]...)
			found = true
			break
		}
	}

	if !found {
		// Task ID not found in the slice
		return fmt.Errorf("task with ID %s not found", taskID)
	}

	return WriteTasks(tasks)
}

func UpdateTask(taskID string, newTask Task) (err error) {
	// this function updates a task in the .citrine.yml file in the current working directory
	tasks, err := GetTasks()
	if err != nil {
		return err
	}

	found := false
	for i, task := range tasks {
		if task.ID == taskID {
			// Update the task in the slice
			tasks[i] = newTask
			tasks[i].UpdatedAt = time.Now().Format(time.RFC3339)
			found = true
			break
		}
	}

	if !found {
		// Task ID not found in the slice
		return fmt.Errorf("task with ID %s not found", taskID)
	}

	return WriteTasks(tasks)
}
func GenerateID() (id string) {
	// this function generates a unique id for a task
	// it uses the current time in nanoseconds as the id
	return fmt.Sprintf("%d", time.Now().UnixNano())
}
