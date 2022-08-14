use std::collections::HashSet;

use clap::{Parser, Subcommand, Args};
use tracing::{Level, trace};
use tracing_subscriber::FmtSubscriber;
use scrapie_rs::{cfg::app_cfg, app, pages::namespace::Selector};

#[derive(Parser)]
#[clap(name = "srapie-rs")]
#[clap(author = "Peter Stanko <peter.stanko@gmail.com>")]
#[clap(version = "0.0.1-dev")]
#[clap(arg_required_else_help(true))]
#[clap(about = "Scrape the contents of webpages", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the web server
    Serve {
        /// set web server addr
        #[clap(short = 'A', long, value_parser)]
        addr: Option<String>,
    },
    /// Scrape webpages
    Scrape(ScrapeArgs),
    /// Print list of available webpages
    PrintPages(ScrapeArgs),
    /// Print configuration
    PrintConfig,
}

#[derive(Args)]
struct ScrapeArgs {
    /// positional args - names of the pages to scrape
    names: Vec<String>,   

    /// select by the tags
    #[clap(short = 'T', long, value_parser)]
    tags: Vec<String>,

    /// select the category by it's name
    #[clap(short = 'C', long, value_parser, default_value_t = String::from("food"))]
    catetory: String,   
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let conf = app_cfg::load_config()?;
    configure_logging();

    let app = app::Application::new(conf);

    match &args.command {
        Some(Commands::Serve{ addr}) => {
            trace!("run web server");
            let addr = addr.as_ref()
                .cloned()
                .unwrap_or(String::from(":8080"));
            println!("Listening on address: {}", addr);
        },
        Some(Commands::Scrape(params)) => {
            trace!("scrape webpages content");
            println!("Scrape called for: {:?}", params.names)
        },
        Some(Commands::PrintConfig) => {
            trace!("printing config");
            println!("{}", serde_yaml::to_string(app.config())?);
        },
        Some(Commands::PrintPages(params)) => {
            trace!("printing pages");
            let sel = Selector { 
                    category: params.catetory.clone(), 
                    names: HashSet::from_iter(params.names.iter().cloned()), 
                    tags:  HashSet::from_iter(params.tags.iter().cloned()),
                };
            for page in app.pages(&sel) {
                // FUTURE: use some nice tabular output
                println!("{} - {}", page.codename, page.homepage);
            }
        },
        None => {
        }
    }
    Ok(())
}

fn configure_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

