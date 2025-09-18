use clap::Parser;

use plz::openai::get_response;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(num_args = 1.., trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.query.is_empty() {
        eprintln!("Error: you must provide a query after `--`.\nExample: plz -- why is rust awesome?");
        std::process::exit(1);
    }

    let question = args.query.join(" ");
    match get_response(&question).await {
        Ok(answer) => {
            println!("{}", answer);
        }
        Err(err) => {
            eprintln!("Error getting response: {:#}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
