mod config;
mod hash;
mod worker;

fn main() {
    let (n_zeros, n_results) = config::get_params();
    worker::run(n_zeros, n_results);
}
