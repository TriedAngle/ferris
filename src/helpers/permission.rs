use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::UserId;

pub async fn check_administrator(
    ctx: &Context,
    msg: &Message,
    user_id: Option<UserId>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let channel = msg.channel(ctx).await.unwrap().guild().unwrap();
    let permissions = match channel
        .permissions_for_user(ctx, user_id.unwrap_or(msg.author.id))
        .await
    {
        Ok(permissions) => permissions,
        Err(e) => {
            println!("error {:?}", e);
            return Ok(false);
        }
    };
    println!("{:?}", permissions);
    if permissions.administrator() {
        Ok(true)
    } else {
        msg.channel_id
            .say(ctx, "You are not authorized to do that")
            .await?;
        Ok(false)
    }
}
