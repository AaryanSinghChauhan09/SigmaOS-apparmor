use anyhow::Result;
use clap::Parser;
use apparmor::apparmor;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = apparmor::Config { debug: cli.debug, ..Default::default() };
    let mut component = apparmor::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-apparmor running (Ctrl+C to stop)");
    tokio::runtime::Runtime::new()?.block_on(async {
        tokio::signal::ctrl_c().await.unwrap();
    });
    
    component.stop()?;
    Ok(())
}
