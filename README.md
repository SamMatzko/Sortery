<h1>Sortery</h1>

Sortery is a basic command-line file and directory sortor, written in [Rust](https://github.com/rust-lang/rust). It can be compiled using [Cargo](https://github.com/rust-lang/cargo). To get help on using Sortery, simply run `sortery -h` or `sortery --help`.

<h2>Installation</h2>

<h3>For Linux</h3>

To install Sortery, simply run `install.sh` from any directory, and Sortery will be installed in that directory. If you move the Sortery
directory, you will have to install it again.

<h3>For other operating systems</h3>

`install.sh` does not work for Windows or Mac, but Sortery can still be easily installed without it. Put the Sortery directory where you want it to be installed. Open a terminal, and change to the Sortery directory. Run `cargo build`. Now, create an executable shell script named `sortery` that runs `target/debug/sortery`.
Now you're ready to start sorting!

<h2>Requirements/Dependencies</h2>
<ul>
  <li>
    <a href="https://github.com/rust-lang/cargo">Cargo</a>
  </li>
</ul>

<h2>What you can do</h2>

I don't have ready access to a Windows or a Mac, and I don't use either one, so anyone who's willing to contribute to install scripts for those two OSs is encouraged to do so. I would like Sortery to be available to as many people as possible. Right now, the install script works only on Linux.

<h2>Version</h2>

0.2.3

<h2>License</h2>

[MIT](https://github.com/SamMatzko/Sortery/blob/master/LICENSE-MIT.txt)

<h1>Usage</h1>

<h2>Basic Principles</h2>

No matter what sorting commands you use, the first two arguments passed to Sortery are mandatory: the source directory and the target directory. It varies depending
on how you're sorting, but the basic principles are this: Sortery gets files and/or dirs from the source directory, and sorts them into the target directory. Here
is a simple example that moves all the contents of one directory to another directory:

```
sortery /home/user/my_dir1 /home/user/my_dir2 --extract
```

In this case, Sortery gets all its files and directories from `/home/user/my_dir1`, and moves them into `/home/user/my_dir2`. You can see the [Sortery Wiki](https://github.com/SamMatzko/Sortery/wiki) for full documentation on the command-line arguments and usage.
