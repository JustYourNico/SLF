use std::io::{stdin, stdout, Write};

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
  read_line(&mut cmd);
  if cmd.starts_with("create") {
    let argv: Vec<&str> = cmd.split(" ").collect();
    start_server(argv);
    menu_loop();
  }
  else if cmd.starts_with("join") {
     join_menu(); 
  }
}

fn start_server(argv: Vec<&str>) {
    
}

fn menu_loop() {
  let mut topics: Vec<&str> = vec!();
  let mut players: Vec<&str> = vec!();
  let mut configpath: &str = "~/.config/slf/default";
  let brk: bool = false;

  while !brk {
      show_menu(&mut topics, &mut players, &mut configpath);
  }
}

fn show_menu(topics: &mut Vec<&str>, players: &mut Vec<&str>,
             configpath: &mut &str){
  //display IP
  println!("game is currently only availabe for local networks");
  println!("there are services like vpns or Hamatchi to");
  println!("simulate a local network availabe for free");
  println!("availiable commands:");
  println!(" - start       (start game) ");
  println!(" - players     (shows connected players) ");
  println!(" - config      (choose config file) ");
  println!(" - list_topics (add topic to list) ");
  println!(" - add_topic   (add topic to list) ");
  println!(" - remove_topic (remove topic from list) ");
  println!(" - exit        (exit game) ");
  print!("> ");
  let mut com = String::new();
  read_line(&mut com);

  // matching only the first char to safely ignore eventual args
  match com.trim().chars().nth(0).unwrap() {
    's' => start_game(&topics),
    'p' => list_players(&players),
    'c' => choose_config(configpath),
    'l' => list_topics(&topics),
    'a' => add_topic(topics),
    'r' => remove_topic(topics),
    _ => {
      println!("not a valid input!")
    }
  }
}

fn join_menu(){
  println!("Enter IP Address to join a game:");
  print!("> ");
  let mut ip = String::new();
  read_line(&mut ip);
}

fn start_game(topics: &Vec<&str>) {
  
}

fn list_players(players: &Vec<&str>) {
  
}

fn choose_config(configpath: &mut &str) {
  
}

fn list_topics(topics: &Vec<&str>) {
  for (i, topic) in topics.iter().enumerate() {
    println!("{}: {}", i, topic);
  }
}

fn add_topic<'a>(topics: &mut Vec<&'a str>) {
  print!("Enter topic: ");
  let mut empty: String = String::new();
  read_line(&mut empty);
  (*topics).push();
}

fn remove_topic(topics: &mut Vec<&str>) {
  for (i, topic) in topics.iter().enumerate() {
    println!("{}: {}", i, topic);
  }
  print!("Enter topic number to delete: ");
  let mut inx_str = String::new();
  read_line(&mut inx_str);
  let inx_str_clean: &str = inx_str.split(" ")
                                   .collect::<Vec<&str>>()[0]
                                   .trim();
  let inx: usize = usize::from_str_radix(inx_str_clean, 10)
                   .expect("Gimme gimme gimme a number or a pie!");
  topics.remove(inx);
}

fn read_line(s: &mut String) {
  let _ = stdout().flush();
  stdin().read_line(s).expect("How about a string?");
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

