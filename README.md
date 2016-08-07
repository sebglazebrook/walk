# Overview

Similar to linux find it walks the given directory and returns a list of files and directories.

# Why not just use find?

I built this to experiment with concurrent programming ideas so when walking a large directory structure it will run a lot quicker than the linux find command.

# Usage

```
finder
```

```
finder /usr/local/bin
```

```
--ignore /pattern/    # good idea to bad ??
--ignore-dir dir_name
--ignore-dirs
--ignore-files
--follow-symlinks
```

