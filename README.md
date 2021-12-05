<h1>Coral</h1>

Coral is a basic command-line file and directory sortor, written in [Rust](https://github.com/rust-lang/rust). It can be compiled using [Cargo](https://github.com/rust-lang/cargo). To get help on using Coral, simply run `coral -h` or `coral --help`.

Coral is licensed under the GNU General Public License, version 3. This protects your right to copy, modify, and distribute Coral according to the terms of the
license. See `GPL-license_v3.txt` for full terms and information.

<h2>Installation</h2>

<h3>For Linux</h3>

To install Coral, simply run `install.sh` from any directory, and Coral will be installed in that directory. If you move the Coral
directory, you will have to install it again.

<h3>For other operating systems</h3>

`install.sh` does not work for Windows or Mac, but Coral can still be easily installed without it. Put the Coral directory where you want it to be installed. Open a terminal, and change to the Coral directory. Run `cargo build`. Now, create an executable shell script named `coral` that runs `target/debug/coral`.
Now you're ready to start sorting!

<h2>Requirements/Dependencies</h2>
<ul>
  <li>
    <a href="https://github.com/rust-lang/cargo">Cargo</a>
  </li>
</ul>

<h2>What you can do</h2>

I don't have ready access to a Windows or a Mac, and I don't use either one, so anyone who's willing to contribute to install scripts for those two OSs is encouraged to do so. I would like Coral to be available to as many people as possible. Right now, the install script works only on Linux.

<h2>Version</h2>

0.1.2

<h2>License</h2>

[MIT](https://github.com/SamMatzko/Coral/blob/master/LICENSE-MIT.txt)

<h1>Usage</h1>

<h2>Basic Principles</h2>

No matter what sorting commands you use, the first two arguments passed to Coral are mandatory: the source directory and the target directory. It varies depending
on how you're sorting, but the basic principles are this: Coral gets files and/or dirs from the source directory, and sorts them into the target directory. Here
is a simple example that moves all the contents of one directory to another directory:

```
coral /home/user/my_dir1 /home/user/my_dir2 --extract
```

In this case, Coral gets all its files and directories from `/home/user/my_dir1`, and moves them into `/home/user/my_dir2`.

<h2>Commands-line Arguments</h2>

How to order the command-line arguments when using Coral:

```
coral [SOURCE] [TARGET] [--help] <args>
```

Below is a table showing all the command-line arguments available for Coral, and how they can be used. These arguments replace `<args>` in the example above.

| Usage | Argument        | What it does           |
| ----- | --------------- | ---------------------- |
|   #   | -e, --extract   | Extracts all the files and dirs from `SOURCE` into `TARGET`. |
|   *   | -h, --help      | Show the help message. |

The `Usage` column shows how the specific argument can be used. Arguments that can be passed with any other arguments without affecting them are marked with a `*`. Arguments that can only be run by themselves and/or with their specific sub-arguments are marked with a `#`. Sub-arguments, arguments that can only be passed
if a certain "parent argument" is passed as well, are marked with a `&` and the short version of their "parent argument". For example, a sub-argument of the `-e` command would have `&-e` in its `Usage` column.
