# FRE

### NAME

- fre - Find Replace Edit

### SYNOPSIS

- fre [OPTIONS...] <pattern\> <replace\> <path\>
- fre <pattern\> [OPTIONS...] <replace\> <path\>
- fre <pattern\> <replace\> [OPTIONS...] <path\>
- fre <pattern\> <replace\> <path\> [OPTIONS...]

### DESCRIPTION

- Find patterns to replace and transform text in a given file or files.

### OPTIONS

- -r
  - Recursive (Recusively go through each file in the given directory.)
- -rf
  - Recursive Full (Recusively go through each file in the given directory and its subdirectories.)
- -e
  - Edit (Replace each matched pattern with the given text in place. Without this flag the result won't take effect in a file it will only print the result to stdout.)
- -d
  - Delete (Delete all lines where the pattern matches.)
- --help
  - Prints out the usage information.
