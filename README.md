# Organizer
A fast, flexible CLI tool to organize and deduplicate files across directories using customizable filters and actions.

This is a file management and organization tool I created for my needs, and happy to share it with anyone with similar problems.

## A little background
My home media backup was huge, and scattered across many HDDs around my house. So when finally I decided to organize it, the archive had around 140,000 photos (approximately 50k per drive), with many duplications over different filenames and metadata. Not only that, but one of the cameras we owned would always reset it's date to 2000/01/01 - causing an even bigger headache.

I created this tool to help me organize those files, with means of speed, ease of use and reliability.


## Features
- Detects duplicate files by name, size, date created, date modified, or image content
- Supports copy, delete, and verbose actions
- Recursive directory scanning
- Skip self-reference to avoid matching files with themselves

## Installation
```shell
cargo install --path .
```

## Usage
```
organizer <COMMAND> [OPTIONS] [<ARGS>]

Commands:
    dup     File duplication finder
    find    File search following defined filters

    -r --recursive
            Scan directories recursively
    -v --verbose
            Verbose
    -h --help
            Show this message
    --action <ACTION>
            Actions to perform: VERBOSE, DELETE, COPY=/path (default: VERBOSE)

dup:
    dup applies defined actions to found duplicates
    organizer dup [OPTIONS] <TARGETS...> <REFERENCE>
    
    Available Modes:
        - DUPLICATES - Perform actions on found duplicates of the reference
        - REFERENCE - Perform action on the reference if duplicates are found
        - UNIQUES_REFERENCE - Perform action on the reference if duplicates are not found

find:
    organizer find [OPTIONS] --filters <FILTER...> <TARGETS>


Available Filters:
    - NAME - Match by file name (case-insensitive)
    - SIZE - Match by file size
    - DATE_CREATED - Match by creation date
    - DATE_MODIFIED - Match by modification date
    - IMAGE_CONTENT - Match by image content hash
    - SKIP_SELF - Skip files that are the same as the reference

Available Actions:
    - VERBOSE - Print found duplicates to console
    - DELETE - Delete duplicate files
    - COPY=/path - Copy duplicate files to specified directory
```


**Example:**
```shell
organizer --by NAME,SIZE --action VERBOSE ./photos ./backup ./reference
```

## TODO
- [ ] Somehow create a intuitive way to compare exif year
- [ ] Make actions
- [ ] Improve performance of image comparison
