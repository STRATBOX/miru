// dependencies
use color_eyre::Result;
use dotenv::dotenv;

// module definitions

// use module dependencies
use miru::run;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    run().await?;

    Ok(())
}
