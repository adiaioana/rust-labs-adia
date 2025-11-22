//use std::fmt::Display;
use std::fmt;
/*
impl fmt::Display for Option<u16>{
    fn fmt(&self, formatter: &mut fmt::Formatter)-> fmt::Result {
        match *self{
        Some(&self) => write!({},self),
        None => write!("can't"),
        }
    }
} */
fn is_prime(x: u32) -> bool {
    if x == 2 {
        return true;
    }
    if x < 2 {
        return false;
    }

    let mut d = 3;
    if x % 2 == 0 {
        return false;
    }
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d = d + 2;
    }
    true
}

/// ex1
fn next_prime(mut x:&mut u16) -> Option<u16> {

    let mut y: u32=(*x) as u32;
    let mut ok: u16=0;
    
    loop{
        y=y+1;
        let z:u16= y as u16;
        if (z as u32)!=y{
            return None;
        }

        if is_prime(y)==true{
            *x=y as u16;
            return Some(y as u16);
        }
    }
    return None;
}
/// ex2
fn checked_addition(mut x:u32, mut y: u32)-> Option<u32>{
    let d:u64=(x as u64)+(y as u64);
    let e:u32=x+y;
    if (d as u32)!=e{
        panic!("Addition exceeds data type ranges\n");
        return None;
    }
    return Some(e)
}
fn checked_multiplication(mut x:u32, mut y: u32)-> Option<u32>{
    let d:u64=(x as u64)*(y as u64);
    let e:u32=x*y;
    if (d as u32)!=e{
        panic!("Multiplication exceeds data type ranges\n");
        return None;
    }
    return Some(e)
}

/// ex3
fn checked_addition_result(mut x:u32, mut y: u32)-> Result<u32, &'static str>{
    let d:u64=(x as u64)+(y as u64);
    let e:u32=x+y;
    if (d as u32)!=e{
        return Err("Addition exceeds data type ranges\n");
    }
    return Ok(e)
}
fn checked_multiplication_result(mut x:u32, mut y: u32)->  Result<u32, &'static str>{
    let d:u64=(x as u64)*(y as u64);
    let e:u32=x*y;
    if (d as u32)!=e{
        
        return Err("Multiplication exceeds data type ranges\n");
    }
    return Ok(e)
}
//ex4
struct AppError {
    code: usize,
    message: String
}
/*
1> character is not ascii
2>character is not a digit
3> character is not a base16 digit
4> character is not a letter
5> character is not printable *//* 
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            1 => "Sorry, character is not ASCII!",
            2 => "Sorry, character is not digit!",
            3 => "Sorry, character is not a base16 digit!",
            4=> "Sorry, character is not a letter",
            5=> "Sorry, character is not printable",
            _ => "Sorry, no error",
        };

        write!(f, "{}", err_msg)
    }
}*/
/* 
fn produce_error_for_test(x:AppError) ->Result<(i32),AppError> {
    
    return Err(AppError{code:1,message:String::from("not ascii")});

    Ok(1 as i32);
}*/
fn print_error(x: AppError) {
    eprintln!("Uite eroarea> {}\n",x.message);
}

/*
1> character is not ascii
2>character is not a digit
3> character is not a base16 digit
4> character is not a letter
5> character is not printable */
fn check_for_errors(mut x:char) ->Result<char,AppError>{
    if !x.is_ascii() {
        return Err(AppError{
            code: 1, message:String::from("not ascii"),
        })
    }
    if !x.is_digit(10) {
        return Err(AppError{
            code: 2, message:String::from("not digit"),
        })
    }
    if !x.is_ascii_hexdigit(){
        return Err(AppError { code: 3, message:String::from("not base16 digit"),});
        
    }
    if !x.is_ascii_alphabetic() {
        return Err(AppError { code: 4, message:String::from("not letter"), });
    }
    if !x.is_numeric() {
        return Err(AppError { code: 5, message:String::from("not printable"), });
    }
    Ok(x)
}
fn checked_to_uppercase(mut x:&mut char) -> Result<(char),AppError>{
    if !x.is_alphabetic() {
        return Err(AppError { code: 4, message:String::from("not letter")  });
    }
    x.to_ascii_uppercase();
    return Ok(*x);
}
fn checked_to_lowercase(mut x:&mut char) -> Result<(char),AppError>{
    if !x.is_alphabetic() {
        return Err(AppError { code: 4, message:String::from("not letter") });
    }
    x.to_ascii_uppercase();
    return Ok(*x);
}
fn checked_char_to_number(mut x:&mut char) -> Result<(char),AppError>{
    if !x.is_ascii() {
        return Err(AppError { code: 1, message:String::from("not ascii") });
    }
    return Ok(*x);
}//
fn checked_print_char(mut x:&mut char) -> Result<(char),AppError>{
    if !x.is_numeric(){
        return Err(AppError { code: 5, message:String::from("not printable")});
    }
    print!("{}\n",*x);
    return Ok(*x);
}
fn main() {
    /* 
    let mut a=15;
    let mut ans_: Option<u16>=Some(0);
   /* while let Some(i)=next_prime(& mut a){
        print!("uite > {}\n",a);
    }
 */
    let mut a:u32=3;
    let mut b:u32= 5;
    if let Some(i)=checked_addition(a,b){
        print!("uite rez adunarii> {}\n", a+b);
    }
    if let Some(i)=checked_multiplication(a,b){
        print!("uite rez inmultirii> {}\n", a*b);
    }
    a=3;
    b=4294967290;
    println!("\n");
    if let Ok(i)=checked_addition_result(a,b){
        print!("uite rez adunarii cu result> {}\n", a+b);
    }
    if let Ok(i)=checked_multiplication_result(a,b){
        print!("uite rez inmultirii cu result> {}\n", a*b);
    }
*/
    println!("\n");
    let mut x='0';
    if let Ok(i)=checked_to_lowercase(&mut x) {
        eprintln!("to lowercase> {}\n",x);
    }
}
