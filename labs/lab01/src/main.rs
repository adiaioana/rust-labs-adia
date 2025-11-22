fn main() {
    let mut x = 0;
    loop {
        if isprime(x) {
            println!("{} is prime", x);
        }

        x = x + 1;
        if x == 101 {
            break;
        }
    }
    println!("_______________________");

    let mut y: i32;
    x = 0;
    loop {
        y = 0;
        loop {
            if coprime(x, y) {
                println!("{} & {} are coprime", x, y);
            }
            y = y + 1;
            if y == 101 {
                break;
            }
        }
        x = x + 1;
        if x == 101 {
            break;
        }
    }

    x = 99;
    loop {
        bottles_of_beer(x);
        x = x - 1;
        if x == 0 {
            break;
        }
    }
}
// exercitiul 1
fn isprime(x: i32) -> bool {
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

//exercitiul 2
fn coprime(x: i32, y: i32) -> bool {
    if x * y == 0 {
        return false;
    }

    let mut a = x;
    let mut b = y;
    let mut r: i32 = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }
    if b == 1 {
        return true;
    }
    false
}

//exercitiul 3
fn bottles_of_beer(x: i32) {
    if x > 2 {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.", x,x,x-1);
    } else if x == 2 {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottle of beer on the wall.", x,x,x-1);
    } else {
        println!("1 bottle of beer on the wall,\n1 bottle of beer.\nTake one down, pass it around,\nNo bottles of beer on the wall.");
    }
}
