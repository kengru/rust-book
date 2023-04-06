# rb-14 Creating workspaces.

## Things learned

- Workspaces are used when you have to manage multiple packages
  for a same project, for example, having two libraries and one
  binary.
- If on a workspace all the libraries will output the compiled
  artifacts on the parent's `target` directory.
- When running a space from another directory you can add the
  `-p` flag to `cargo run` and add the name of the package.
