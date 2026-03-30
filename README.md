# GDCleaner

Simple asynchronous tool to clean up dependency and compilation folders.

## How it works

GDCleaner asynchronously walks all subdirectories of the specified path, spawning a task for each subdirectory. A semaphore limits the number of concurrent tasks to avoid overwhelming the system.

Once discovery is complete, the program presents each folder to delete one by one and prompts for confirmation.

## Usage

```
./gdcleaner [path]
```

For each discovered folder, you will be prompted:

```
Delete /some/project/target ? [Y/n/path]
```

- `Y` — delete the folder
- `n` — skip
- `path` — print the full path and prompt again

## Options

| Option | Description |
|--------|-------------|
| `--force` | Delete without asking for confirmation. **Use with caution.** |
| `--only <language>` | Only target folders for the specified language (see `config.toml` for available languages) |
| `--skip_size` | Skip folder size calculation. The program will run faster. |

## Configuration

Languages, their identifier files, and target folders are defined in `config.toml`:

```toml
[[lang]]
name = "Rust"
identifier = ["Cargo.toml"]
target = ["target"]

[[lang]]
name = "nodejs"
identifier = ["package.json"]
target = ["node_modules"]
...
```

- `identifier` — files used to detect the language in a directory
- `target` — folders to flag for deletion when the language is detected

## Install on your system
```
cargo install gdcleaner
```
