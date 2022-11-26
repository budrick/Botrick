use crate::bot::Sender;
use crate::bot::command::Command;
use crate::bot::Config;
use crate::bot::get_url_title;
use crate::bot::get_urls;
use crate::color::*;

pub fn default(command: Command, sender: Sender, config: Config) {
    let urls = get_urls(command.params.as_str());
    if !config.inspect_urls || urls.is_empty() {
        return;
    }
    let title = get_url_title(urls[0].as_str());
    if title.is_none() {
        return;
    }
    let colbit = colorize(Color::Green, None, "LINK >>");
    let _ = sender
        .send_privmsg(
            &command.channel,
            format!("{} {}", colbit, title.unwrap()),
        );
}