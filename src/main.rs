mod commands;
use std::env;
use poise::serenity_prelude as serenity;
use serenity::GatewayIntents;
#[cfg(test)]
mod tests;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

// User data, which is stored and accessible in all command invocations
struct Data {
    compliments: Vec<String>,
    insults: Vec<String>,
    answers: Vec<String>
}

// Declare shared types, shortens command context and error types
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn load_list(path: &Path) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in buf_reader.lines() {
        let line = line?;
        let line = line.trim();

        if !line.is_empty() {
            lines.push(line.to_owned());
        }
    }
    
    if lines.is_empty() {
        return Err(format!("{} contains no data", path.display()).into());
    }

    Ok(lines)
}

#[tokio::main]
async fn main() {
    // Loads local .env variables if file is present 
    dotenvy::dotenv().ok();

    // Setup logger
    tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()),
    )
    .init();

    // Configure the client with your toke
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    // Load files for responses
    let data_dir = PathBuf::from(
    env::var("DATA_DIR")
        .unwrap_or_else(|_| "./data".to_owned()),
    );

    let data = Data {
        compliments: load_list(&data_dir.join("compliments.txt"))
            .expect("Failed to load compliments"),
        insults: load_list(&data_dir.join("insults.txt"))
            .expect("Failed to load insults"),
        answers: load_list(&data_dir.join("answers.txt"))
            .expect("Failed to load insults"),
    };
    
    // Set gateway intents, which decides what events the bot will be notified about
    // You do not need any if you are only responding to slash commands
    let intents = GatewayIntents::empty();

    // Builds commands with poise
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
           commands: commands::all(),
           ..Default::default()
        })
         // Registers the commands with discord
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        }).build();

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
        let mut client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await
            .expect("Failed to create discord client");

        if let Err(why) = client.start().await {
            tracing::error!(error = %why, "Discord client stopped");
        }
}
