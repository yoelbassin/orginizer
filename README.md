# Organizer

A fast, flexible CLI tool to organize and deduplicate files across directories using customizable filters and actions.

## Features
- Detects duplicate files by name, size, date, or image content
- Supports copy, delete, and verbose actions
- Recursive directory scanning
- Exclude files/folders with glob patterns
- Progress bar and dry-run mode

## Installation
```sh
cargo install --path .
```

## Usage
```sh
organizer [OPTIONS] <TARGETS>... <REFERENCE>
```

**Options:**
- `--by`         Filters to match by (default: NAME,SIZE,IMAGE_CONTENT,SKIP_SELF)
- `--action`     Actions to perform: VERBOSE, DELETE, COPY=/path (default: VERBOSE)
- `--exclude`    Comma-separated glob patterns to exclude
- `-r, --recursive`  Scan directories recursively
- `--dry-run`    Show what would be done without making changes
- `--verbose`    Enable verbose output

**Example:**
```sh
organizer --by NAME,SIZE --action DELETE ./photos ./backup ./reference
```

