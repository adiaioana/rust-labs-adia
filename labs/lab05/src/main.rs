
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
fn to_nr(numar: String) -> i32 {
    let mut nr = 0;
    let digits = numar.chars().collect::<Vec<char>>();
    for i in 0..digits.len() {
        nr = nr * 10 + (digits[i] as i32) - ('0' as i32);
    }
    return nr;
}

fn P1() -> Result<(), Error> {
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);
    let mut varsta_min = 999;
    let mut varsta_max = -1;
    let mut varsta = 0;
    let mut nume_min = String::from("");
    let mut nume_max = String::from("");
    let mut vec: Vec<(String, String)> = Vec::new();
    let mut pereche = (String::from(""), String::from(""));
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let arr = line.split(',');
                let mut ind = 0;
                let mut lista: [String; 10] = Default::default();
                for word in arr {
                    lista[ind] = String::from(word.clone());
                    ind = ind + 1;
                }
                let varsta = to_nr(lista[2].clone());
                if varsta < varsta_min {
                    varsta_min = varsta;
                    nume_min = lista[0].clone();
                }
                if varsta > varsta_max {
                    varsta_max = varsta;
                    nume_max = lista[0].clone();
                }
            }
            Err(err) => return Err(err),
        }
    }
    print!("Elevul cel mai batran rau de tot {}\n", nume_max);
    print!("Elevul cel mai tanar da cu ce pret {}\n", nume_min);
    Ok(())
}
#[derive(Debug, Copy, Clone)]
struct Canva{
    pixels:[[char;50];10],
}
fn new_canvas() -> Canva {

    let mut c:Canva=Canva{pixels:([[' ';50];10])};
    c
}
fn set_pixels(C:&mut Canva,mat:&[(i32,i32,i32)]) {
    for nr in mat{
        let mut ind=0; let mut x=0; let mut y=0;
            x=nr.0;
            y=nr.1;
            C.pixels[x as usize][y as usize]=nr.2 as u8 as char;
    }
}
fn print(C:Canva){
    for lines in C.pixels{
        println!("{:?}",lines);
    }
}
fn P2(){
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);

}
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    phone: String,
    age: u32,
}
#[derive(Debug, Deserialize)]
struct People{
    ppl: [Person;2]
}
fn P3() {
    let mut name_min=String::from("");
    let mut age_min=199;
    let mut name_max=String::from("");
    let mut age_max=0;
    let content = fs::read_to_string("person.json").unwrap();
    let p:People= serde_json::from_str(&content).unwrap();
    for per in p.ppl {
        if per.age< age_min{
            age_min=per.age;
            name_min=per.name.clone();
        }
        if per.age>age_max{
            age_max=per.age;
            name_max=per.name.clone();
        }
    }
    println!("mARELE e {}", name_max);
    println!("mICUL e {}", name_min);
}

fn main() {
    P3();
}
