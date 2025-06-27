# Organizer

A fast, flexible CLI tool to organize and deduplicate files across directories using customizable filters and actions.

## Features
- Detects duplicate files by name, size, date created, date modified, or image content
- Supports copy, delete, and verbose actions
- Recursive directory scanning
- Skip self-reference to avoid matching files with themselves

## Installation
```sh
cargo install --path .
```

## Usage
```sh
organizer [OPTIONS] <TARGETS>... <REFERENCE>
```

**Arguments:**
- `<TARGETS>...`        Target directories/files to search for duplicates
- `<REFERENCE>`         Reference directory/file to match against

**Options:**
- `--by <BY>`           Filters to match by (default: NAME,SIZE,IMAGE_CONTENT,SKIP_SELF)
- `--action <ACTION>`   Actions to perform: VERBOSE, DELETE, COPY=/path (default: VERBOSE)
- `-r, --recursive`     Scan directories recursively
- `--mode <MODE>`       Mode of operation for actions (default DUPLICATES)
- `-h, --help`          Print help
- `-V, --version`       Print version

**Available Filters:**
- `NAME` - Match by file name (case-insensitive)
- `SIZE` - Match by file size
- `DATE_CREATED` - Match by creation date
- `DATE_MODIFIED` - Match by modification date
- `IMAGE_CONTENT` - Match by image content hash
- `SKIP_SELF` - Skip files that are the same as the reference

**Available Actions:**
- `VERBOSE` - Print found duplicates to console
- `DELETE` - Delete duplicate files
- `COPY=/path` - Copy duplicate files to specified directory

**Available Modes**
- `DUPLICATES` - Perform actions on found duplicates of the reference
- `REFERENCE` - Perform action on the reference if duplicates are found
- `UNIQUES_REFERENCE` - Perform action on the reference if duplicates are not found

**Example:**
```sh
organizer --by NAME,SIZE --action VERBOSE ./photos ./backup ./reference
```

## TODO
- [ ] Improve performance of image comparison
