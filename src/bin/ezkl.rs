use ezkl::commands::Cli;
use ezkl::execute::run;
use log::{error, info};
use rand::seq::SliceRandom;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::create();
    colog::init();
    banner();
    info!("{}", &args.as_json()?);
    let res = run(args);
    match &res {
        Ok(_) => info!("verify succeeded"),
        Err(e) => error!("verify failed: {}", e),
    };
    res
}

fn banner() {
    let ell: Vec<&str> = vec![
        "for Neural Networks",
        "Linear Algebra",
        "for Layers",
        "for the Laconic",
        "Learning",
        "for Liberty",
        "for the Lyrical",
    ];
    info!(
        "{}",
        format!(
            "
        ███████╗███████╗██╗  ██╗██╗
        ██╔════╝╚══███╔╝██║ ██╔╝██║
        █████╗    ███╔╝ █████╔╝ ██║
        ██╔══╝   ███╔╝  ██╔═██╗ ██║
        ███████╗███████╗██║  ██╗███████╗
        ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝

        -----------------------------------------------------------
        Easy Zero Knowledge {}.
        -----------------------------------------------------------
        ",
            ell.choose(&mut rand::thread_rng()).unwrap()
        )
    );
}
