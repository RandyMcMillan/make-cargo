use std::env;
use webview_official::{SizeHint, WebviewBuilder};

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("TEST")
        .width(1024)
        .height(768)
        .resize(SizeHint::NONE)
        .init("window.x = 42")
        .dispatch(|w| {
            w.set_size(800, 600, SizeHint::MIN);
            println!("Hello World");
        })
        .url("https://google.com")
        .build();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'make-cargo'. Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
			webview.run();
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }

    webview.eval("console.log('The anwser is ' + window.x);");
    let w = webview.clone();
    webview.bind("xxx", move |seq, _req| {
        w.r#return(seq, 0, "{ result: 'We always knew it!' }");
    });
}
