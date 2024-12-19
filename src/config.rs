use clap::Parser;

#[derive(Parser)]
struct Params {
    #[arg(short = 'N')]
    n_zeros: u32,

    #[arg(short = 'F')]
    n_results: u32,
}

pub fn get_params() -> (u32, u32) {
    let params = Params::parse();

    (params.n_zeros, params.n_results)
}
