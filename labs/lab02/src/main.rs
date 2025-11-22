
fn add_chars_n(mut str: String , ch:char, nr:i32) -> String{
    let mut ind:i32=0;
    loop{
        ind+=1;
        if ind>nr {
            return str;
        }
        str.push(ch);
    }
}
/*
add_space: concatenates n spaces to the string
add_str: concatenates a str to the string
add_integer: concatenates an integer to the string. Add separators at every 3 digits.
add_float: concatenates a float to the string
*/
fn int_to_string(mut str: &mut String , nr:i32){

    let mut aux: String = String::from("");
    let mut cif:i32;
    let mut ch:char;
    let mut ind:i32=0;
    let mut cnr=nr;
    
    while cnr>0 {
        cif=cnr%10;
        ch=(cif as u8 + '0' as u8) as char;
        if ind%3==0 {
            aux.push('_');
        }
        aux.push(ch);
        ind=ind+1;
        cnr=cnr/10;
    }

    ind=aux.len() as i32;
    ind-=1;
    while ind>0 {
        let b: u8 = aux.as_bytes()[ind as usize];
        let c: char = b as char;
        (*str).push(c);
        ind-=1;
    }
}

fn add_string(mut str1: &mut String,mut str2:String) {
    (*str1).push_str(&str2);
}
fn add_integer(mut str: &mut String , nr:i32){
    let mut strrandom:String= String::from("");
    int_to_string(&mut strrandom, nr);
    add_string(str, strrandom);
}

fn add_space(mut str: &mut String , nr:i32) {
    add_chars_n_with_ref(str, ' ', nr);
}
fn add_str(mut str1: &mut String, str_to_add: &str) {
    (*str1).push_str(&str_to_add);
}
fn add_float(mut str: &mut String , nr:f32) {
    let mut p10=1;
    let mut aux:String=String::from("");
    let mut cif:i32;
    let mut ch:char;
    let mut cnr=nr;
    //let mut nrcif=0;
    if cnr as i32==0{
        aux.push('0');
    }
    while cnr as  i32>0 {
        cif=(cnr as i32)%10;
        ch=(cif as u8 + '0' as u8) as char;
        aux.push(ch);
        cnr/=(10 as f32);
        //nrcif+=1;
    }
    let mut ind:i32=aux.len() as i32;

    ind-=1;
    while ind>=0 {
        let b: u8 = aux.as_bytes()[ind as usize];
        let c: char = b as char;
        (*str).push(c);
        
        ind-=1;
    }
    (*str).push('.');
    let mut ind_for_debug=0;
    p10=10;
    loop{
        let number_as_int: f32=nr*(p10 as f32);
       // print!("{} : nr as int e {} iar nr normal e {}\n", p10, number_as_int,(number_as_int as i32) as f32);
        //if nrcif<=0 {
            cif=((nr*p10 as f32) as i32)%10;
            ch=(cif as u8 + '0' as u8) as char;
            (*str).push(ch);
        //    print!("{} ; ch add e {} iar str e {}\n", p10, ch, *str);
        //}
        //ind_for_debug+=1;
        //nrcif-=1;
        p10*=10;
        if (number_as_int as i32) as f32 >= number_as_int as f32 {
            break;
        }
    }

}

fn add_chars_n_with_ref(mut str: &mut String , ch:char, nr:i32) {
    let mut ind:i32=0;
    loop{
        ind+=1;
        if ind>nr {
            return ;
        }
        (*str).push(ch);
    }
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
   /* while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_with_ref(&mut s, c, 26 - i);

        i += 1;
    } */
    add_space(&mut s, 20);
    let strrandom="I 💚\n";
    add_str(&mut s, strrandom);
    add_space(&mut s, 20);
    let strrandom1="RUST.\n\n";
    add_str(&mut s, strrandom1);
    add_space(&mut s, 5);
    add_str(&mut s, "Most");
    add_space(&mut s, 5);
    add_str(&mut s, "crate");
    add_integer(&mut s,306437968 );
    add_str(&mut s, "          and     lastest         is\n");
    add_str(&mut s, "downloaded        has             downloads     the         version    ");
    add_float(&mut s, 2.0);
    add_str(&mut s, ".\n");

  //  let x:f32=0.023;
    //add_float(&mut s, x);
    print!("{}", s);
}