<h1 style='text-align: center'>
  mo<img src='./docs/Rust.svg' alt='R' height='24' />se
</h1>

Command-line tool to translate Morse code to Latin and vice versa.

## Usage

    morse <latin|morse>

### Examples

- Morse code to Latin


      morse '... --- ...   ... --- ...   ... --- ...'
      SOS SOS SOS

- Latin to Morse code


      morse SOS SOS SOS
      ... --- ...   ... --- ...   ... --- ...

## Make Dependencies

- git
- rustc
- cargo

## Build & Run

1. Acquire the git repository with `git clone https://github.com/rasmusmerzin/morse`.

1. Change directory to `morse` running `cd morse`.

1. Compile with cargo `cargo build` or compile & run `cargo run <arguments>`.

1. Run with `target/debug/morse <arguments>`.
