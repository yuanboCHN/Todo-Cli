use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(name = "todo", about = "A simple CLI todo manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add
    Add { content: String },
    /// list
    List,
    /// done
    Done { id: u32 },
}

#[derive(Serialize, Deserialize)]
struct Todo{
    id: u32,
    content: String,
    done: bool,
}

fn data_path() -> PathBuf{
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".todo.json")
}

fn load() -> Vec<Todo>{
    let path = data_path();
    match fs::read_to_string(&path){
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save(todos: &[Todo]) {
    let path = data_path();
    let text = serde_json::to_string_pretty(todos).expect("serialize failed");
    fs::write(&path, text).expect("write failed");
}

fn main() {
    let cli = Cli::parse();

    let mut todos = load();
    let mut next_id: u32 = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    match cli.command {
        Commands::Add { content } => {
            let todo = Todo{
                id: next_id,
                content,
                done: false,
            };

            println!("添加:[{}] {}", todo.id, todo.content);
            todos.push(todo);
            next_id += 1;
            save(&todos);
        }
        Commands::List => {
            if todos.is_empty(){
                println!("暂无待办");
                return;
            }
            for t in &todos{
                let mark = if t.done{
                    "x"
                } else {" "};
                println!("[{}] {}.{}", mark, t.id, t.content);
            }
            
        }
        Commands::Done { id } => {
            match todos.iter_mut().find(|t| t.id == id){
                Some(t) =>{
                    t.done = true;
                    println!("完成：[{}] {}", t.id, t.content);
                    
                }
                None => println!("找不到id={}的待办", id),
            }
            save(&todos);
        }
    }
}