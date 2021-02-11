use dialoguer::{theme::ColorfulTheme, Input, Password};
use booru::{
    error::BooruError,
    config::{Config, AccountConfig},
    http::BooruClient,
};
use crate::commands::utils::Spinner;

pub fn login() -> Result<(), BooruError>{
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .interact_text()?;
    
    println!(include_str!("text/apikey.txt"));

    let api_key: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("API key")
        .interact()?;

    let spinner = Spinner::start("Checking");

    let conf = Config::default()
        .account(AccountConfig { username, api_key });

    let client = BooruClient::from_config(&conf)?;

    println!("{:#?}", client.profile());
    
    conf.save()?;

    spinner.stop("Success!");

    Ok(())
}
