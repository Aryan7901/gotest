# gotest
A testing utility wrapper for Go written in Rust and Bash

# Usage

### With no args

This will test all files in the the directory and give the overall coverage in percentage

```
gotest
```

### With directory name specified

This will test all files in the specified directory

```
gotest <dir-name>
```

## Flags

```
-h --html    To create html output
-o           To open the html output (to be used with -h)
-ho          Combines both of the above flags
```
