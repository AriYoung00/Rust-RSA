extern crate num;
// extern crate azul;

mod primes;
mod test;
mod rand;
mod rsa;
mod vis;
mod cli;
mod io;

fn main() {
    cli::init_cli_interface();
}