use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;
use std::fs;

struct Pair(String,String);
fn P1() -> Result<(),Error> {
    let mut no_bytes=0; let mut ind_bytes=0; let mut mx_bytes=0; 
    let mut no_chars=0; let mut ind_chars=0; let mut mx_chars=0;
    
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    let mut bytes_line=String::from("");
    let mut chars_line=String::from("");

    for line in reader.lines() {
        match line {
            Ok(line) => {
                no_bytes=line.len();
                no_chars=line.chars().count();
                if no_chars>mx_chars{
                    mx_chars=no_chars;
                    chars_line=line.clone();
                }
                if no_bytes>mx_bytes{
                    mx_bytes=no_bytes;
                    bytes_line=line.clone();
                }
                println!("{}", line.clone());
            },
            Err(err) => return Err(err),
        }
    }
    println!("\n");
    println!("Most bytes>\n{}",bytes_line.clone());
    println!("Most chars>\n{}",chars_line.clone());

    Ok(())
    
    
    
    
    
    /* 
    let path_to_read = Path::new("file.txt");

    let file = File::open(&path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            print!("{} : {}", num, line?.to_uppercase());
        }
*//*
    
    let mut ind=0;
    let mut no_bytes=0; let mut ind_bytes=0; let mut mx_bytes=0; 
    let mut no_chars=0; let mut ind_chars=0; let mut mx_chars=0;
    for i in data.chars(){
        if i=='\n' {
            if no_bytes>mx_bytes{
                mx_bytes=no_bytes;
                ind_bytes=ind;
            }
            if no_chars>mx_chars{
                mx_chars=no_chars;
                ind_chars=ind;
            }
            ind=ind+1;
        }
        else {
            
        }
    }
    Ok(()) */
}


fn P3()-> Result<(),Error> {
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    
    let mut vec: Vec<(String,String)> = Vec::new();
    let mut pereche=(String::from(""),String::from(""));
    for line in reader.lines() {
        match line {
        Ok(line) => {
            let arr=line.split_whitespace();
            let mut ind=0;
            for word in arr{
                if ind==0 {pereche.0=String::from(word.clone());}
                if ind==1 {pereche.1=String::from(word.clone()); vec.push(pereche.clone());}
                ind=ind+1;
            }
        },
        Err(err)=> return Err(err),
    
        }
     }
     
    let file1 = File::open("new.txt")?;
    let reader = BufReader::new(file1);
    let data = fs::read_to_string("new.txt").expect("Unable to read file");
    
     let mut Modified:String=String::from("");
     let words=data.split_whitespace();
    for wrd in words{
        let mut flag=0;
        for iter in vec.clone() {

            if wrd.eq(iter.1.as_str()) {
                Modified=Modified+iter.0.clone().as_str();
                flag=1;
                break;
            }
        }
        if flag==0 {
            Modified=Modified+wrd;
        }
        Modified=Modified+" ";
     }
     Modified.pop();
     Modified=Modified+".";
     print!("{}\n",Modified);
    Ok(())
}

fn P4() -> Result<(),Error> {
    let file = File::open("of.txt")?; ///am stocat aici ce aveam in C:\Windows\System32\drivers\etc\hosts 
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        
        match line {
        Ok(line) => {
    		let arr=line.split_whitespace();
            let mut comm=0;
            let mut ind=0;
            let mut Str1:String=String::from("");
            let mut Str2:String=String::from("");
            for word in arr{
                if ind==0 && word.eq("#") {
                    comm=1;
                }
                else if Str1.len()==0 && word.len()>0 {
                    Str1=String::from(word);
                }
                else if Str2.len()==0 && word.len()>0{
                    Str2=String::from(word);
                }
                ind=ind+1;
		    }
            if comm==0 && ind>0{
                print!("{} => {}\n", Str2, Str1);
            }
	    },
        Err(err)=> return Err(err),
    
        }
     }
     Ok(())
}
struct AppError {
    code: usize,
    message: String
}
fn print_error(x: AppError) {
    eprintln!("Uite eroarea> {}\n",x.message);
}

fn P2() ->Result<(),AppError> {

    let data = fs::read_to_string("rot13.txt").expect("Unable to read file");
    let mut Modified:String=String::from("");
     
    for ch in data.chars(){
        if ch.is_ascii() {
            let mut ascii_ref=ch as u32;
            ascii_ref=(ascii_ref+13);
            if ascii_ref>123{ //lower ramane lower
                ascii_ref=97+ascii_ref-123;
            }
            else if ascii_ref-13<=90 && ascii_ref>90 { //upper ramane upper
                ascii_ref=65+ascii_ref-91;
            }
            let mut new_char:char=(ascii_ref as u8) as char;
            //print!("{} este {}\n",ch,new_char);
            print!("{}",new_char);
            //Modified=Modified+ch.to_string().as_str();
        }
        else {
            return Err(AppError{code:1,message:String::from("Codul contine caractere non-ASCII"),});
        }
        
     }
     
    Ok(())
}

fn main() {
    P2();
}
