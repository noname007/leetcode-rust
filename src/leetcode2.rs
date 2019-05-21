pub fn run() {
    println!("Hello, world!");
   println!("{:#?}", reverse(0x3fffffff));
   println!("{:#?}", reverse(-2147483412));
   println!("{:#?}", reverse(-1463847412));
   println!("{:#?}", reverse(1463847412));
}

pub fn reverse(x: i32) -> i32 {
    let mut num = x;

    let mut rev = 0;

    let max_i32 = 0x7fff_ffff;



//    let min_i32 = 0x8000_0000 ;
    let min_i32 = -2147483648 ;


//    println!("{}", max_i32);
//    println!("{}", max_i32/10);
//    println!("{}", min_i32/10);

    while num != 0 {

        if num > 0 {
            if rev > max_i32/10 {
                return 0;
            }
        }else if num < 0{
            if  rev < min_i32 /10 {
                return 0;
            }
        }

        rev = rev * 10 + num % 10;
//        println!("{}", rev);
        num /= 10;
    }

    rev
}