mod config;
mod weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match config::parse_argv() {
        Err(e) => {
            eprintln!("{:#?}", e);
            std::process::exit(1);
        }
        Ok(my_config) => {
            config::run(my_config).await?;
        }
    }

    Ok(())
}
