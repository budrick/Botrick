use std::collections::HashMap;

use matchit::Router;
use anyhow::Result;

type CommandHandler = fn(&str);

fn main() -> Result<()> {

    let mut commandlist:HashMap<&str, CommandHandler> = HashMap::new();
    commandlist.insert("toast", |params| {
        println!("Params: {:#?}", params);
    });

    let c = commandlist.get("toast").expect("Cmd not found");
    c("the text over here");
    
    // let mut r = Router::new();
    // r.insert("/test/:param", move || {
    //     println!("Sthing");
    // })?;
    // let m = r.at("/test/thing")?;
    // (m.value)();
    Ok(())
}