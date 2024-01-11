# Citrine CLI
Citrine is a minimal and modular task management command-line interface (CLI) that excels at doing one thing well, in line with the UNIX philosophy.

Citrine outputs results in a simple, text-based format, ideal for piping into other programs like `sort` or `awk` for task organization or filtering based on criteria. This design allows users to build customized workflows by chaining commands using pipes (`|`) and other shell features.

The CLI commands are non-interactive and stateless, perfect for scripting and automation with other tools such as `make` or `systemd` timers. Citrine creates a `.citrine` file in each working directory to track tasks locally.

## Quick Start
🌟 To see available subcommands and their usage, run `citrine --help`. For help with a specific command, type `citrine add --help`.

## Installation
### Step-by-Step Installation for Regular Usage

### On Linux or Other Unix-Based Systems:

1. **Download and Locate the Binary:**
   - Download the latest release binary suitable for your OS and architecture.

2. **Move the Binary:**
   - Move the downloaded binary to `/usr/bin` for system-wide access.
   - Example:
     ```sh
     sudo mv citrine-linux-x86-64 /usr/bin/citrine
     ```

3. **Create a Symlink (Optional):**
   - Make the executable easily callable with a symlink.
   - Example:
     ```sh
     sudo ln -s /usr/bin/citrine /usr/bin/citrine
     ```

By moving the binary to `/usr/bin`, you make it accessible everywhere. Use `sudo` if needed.

### On Windows:

1. **Locate the Binary:**
   Find `citrine.exe`.

2. **Access Environment Variables:**
   - Right-click "This PC" or "My Computer."
   - Select "Properties."
   - Click "Advanced system settings."
   - Click "Environment Variables."

3. **Edit PATH Variable:**
   - In "System Variables," select `Path` and click "Edit."

4. **Add Binary Folder to PATH:**
   - Click "New" and enter the path to `citrine.exe`.
   - Confirm with "OK."

5. **Verify Installation:**
   - Open a new Command Prompt.
   - Type `citrine` and hit Enter. If set up correctly, you should see Citrine's output.

## Usage
### Available Commands:

- `add` - Add a new task 📝.
- `update` - Modify an existing task ✏️.
- `delete` - Remove a task 🗑️.
- `list` - Display all tasks 📜.
- `help` - Show help information ℹ️.

### Global Options:

- `-h, --help` - View help.
- `-V, --version` - Show version number.

## Add a Task

```shell
citrine add [OPTIONS] <TITLE>
```

### Arguments:

- `<TITLE>` - The title of the task.

### Options:

- `-d, --due <DUE_DATE>` - The due date in RFC 3339 format (`YYYY-MM-DDTHH:MM:SS+00:00`) or naive date format (`YYYY-MM-DD`)
- `-p, --priority <PRIORITY>` - Priority level (0-9).
- `-t, --tags <TAGS>` - Associated tags, comma-separated.
- `-h, --help` - Display help.

## Update a Task

```shell
citrine update [OPTIONS] <ID>
```

### Arguments:

- `<ID>` - The ID of the task to be updated.

### Options:

- All options are the same as for `add`, with the addition of:
- `-s, --status <STATUS>` - The task's status.
- `-m, --message <TITLE>` - A new title.
- `-r, --remove-tags <REMOVE_TAG>` - Tags to remove.
- `-a, --append-tags <APPEND_TAG>` - Tags to append.
- `-h, --help` - Help.

## Delete a Task

```shell
citrine delete <ID>
```

### Arguments:

- `<ID>` - The ID of the task to be deleted.

### Options:

- `-h, --help` - Help.

## List Tasks

```shell
citrine list
```

Use `citrine list` to see an overview of all tasks.

*Replace placeholders with actual values when executing commands.*

## Advanced Usage
For more advanced users, here are some examples of how Citrine's output can be used in conjunction with other CLI tools to create interesting behaviors:

```sh
# Get all high priority tasks
citrine list | grep -E 'priority: [789]'
# Get medium priority tasks
citrine list | grep -E 'priority: [456]'
# Get low priority tasks
citrine list | grep -E 'priority: [123]'
```
```sh
# Get all tasks due today
citrine list | grep "$(date +%Y-%m-%d)"
# Count the number of completed tasks
citrine list | grep -c '[x]'
# Set relative deadlines using the GNU date command
citrine add -d "$(date --rfc-3339=seconds --date='next Friday')" "Task Title"
```
