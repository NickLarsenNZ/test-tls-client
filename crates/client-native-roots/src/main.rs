#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(url) = std::env::args().nth(1) {
        reqwest::get(url).await?;
    } else {
        eprintln!("Please supply a URL as the first argument");
        std::process::exit(1);
    };

    println!("Request successful");

    Ok(())
}
