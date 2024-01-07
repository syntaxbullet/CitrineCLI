# The Citrine CLI
The CitrineCLI is the core component of citrine and the main entrypoint into the ecosystem that will be built around it, it exposes basic `create`, `read/list`, `update`, and `delete` capabilities for other applications and tools to interact with.
## Citrine's CLI follows the UNIX philosophy

The CLI is designed to be minimalist, modular and re-usable. On its own it is not capable of performing any complex task management workflows like recurring events or deadline notifications, instead it is designed to be easily combined with arbitrary utilities like `cron` or `grep` to provide it with time-triggers or search functionality.

Citrine's CLI tools are built to do one thing and do it well. They output results in a simple, text-based format that can be easily piped into other programs for further processing. This approach allows users to build their own customized workflows by chaining together commands using pipes (`|`) and other shell features.

For instance, a user could use Citrine's CLI to list tasks and then pipe the output to `sort` or `awk` for organizing tasks by priority or filtering tasks based on certain criteria. This flexibility is a core advantage of such a design – rather than being confined to the features provided by a monolithic application, users can leverage the entire ecosystem of UNIX command line tools.

Moreover, Citrine's CLI commands are designed to be non-interactive and stateless, which means they can be run as part of scripts without requiring user input during execution. This makes them ideal for automation via scripts or as part of larger systems using tools like `make` or even `systemd` timers.

Error handling in Citrine's CLI follows the UNIX convention: success is silent (exit code 0), while errors produce diagnostic messages to standard error (stderr) and exit with a non-zero status. This makes it easy to check for errors in scripts and take corrective action when necessary.

The configuration for Citrine’s CLI is typically stored in plain text files which users can edit using their preferred text editors. This empowers users with full control over their environment, allowing them to version control configurations, share them across teams, or quickly apply changes without navigating complex graphical interfaces.

In summary, Citrine's CLI offers a powerful foundation that embraces the composability and efficiency of traditional UNIX command-line tools. By supporting scripting and customization through standard input/output conventions, it invites both novice and experienced users alike to construct tailored solutions that fit seamlessly into their existing toolchains and workflows.
## CitrineCLI Spec

### Creating a Task with CitrineCLI

The `citrine add` command allows users to create a new task in the Citrine task manager. To create a task with specific attributes such as a due date, priority level, and tags, you can use the following flags:

- `-d --due`: Specify the due date and time for the task in RFC3339 format.
- `-p --priority`: Set the priority level of the task, using an integer between 0 (lowest) to 9 (highest).
- `-t --tags`: Add a list of tags to the task for categorization or filtering.

#### Syntax

```
citrine add <task_description> [flags]
```

`<task_description>` is a mandatory field where you describe the task to be added.

Flags are optional parameters that allow further customization of the task.

#### Example Usage

To create a math homework task with a due date, priority level, and tags, you would use `citrine add` like this:

```
citrine add do math homework -d "2023-04-05T14:00:00Z" -p 5 -t calculus,homework,urgent
```

In this example:
- The due timestamp is `2023-04-05T14:00:00Z`, which follows the RFC3339 format.
- The priority is set at `5`, indicating it's of medium-high importance.
- The tags `calculus`, `homework`, and `urgent` are added for organizational purposes.

### Updating a Task with CitrineCLI

The `citrine update` command allows users to modify an existing task in the Citrine task manager. To update a task with new attributes such as a due date, priority level, and tags or to change its description, you can use the following flags:

- `-d --due`: Update the due date and time for the task in RFC3339 format.
- `-p --priority`: Change the priority level of the task, using an integer between 0 (lowest) to 9 (highest).
- `-t --tags`: Replace current tags with a new list of tags for categorization or filtering.
- `--append-tags`: Add more tags to the existing list without removing current ones.
- `--remove-tags`: Remove specific tags from the existing list.
- `-m --message`: Update the description of the task.
- `-s --status`: Update the status of the task.

#### Syntax

```
citrine update <task_id> [flags]
```

`<task_id>` is a mandatory field that specifies which task should be updated. It is assumed that each task has a unique, sequential ID.

Flags are optional parameters that allow further customization of the task.

#### Example Usage

To update math homework task with ID 42 by changing its due date, priority level, and adding an additional tag, you would use `citrine update` like this:

```
citrine update 42 -d "2023-04-06T18:00:00Z" -p 6 --append-tags rescheduled
```

In this example:
- The due timestamp is changed to `2023-04-06T18:00:00Z`, according to RFC3339 format.
- The priority is updated to `6`, indicating it's of high importance now.
- The tag `rescheduled` is appended to the existing list of tags.

To change the description and remove specific tags from a task, use:

```
citrine update 42 -m "complete advanced calculus homework" --remove-tags urgent
```

Here:
- The description is updated to "complete advanced calculus homework".
- The tag `urgent` is removed from the existing list of tags.


considering this template create documentation on how to delete a task

### Deleting a Task with CitrineCLI

The `citrine delete` command is used to remove an existing task from the Citrine task manager. When a task is no longer needed or has been completed, you can delete it to keep your task list organized. Deleting a task is a straightforward process and only requires the unique ID of the task you wish to remove.

#### Syntax

```
citrine delete <task_id>
```

`<task_id>` is a mandatory field that specifies which task should be deleted. Each task has a unique, sequential ID that identifies it within the Citrine system.

#### Example Usage

To delete a math homework task with ID 42 from your list of tasks, you would use `citrine delete` as follows:

```
citrine delete 42
```

This command will permanently remove the task with ID 42 from the system. Please note that once a task is deleted, it cannot be recovered. Use this command with caution to avoid accidentally deleting important tasks.

It's always good practice to verify the details of the task you intend to delete before executing this command to ensure that no critical tasks are lost inadvertently.

### Listing Tasks with CitrineCLI

The `citrine list` command allows users to display a list of tasks in the Citrine task manager.

#### Syntax

```
citrine list 
```
to apply search or filter the list you can for example pipe it into grep:
```
citrine list | grep "priority: 9"
```
```
citrine list | grep "[x]" // completed tasks
citrine list | grep "[ ]" // open tasks
citrine list | grep "[>]" // in-progress tasks
citrine list | grep "[!]" // overdue tasks
```

```
citrine list | grep "2024-02-14" // find all tasks due valentines day.
```
