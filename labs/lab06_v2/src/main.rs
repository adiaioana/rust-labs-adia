use std::env::vars;
use std::fs;
use std::fs::File;
use std::process::exit;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;
struct AppError {
    code: usize,
    message: String,
}
fn print_error(x: AppError) {
    eprintln!("Uite eroarea> {}\n", x.message);
}
trait Command{
    fn get_name(&self) -> String;
    fn exec(&mut self, x:String);
}
struct PingCommand;

struct PongCommand;
struct CountCommand;
struct TimesCommand{
    count: usize,
}

impl Command for PongCommand{
    fn get_name(&self) -> String {
        return String::from("pong");
    }
    fn exec(&mut self, x:String) {
        println!("ping");
    }
}
impl Command for PingCommand{
    fn get_name(&self) -> String {
        return String::from("ping");
    }
    fn exec(&mut self, x:String) {
        println!("pong");
    }
}
impl Command for CountCommand{
    fn get_name(&self) -> String {
        return String::from("count");
    }
    fn exec(&mut self, x:String) {
        let mut nr=0;
        for cuv in x.split(' ').clone(){ 
            if cuv.len()==0{
                continue;
            }

            for ch in cuv.chars(){
                if ch!=' ' && ch!='\t' && ch!='\n' {
                    nr=nr+1;
                    break;
                }
            }
        }
        println!("Count result is {}",nr);
    }
    
}
impl Command for TimesCommand{
    fn get_name(&self) -> String {
        return String::from("times");
    }
    fn exec(&mut self, x:String) {
        self.count=self.count+1;
        println!("Times was called for {} TIMES", self.count);
    }
}

struct Terminal{
    commands:Vec<(String,Box<dyn Command>)>,
}

trait Terminal_Fc{
    fn new() -> Self;
    fn register(&mut self, x:Box<dyn Command>);
    fn run(&mut self) -> Result<(), Error>;
    fn check(x:&str) -> bool;
}
static mut going:i32=1;
fn stop() {
    println!("Ne-am oprit");
    unsafe{going=0;}
}

impl Terminal_Fc for Terminal {
    
    fn check(x:&str) -> bool {
        true
    }
    fn new() -> Self {
        Terminal { commands: Vec::new(), }
    }
    fn register(&mut self, x:Box<dyn Command>) {
        let nume=(*x).get_name();
       // println!("T> Am inregistrat {}", nume);
        self.commands.push({(nume,x)});
    }
    fn run(&mut self) -> Result<(), Error> {

        let mut first_TC=TimesCommand{count:0};
        let file = File::open("commands.txt")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            unsafe{
                if going==0{
                    break;
                }
            }
            match line {
                Ok(line) => {
                    let mut ind=0;
                    let mut ok:bool= false;
                    let mut head_str:String=String::from("");
                    let mut comm_str:String=String::from("");
                    for word in line.split(' '){
                        if ind==0 {
                            let head=word;
                            
                            let mut has_chars=0;
                            for ch in word.chars() {
                                if ch!=' ' && ch!='\t' && ch!='\n' {
                                    has_chars=1;
                                }
                            }
                            if has_chars==1{
                           // println!("Ln> Comanda este {}", head);
                            ok=false;
                            for it in self.commands.iter() {
                                //println!("Am comparat {} cu {} ", (*it).0, head);
                                if (*it).0==head {
                                    ok=true;
                                }
                            }
                            if ok==false{
                                //exit();
                                if head=="stop" {
                                    ok=true;
                                }
                            }
                            head_str=String::from(head);
                            ind=1;
                           // println!("OK este {}", ok);
                            }
                        }
                        else {
                            comm_str.push_str(word);
                            comm_str.push(' ');
                        }
                    }
                    if ok==true{
                        if head_str=="ping" {
                            PingCommand.exec(comm_str);
                        }
                        else if head_str=="count" {
                            CountCommand.exec(comm_str);
                        }
                        else if head_str=="times" {
                            first_TC.exec(comm_str);
                        }
                        else if head_str=="stop" {
                            stop();
                            //exit(1);
                        }
                        else if head_str=="pong" {
                            PongCommand.exec(comm_str);
                        }
                        }
                    }
                
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(PongCommand {}));

    terminal.run();
}
