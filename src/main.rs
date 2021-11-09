use rand::{thread_rng, Rng};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::{Message, ReactionType};

#[group]
#[commands(ping)]
struct General;

struct Handler;

//😀
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        let x: u32 = thread_rng().gen_range(0x1F600..0x1F64F);
        let emoji = char::from_u32(x).unwrap_or('💔');
        message
            .react(ctx, ReactionType::Unicode(emoji.to_string()))
            .await
            .expect("whgooops");
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "OTA3MzcyNjgxNTcyMjYxOTI4.YYmOqQ.Hvn0fMdVLT8bMCjaw0N7a6FJbJY";
    //env::var("OTA3MzcyNjgxNTcyMjYxOTI4.YYmOqQ.Hvn0fMdVLT8bMCjaw0N7a6FJbJY").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
