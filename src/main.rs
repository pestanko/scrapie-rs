use clap::{Parser, Subcommand, Args};
use tracing::{Level, trace};
use tracing_subscriber::FmtSubscriber;
use scrapie_rs::{cfg::app_cfg, pages::categories::load_category};

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
        #[clap(short = 'A', long, action)]
        addr: Option<String>,
    },
    /// Scrape webpages
    Scrape(ScrapeArgs),
    /// Print configuration
    PrintConfig {},
    PrintPages {},
}

#[derive(Args)]
struct ScrapeArgs {
    /// select by the name
    #[clap(short = 'N', long, action)]
    name: Option<String>,
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut conf = app_cfg::load_config()?;
    configure_logging();
    match &args.command {
        Some(Commands::Serve{ addr}) => {
            trace!("run web server");
            let addr = addr.as_ref()
                .cloned()
                .unwrap_or(String::from(":8080"));
            conf.web.addr = addr.clone();
            println!("Listening on address: {}", addr);
        },
        Some(Commands::Scrape(params)) => {
            trace!("scrape webpages content");
            println!("Scrape called for: {:?}", params.name)
        },
        Some(Commands::PrintConfig {}) => {
            trace!("printing config");
            println!("Config: {:#?}", conf);
        },
        Some(Commands::PrintPages {}) => {
            trace!("printing pages");
            let cat_name = conf.categories.get(0).expect("TODO: Fixme");
            let cat = load_category(cat_name)?;
            for page in &cat.pages {
                println!("{}", page.codename);
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

