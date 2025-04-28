# Garbage-Remove

INotify based high performance garbage cleaning solution for Android.

> Note: this will remove any matched file immediately, don't use it to clean caches.

To install as a service in Magisk/KernelSU, see [garbage-remover](https://github.com/mokurin000/garbage-remover).

## Example Configuration

```toml
paths = [
    "/storage/emulated/0/Pictures/.gs",
    "/storage/emulated/0/Pictures/.gs_fs0",
]
globs = [
    "/storage/emulated/0/*/.thumbnails/.database_uuid",
]

```
