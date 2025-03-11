A simple cli using clap.

## help
```console
cargo run -- help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\hello-clap.exe help`
A hello world cli

Usage: hello-clap.exe <COMMAND>

Commands:
  echo     echo a msg
  sum-two  sum two numbers
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## sum-two
```console
cargo run -- sum-two 2 4
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\hello-clap.exe sum-two 2 4`
6

cargo run sum-two --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target\debug\hello-clap.exe sum-two --help`
sum two numbers

Usage: hello-clap.exe sum-two <NUM1> <NUM2>

Arguments:
  <NUM1>  
  <NUM2>  

Options:
  -h, --help  Print help
```

## echo 

```console
>cargo run -- echo --help
   Compiling hello-clap v0.1.0 (C:\Users\Mongoose\Desktop\Programing Files\Rust\rust-first-steps\hello-clap)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target\debug\hello-clap.exe echo --help`
echo a msg

Usage: hello-clap.exe echo [OPTIONS] <MSG>

Arguments:
  <MSG>  

Options:
  -l          include a new line or not
  -h, --help  Print help

>cargo run -- echo "this is a test"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\hello-clap.exe echo "this is a test"`
this is a test
>

>cargo run -- echo "this is a test" -l 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target\debug\hello-clap.exe echo "this is a test" -l`
this is a test

>
```