# Remotes

Xvc uses _remotes_ to store content of the files.  
These remotes are different than Git remotes. 
They don't contain Git history of a repository, but they _can_ store contents of the files tracked by Xvc. 

A remote uses the same content-addresses to store the files. 
This means, if there is a file in Xvc repository that points to `/b3/1886572424...defa/0.png` in local cache, this path will be used to identify the content in remote as well. 

Additionally, Xvc stores _remote event logs_ that lists which operations are performed on that remote. 
By using these event logs, it's possible to identify what has gone on with remotes without checking the file lists. 
These event logs are also shared with the other users, and a user can identify which files are present in a remote even without a connection. 

# Basic Operations

All remotes should support the following operations: 

- **List** to list the files available in the remote.
- **Send** to upload files from local cache to a remote.
- **Receive** to download files from a remote to local cache.
- **Delete** to delete file from a remote.

All these operations record a distinct event to the event log. 

Events record the event, guid of the remote and the event content.

Event contents are like the following:

- **List** includes the listing got from the remote.
This means once a list is retrieved from the remote, it's available for local operations.
Most recent lists are starting point to determine files available in a remote.
- **Send** event contains the affected paths.
These paths are added to remote file list. 
- **Receive** event contains the affected paths. 
These paths are added to remote file list. 
- **Delete** to delete multiple files at once.
These paths are removed from remote file list. 

# Remote types
 
## Local Remotes

Although it looks like an oxymoron, a local remote is a directory in the local file system. 
It may be a mount point shared with others, or another disk that you may use for backups and sharing. 

- **List** uses `std::fs::listdir`.
- **Send** uses `std::fs::copy` with rayon.
- **Receive** uses `std::fs::copy` with rayon.
- **Delete** uses `std::fs::remove_file` with rayon.

## Generic Remotes

These remotes define commands for each of the operations listed above. 
It allows to run external programs such as `rsync`, `rclone`, `s5cmd`.
For such remotes, commands for the above operations must be defined and they will be run in separate processes. 

This remote type offloads the responsibility of commands to the user. 

The user is expected to supply the value following variables:

- `{URL}`: The url for the remote. 
This can be anything the commands to send/receive/list will accept. 
It's to build the paths with minor repeats. 
- `{REMOTE_DIR}`: You can separate the remote directory. 
- `{PATH}`: This is set by Xvc for each singular commands. 
It's a relative path to the local cache directory. 
- `{PROCESS_POOL_SIZE}`: This value is used to set the number of processes to perform operations.
Setting this to `1` makes all operations sequential. 

- `List Command`: A command to list the `{URL}`.
For example, for `rsync --list-only {URL}{REMOTE_DIR}` 
- `Send Command`: A command to send a file to `{URL}{REMOTE_DIR}`. 
It can use `{URL}` and should use `{PATH}` in the command. 
An example may be `rsync -a {PATH} {URL}{REMOTE_DIR}{PATH}`
- `Receive Command`: A command to receive a file from a remote. 
It can use `{URL}` and `{REMOTE_DIR}`, and should use `{PATH}` in the command. 
Example: `rsync -a {URL}{REMOTE_DIR}{PATH} {PATH}`
- `Delete Command`: A command to delete a file from the remote. 
It can use `{URL}` and `{REMOTE_DIR}`, and should use `{PATH}` in the command. 
Example: `ssh {URL} "rm {REMOTE_DIR}{PATH}"`

Generic remotes use these commands to create multiple processes to send/receive/delete files. 
It's not as fast as using other types because of the overhead involved, but it's flexibility is useful. 

## Rclone Remotes

Xvc can use your Rclone configuration to upload/download the data files.

## SSH Remotes

## S3 (and compatible) remotes

# Operations on Remotes 

## Creating a remote

## Moving a remote

## Deleting a remote


