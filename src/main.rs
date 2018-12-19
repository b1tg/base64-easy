#![allow(unused)]
fn main() {
    let s = String::from("Man");

    let chars = s.as_bytes();
    let mut target: [i32; 4] = [0; 4];

    let mut b = String::from("");
    for x in chars {
        b.push_str(&format!("{:08b}", x));

    }
    println!("{}",&b);

    let mut len = b.len();

    let mut loop_t = len / 6;
    let remainder = len % 6;

    if remainder != 0 {
        //"{number:>0width$}", number=1, width=6
        b.push_str(&format!("{n:>0width$}",n=0,width=(6-remainder)));
        println!("{}",&b);
        loop_t += 1;
    }
    
    println!("{},{},{}",len, loop_t, remainder);
    //100110 111000 011101 110
    //10011011 10000111 01110

    //b6 -> b8

    //let mut vec = Vec::new();
    let mut rrr = String::from("");
    let mut n =1;
    while n <= loop_t {

        let slice = &b[(6*(n-1))..(6*n)];
        println!("{}", slice);

        let intval = u8::from_str_radix(slice, 2).unwrap();
        println!("{}", intval); 

        //vec.push(intval);
        println!("{}",base64map(intval)); 
        //vec.push(base64map(intval));
        rrr.push_str(&base64map(intval));
       
        
        n+=1;
    }

    if (remainder == 2) {
        rrr.push_str("==");
    } else if (remainder == 4) {
        rrr.push_str("=");
    }

    //println!("{:?}",vec);
    println!("{:?}",rrr);

    //let result = String::from_utf8(vec).unwrap();
    //println!("{}",&result);
    
    //assert_eq!(&[77,97,110], s.as_bytes());
}




fn base64map<'a>(index: u8) -> String {

    let mut offset = 0;
    if (index >=0 && index < 26) {
        offset = 65;
    } else if (index >=26 && index < 52) {
        offset = 97-26;
    } else if (index >=52 && index < 62) {
        offset = 80-52;
    } else if (index == 62 ) {
        return "+".to_owned();
    } else if (index == 63 ) {
        return "/".to_owned();
    }
    let result = format!("{}",(index + offset) as char);
    result

}
