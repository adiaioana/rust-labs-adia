use std::env::vars;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;
struct AppError {
    code: usize,
    message: String,
}
fn print_error(x: AppError) {
    eprintln!("Uite eroarea> {}\n", x.message);
}

fn start_init( arr:&mut Vec<(String,i32)>) {
    arr.push((String::from("ping"),1 as i32));
    arr.push((String::from("count"),2 as i32));
    arr.push((String::from("run"),3 as i32));
    arr.push((String::from("stop"),4 as i32));

}

fn type_of_command(x:String)->i32{
    let mut of:Vec<(String,i32)>=Vec::new();
    let mut arr:&mut Vec<(String,i32)>=& mut of;
    start_init(arr);
    for it in arr.clone(){
        if it.0==x {
            return it.1;
        }
    }
    0
}
fn to_command(x:String) -> Command{
    let mut res:Command =Command { head: (String::from("1")), args: vec![vec![' ';0];0], tp: (0) };
    let mut ok=true;
    for word in x.split(' ') {
        if word.len()==0 {
            continue;
        }
        if ok {
            res.head=String::from(word).clone();
            res.tp=type_of_command(res.head.clone());
            ok=false;
        }
        else {
            let mut vec:Vec<char>=Vec::new();
            for ch in word.chars(){
                vec.push(ch);
            }
            res.args.push(vec);
        }
    }
    res
}
#[derive(Debug, Clone)]
struct Command{
    head: String,
    args: Vec<Vec<char>>,
    tp: i32,
}
struct Ping;

impl Ping {
    fn exec(&mut self) {
        println!("ping");
    }
}
trait MyFc{
    fn get_name(&mut self, x:String) -> String;
    fn exec(&mut self,x:String) -> Command;
}

impl MyFc for Command{

    fn get_name(&mut self,x:String) -> String {
        *self=to_command(x);
        return (*self).head.clone();
    }
    fn exec(&mut self,x:String) -> Command{
        *self=to_command(x);
        if self.tp==1{
            Ping.exec();
        }
        return (*self).clone();
    }
}

fn P1()-> Result<(), Error>{

    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut C:Command=Command { head: (String::from("1")), args: vec![vec![' ';20];20], tp: (0) };
                let name_of=C.get_name(line);
                if C.tp>0{
                    match C.tp {
                        1 => {Ping.exec()},
                        2 => {C.},
                        _ => println!("Comanda nerec")

                    }
                   // println!("{}>",name_of);
                    /*for cuv in C.args {
                        println!("{:?} ", cuv);
                    }*/
                }
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

struct Terminal{
    comms:Vec<Command>,
}
impl Terminal {
    fn new() -> Self {
        Terminal {comms:Vec::new()}
    }

}

fn main() {
    P1();
    let mut terminal = Terminal::new();
/* 
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();*/
}
