use serenity::client::Context;
use serenity::model::channel::Message;

const BAD_WORDS_TESTING: &'static [&'static str] = &["fuck", "fucking", "shit", "dumb"];

pub fn check(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return
    }
    let mut warnings: Vec<String> = Vec::new();
    check_bad_words(&msg, &mut warnings);
    check_go(&msg, &mut warnings);
    check_lol(&msg, &mut warnings);

    if !warnings.is_empty() {
        let mut warning_collected: String = String::new();
        for warning in &warnings {
            warning_collected += warning;
            warning_collected += "\n"
        }

        if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("{}...\n{}", msg.author, warning_collected));
            m.embed(|e| {
                e.title("stop :(")
                    .image("https://www.dropbox.com/s/hz23pzn8ur1mao6/mad-rustacean.png?dl=1")
            })
        }) {
            println!("Error sending message: {:?}", e);
        }
    }
}

pub fn check_bad_words(msg: &Message, warnings: &mut Vec<String>) {
    for bad_word in BAD_WORDS_TESTING {
        if msg.content.contains(bad_word) {
            warnings.push(format!("'{}' is a bad word :(", bad_word))
        }
    }
}

pub fn check_go(msg: &Message, warnings: &mut Vec<String>) {
    if msg.content.contains("lol no generics") {
        warnings.push(format!(" 'lol no generics' | bashing go is not cool!").to_owned())
    }
}

pub fn check_lol(msg: &Message, warnings: &mut Vec<String>) {
    if msg.content.contains("yasuo is a great champion") {
        warnings.push(format!("you are just dumb wtf").to_owned())
    }
    if msg.content.contains("bronze") {
        warnings.push(format!("no elo discrimination").to_owned())
    }
}