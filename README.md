# Overview

Similar to linux `find` it walks the given directory and returns a list of files and directories.

# Basic Usage

```
walk
```

By default this returns a list of files and directories ignoring hidden files/directories and paths matching any matchers found in .gitignore and .agignore files

# Advanced Usage

```
walk --all                    # no files will be ignored
walk --directories-only       # only returns the directories found
walk --files-only             # only returns the files found
walk ${HOME}                  # walk your home directory
```


## TODO Overall

- Speed up using threads
