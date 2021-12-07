<h1>Sortery</h1>

Sortery is a basic command-line file and directory sortor for Linux, written in [Rust](https://github.com/rust-lang/rust). It can be compiled using [Cargo](https://github.com/rust-lang/cargo). To get help on using Sortery, simply run `sortery -h` or `sortery --help`.

<h2>Installation</h2>

To install Sortery, simply run `install.sh` from any directory, and Sortery will be installed in that directory. If you move the Sortery
directory, you will have to install it again.

<h2>Dependencies</h2>
<ul>
  <li>
    <a href="https://crates.io/crates/cargo">Cargo</a>
  </li>
  <li>
    <a href="https://crates.io/crates/clap">Clap 2.34.0</a>
  </li>
  <li>
    <a href="https://crates.io/crates/chrono/0.4.19">Chrono 0.4.19</a>
  </li>
  <li>
    <a href="https://crates.io/crates/colored">Colored 2.0.0</a>
  </li>
  <li>
    <a href="https://crates.io/crates/walkdir">Walkdir 2.3.2</a>
  </li>
</ul>

<h2>Version</h2>

0.2.6

<h2>License</h2>

[MIT](https://github.com/SamMatzko/Sortery/blob/master/LICENSE-MIT.txt)

<h1>Usage</h1>

No matter what sorting commands you use, two of the arguments passed to `sortery` are mandatory: the source directory and the target directory. It varies depending on how you're sorting, but the basic principles are this: Sortery gets files and/or dirs from the source directory, and sorts them into the target directory. Here is a simple example that moves all the contents of one directory to another directory:

```
sortery --extract /home/user/my_dir1 /home/user/my_dir2
```

In this case, Sortery gets all its files and directories from `/home/user/my_dir1`, and moves them into `/home/user/my_dir2`. You can see the [Sortery Wiki](https://github.com/SamMatzko/Sortery/wiki) for full documentation on the command-line arguments and usage.
