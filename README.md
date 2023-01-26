notify
======

A simple command line utility to monitor commands, then produce a desktop
notification when they've terminated.

For the longest time, one of the biggest limitations to improving my own
productivity has been identifying when long running tasks have completed,
regardless of success. This project aims to improve productivity by enabling
users to asynchronously work on other things and be notified when a process has
completed.

Historically I have used the `notify-send` command like `echo do something &&
notify-send -u critical cmd "Command is finished"` but this pattern only works
if I remember to run the command with a `&& notify-send...`. For processes that
are already running there are patterns like `wait` in bash, but that only works
if the process is running in the background of your active shell which is almost
never the case for me.

With this utility you can monitor a new command or any existing command if you
have the PID.

Copyright 2023 Luke Campbell
See LICENSE for details.


Usage
-----

To monitor a new process:

```
notify -- <cmd> [<args>...]
```

To monitor an existing process:

```
notify -p <pid>
```

That's it.


Building
--------

```
cargo build
```

Install
-------

```
cargo install --path .
```
