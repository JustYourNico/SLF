use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct Data {
    topics: Vec<&str>,
}

impl Data {
    fn new() -> Data {
        return Data {
            topics: vec!(),
        }
    }
}

fn main() {
    println!("Stadt - Land - Fluss");
    menu();
}

fn menu() {
    println!("availiable commands:");
    println!(" - create [port]");
    println!(" - join ");
    print!("> ");
    let mut cmd = String::new();
    read_line(cmd);
    if cmd.starts_with("create") {
        let argv: Vec<&str> = cmd.split(" ").collect();
        create_menu(argv);
    }
    else if cmd.starts_with("join") {
       join_menu(); 
    }
}

fn create_menu(argv: Vec<&str>){

    //ServerStart
    
    let mut data:Data = Data::new(); 

    //display IP
    println!("game is currently only availabe for local networks");
    println!("there are services like vpns or Hamatchi to");
    println!("simulate a local network availabe for free");
    println!("availiable commands:");
    println!(" - start     (start game) ");
    println!(" - players   (shows connected players) ");
    println!(" - config    (choose config file) ");
    println!(" - add_topic (add topic to list) ");
    println!(" - remove_topic (remove topic from list) ");
    println!(" - exit      (exit game) ");
    print!("> ");
    let mut com = String::new();
    read_line(com);

    // matching only the first char to safely ignore eventual args
    match com.trim().chars().nth(0).unwrap() {
        's' => {},
        'p' => {},
        'c' => {},
        'a' => add_topic(&mut data.topics),
        'r' => remove_topic(&mut data.topics),
        _ => {
            println!("not a valid input!")
        }
    }
}

fn join_menu(){
        
    println!("Enter IP Address to join a game:");
    print!("> ");
    let mut ip = String::new();
    read_line(ip);
}

fn add_topic(topics: &mut Vec<&str>) {
    print!("Enter topic: ");
    let mut topic: 
}

fn remove_topic(topics: &mut Vec<&str>) {
    unimplemented!();
}

fn read_line(s: &mut String) {
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("How about a string?");
}

// config file for default topics - between multiple selectable
// make addition / removal of topics in game possible
// save all topics from last game to new config file
// calculate points (uniqe, multiple, invalid)
// skip command for the case that nobody can finish
// make letter exclusion possible
// boolean setting: auto submit when finished
// validate inputs (correct first letter, more than 1 letter)
// don't let user submit if not finished
//
 // | 1 Stadt | 2 Land | 3 Fluss | 4 xyz |
 // | b------ | b----- | b------ | b---- |
 //
 // > 1 Bremen

