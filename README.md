This is a replica of the **Linux `grep` command-line utility**; the objective here is just to learn how to make a **Rust CLI application**. 

You can use **The Rust Programming Language book** to try to replicate it: https://doc.rust-lang.org/book/ch12-00-an-io-project.html.

This current version was prepared to be used as a **real CLI**, so you can use flags (**-h** and **-i**) just as you would use with `grep`. However, you can still check the code of the **unaltered version** in the first commit of this repository.

To use it, you can download the binary in the **release** section and install it to your system path, or simply by directing the command **right to the binary**:

`./minigrep to poem.txt -i`

If you use Cargo, you can **also** clone this application and install it using:

`cargo install --path .`