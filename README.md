# oak

Like the tree command, but with icons, descriptions, and far less functionality

## Quickstart

Without any flags or arguments, the output of `oak` is the same as `tree -n --noreport`, but with icons.

```
$ oak src/testdata
```

## Descriptions

`oak` uses extended file attributes to print descriptions of files and folders.
To add descriptions, write the `com.oak.description` attribute to a file or folder, for instance using `xattr`:

```
$ xattr -w com.oak.description "This is a file" example/afile.txt
$ xattr -w com.oak.description "This is a folder" example/afolder
$ oak example
example
├── afile.txt # This is a file
└── afolder   # This is a folder
```