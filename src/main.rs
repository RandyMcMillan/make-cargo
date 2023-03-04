// use std::path::PathBuf;
// use std::fs;
use std::env;
// use serde_json;
// use regex::Regex;
use webview_official::{SizeHint, WebviewBuilder};
use rust_embed_for_web::{EmbedableFile, RustEmbed};
#[derive(RustEmbed)]
#[folder = "${PWD}/public"]
struct Asset;



// fn increase(number: i32) {
//     println!("{}", number + 1);
// }
// 
// fn decrease(number: i32) {
//     println!("{}", number - 1);
// }

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

// fn determinate_file_size(file: &str) -> u64 {
//     fs::metadata(file).unwrap().len()
// }
// 
// fn determinate_is_it_file_or_dirctory(arg: &str) -> &str {
//     let file = "File";
//     let dir = "Directory";
//     let re = Regex::new(r"/").unwrap();
//     if re.is_match(arg) {
//         return dir;
//     }
//     return file;
// }
// 
// fn get_current_working_dir() -> std::io::Result<PathBuf> {
//     env::current_dir()
// }

fn main() {

	let index = Asset::get("index.html").unwrap().data();
	let _contents = std::str::from_utf8(index.as_ref()).unwrap();
	// println!("Index file: {}", _contents);

    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("make-cargo")
        .width(1024)
        .height(768)
        .resize(SizeHint::NONE)
        .init("window.x = 42")
        .dispatch(|w| {
            w.set_size(800, 600, SizeHint::MIN);
            println!("Hello make-cargo!!!");
        })
        // .url("https://github.com/RandyMcMillan/make-cargo")
        .build();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'make-cargo'. Try passing some arguments!");
			// default web page
            webview.navigate("https://github.com/RandyMcMillan/make-cargo");
			webview.run(); },
        // one argument passed
        2 => {
            let url = &args[1];
                webview.navigate(url);
				webview.run(); },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let url = &args[2];
            match &cmd[..] {
                "open" => { webview.navigate(url);
				webview.run(); }
                _ => {
                    eprintln!("error: invalid command");
                    help(); },
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
