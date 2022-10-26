# Storages

Xvc uses _storages_ to store content of the files.
These storages are different from Git remotes.
They don't contain Git history of a repository, but they _can_ store contents of the files tracked by Xvc.

A storage uses the same content-addresses used in Xvc cache to store the files.
For example, if there is a file in Xvc repository that points to `/b3/1886572424...defa/0.png` in local cache, this path will be used to identify the content in storage as well.

Additionally, Xvc stores _storage event logs_ that lists which operations are performed on that storage.
By using these event logs, it's possible to identify what has gone on with storages without checking the file lists.
These event logs are also shared with the other users, and a user can identify which files are present in a storage even without a connection.

# Basic Operations

All storages should support the following operations:

- **Init** to initialize a storage
- **List** to list the files available in the storage.
- **Send** to upload files from local cache to a storage.
- **Receive** to download files from a storage to local cache.
- **Delete** to delete file from a storage.

All these operations record a distinct event to the event log.

Events record the event, guid of the storage and the event content.

Event contents are like the following:

- **Init** creates the necessary directories and the guid file in a storage
- **List** includes the listing got from the storage.
  Once a list is retrieved from the storage, it's available for local operations.
  Most recent lists are starting point to determine files available in a storage.
- **Send** event contains the affected paths.
  These paths are added to storage file list.
- **Receive** event contains the affected paths.
  These paths are added to storage file list.
- **Delete** to delete multiple files at once.
  These paths are removed from storage file list.

# Storage types

## Local Storages

A local storage is a directory in the local file system.
It may be a mount point shared with others, or another disk that you use for backups and sharing.

- **Init** uses `std::fs::copy` to copy the GUID file to the appropriate directory
- **List** uses `std::fs::listdir`.
- **Send** uses `std::fs::copy` with rayon.
- **Receive** uses `std::fs::copy` with rayon.
- **Delete** uses `std::fs::remove_file` with rayon.

## Generic Storages

These storages define commands for each of the operations listed above.
It allows to run external programs such as `rsync`, `rclone`, `s5cmd`.
For such storages, commands for the above operations must be defined and they will be run in separate processes.

This storage type offloads the responsibility of exact operations to the user.

The user is expected to supply the value following variables:

- `{URL}`: The url for the storage.
  This can be anything the commands to send/receive/list will accept.
  It's to build the paths with minor repeats.
- `{STORAGE_DIR}`: You can separate the storage directory.
- `{PATH}`: This is set by Xvc for each singular commands.
  It's a relative path to the local cache directory.
- `{PROCESS_POOL_SIZE}`: This value is used to set the number of processes to perform operations.
  Setting this to `1` makes all operations sequential.

- `List Command`: A command to list the `{URL}`.
  For example, for `rsync --list-only {URL}{STORAGE_DIR}`
- `Send Command`: A command to send a file to `{URL}{STORAGE_DIR}`.
  It can use `{URL}` and should use `{PATH}` in the command.
  An example may be `rsync -a {PATH} {URL}{STORAGE_DIR}{PATH}`
- `Receive Command`: A command to receive a file from a storage.
  It can use `{URL}` and `{STORAGE_DIR}`, and should use `{PATH}` in the command.
  Example: `rsync -a {URL}{STORAGE_DIR}{PATH} {PATH}`
- `Delete Command`: A command to delete a file from the storage.
  It can use `{URL}` and `{STORAGE_DIR}`, and should use `{PATH}` in the command.
  Example: `ssh {URL} "rm {STORAGE_DIR}{PATH}"`

Generic storages use these commands to create multiple processes to send/receive/delete files.
It's not as fast as using other types because of the overhead involved, but it's flexibility is useful.

## Rclone Storages

Xvc can use your Rclone configuration to upload/download the data files.

## SSH Storages

## S3 (and compatible) storages

# Operations on Storages

## Creating a storage

## Moving a storage

## Deleting a storage
