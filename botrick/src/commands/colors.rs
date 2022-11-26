use crate::bot::Sender;
use crate::bot::command::Command;
use crate::bot::Config;
use crate::color::*;

pub fn colors(command: Command, sender: Sender, _config: Config) {
    let mut colstring = String::new();
    
    for col in 0..99 {
        colstring
        .push_str(colorize(Color::Num(col), None, format!("{:02}", col).as_str()).as_str());
        
        if (col + 1) % 20 == 0 {
            colstring.push_str("\r\n");
        } else {
            colstring.push(' ');
        }
    }
    
    let _ = sender
    .send_privmsg(command.channel, colstring);
}
