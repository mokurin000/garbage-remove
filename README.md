# Garbage-Remove

Auto clean tracker files and garbage files.

Thread-based, portablity-oriented.

## Example Configuration

```toml
paths = [
    "/storage/emulated/0/Pictures/.gs",
    "/storage/emulated/0/Pictures/.gs_fs0",
]
globs = [
    "/storage/emulated/0/*/.thumbnails/.database_uuid",
]
allow_relative_path = false

[interval]
secs = 30
nanos = 0
```