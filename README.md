# Citrine Task Management CLI
Citrine is a minimal and modular task management command-line interface designed to embody the UNIX philosophy of doing one thing well.

Citrine outputs results in a simple, text-based format that can be easily piped into other programs for further processing. This approach allows users to build their own customized workflows by chaining together commands using pipes (`|`) and other shell features.

For instance, a user could use Citrine's CLI to list tasks and then pipe the output to `sort` or `awk` for organizing tasks by priority or filtering tasks based on certain criteria. This flexibility is a core advantage of such a design – rather than being confined to the features provided by a monolithic application, users can leverage the entire ecosystem of UNIX command line tools.

Moreover, Citrine's CLI commands are designed to be non-interactive and stateless, which means they can be run as part of scripts without requiring user input during execution. This makes them ideal for automation via scripts or as part of larger systems using tools like `make` or even `systemd` timers.

Citrine is designed to keep track of tasks where they are being worked on, the `citrine` command can be called from every folder and creates a `.citrine` file there in order to keep track of your tasks.

## Quick Start
To see a list of subcommands available and how they can be used run `citrine --help` to get help with a specific command type sometihng like `citrine add --help`
## Installation
### Step by Step Installation for regular usage

### On Linux or Other Unix-Based Systems:

1. **Download and Locate the Binary:**
   - Obtain the latest release binary that matches your OS and architecture.

2. **Move the Binary:**
   - Use the `mv` command to move the downloaded binary to the `/usr/bin` directory. This step grants system-wide access to the executable.
   - Example:
     ```sh
     # Assuming you're in the directory where the binary is downloaded
     sudo mv citrine-linux-x86-64 /usr/bin/citrine
     ```

3. **Create a Symlink (Optional):**
   - Creating a symlink allows you to call the executable by a different name or from different locations.
   - Example:
     ```sh
     # Create a symlink to make the executable callable as 'citrine'
     sudo ln -s /usr/bin/citrine /usr/bin/citrine
     ```

By moving the binary to `/usr/bin`, it becomes accessible system-wide. Additionally, creating a symlink (as shown in the example) allows you to call the executable by the desired name (`citrine` in this case) from any location in the terminal.

Remember, the use of `sudo` might be necessary to perform actions that require administrative privileges, such as moving files to system directories (`/usr/bin` in this case).

Always ensure that the permissions are appropriately set to allow execution of the binary after placing it in the system directories.

### On Windows:

1. **Locate the Binary:**
   Find the downloaded binary file, for example, `citrine.exe`.

2. **Access Environment Variables:**
   - Right-click on "This PC" or "My Computer."
   - Select "Properties."
   - Click on "Advanced system settings" on the left panel.
   - In the System Properties window, click on the "Environment Variables" button.

3. **Edit PATH Variable:**
   - In the System Variables section, find and select the `Path` variable, then click "Edit."

4. **Add Binary Folder to PATH:**
   - Click "New" and add the full path to the folder containing the `citrine.exe` binary.
   - Click "OK" on all windows to apply the changes.

5. **Verify Installation:**
   - Open a new Command Prompt window (Win + R, type `cmd`, press Enter).
   - Type `citrine` and press Enter. If the installation was successful, the command should execute the program.

## Usage
### Available Commands:

- `add` - Add a new task to the task list.
- `update` - Update an existing task in the task list.
- `delete` - Delete an existing task from the task list.
- `list` - List all tasks in the task list.
- `help` - Print this message or the help of the given subcommand(s).

### Global Options:

- `-h, --help` - Print help.
- `-V, --version` - Print version.

## Add a Task

```shell
citrine add [OPTIONS] <TITLE>
```

### Arguments:

- `<TITLE>` - The title of the task.

### Options:

- `-d, --due <DUE_DATE>` - The due date of the task in rfc3339 format (`YYYY-MM-DDTHH:MM:SS+00:00`).
- `-p, --priority <PRIORITY>` - The priority of the task (0-9).
- `-t, --tags <TAGS>` - The tags of the task, comma-separated.
- `-h, --help` - Print help.

## Update a Task

```shell
citrine update [OPTIONS] <ID>
```

### Arguments:

- `<ID>` - The id of the task to update.

### Options:

- `-d, --due <DUE_DATE>` - The due date of the task in rfc3339 format.
- `-p, --priority <PRIORITY>` - The priority of the task (0-9).
- `-t, --tags <TAGS>` - The tags of the task, comma-separated.
- `-s, --status <STATUS>` - The status of the task.
- `-m, --message <TITLE>` - The title of the task.
- `-r, --remove-tags <REMOVE_TAG>` - Remove tags from the task.
- `-a, --append-tags <APPEND_TAG>` - Append tags to the task.
- `-h, --help` - Print help.

## Delete a Task

```shell
citrine delete <ID>
```

### Arguments:

- `<ID>` - The id of the task to remove.

### Options:

- `-h, --help` - Print help.

## List Tasks

```shell
citrine list
```

No arguments or options are required to list all task items. Use `citrine list` to see an overview of all tasks.

*Note: Replace `<ID>`, `<TITLE>`, `<DUE_DATE>`, `<PRIORITY>`, `<TAGS>`, `<STATUS>`, `<REMOVE_TAG>`, and `<APPEND_TAG>` with the relevant information when executing commands.*



## Advanced Usage
For more advanced user's here are some examples of how some of the output issued by citrine can be used together with other cli tools in order to create some interesting behavior.

```sh
# get all high priority tasks
citrine list | grep -E 'priority: [789]'
# get medium priority tasks
citrine list | grep -E 'priority: [456]'
# get low priority tasks
citrine list | grep -E 'priority: [123]'
```
```sh
# get all tasks due today
citrine list | grep "$(date +%Y-%m-%d)"
# count the number of completed tasks
citrine list | grep "[x]" | wc -l
# setting relative deadlines using the GNU date command
citrine add -d "$(date --rfc-3339=seconds --date='next Friday')" "Task Title"
```

