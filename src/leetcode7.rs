pub fn run() {
    let i = 100_i8;
    u32::max_value();
//    println!("checked {:?}", i.checked_add(i));
//    println!("saturating {:?}", i.saturating_add(i));
//    println!("wrapping {:?}", i.wrapping_add(i));

//    assert_eq!(i32::max_value(), 0x7fffffff);
//    assert_eq!(i32::max_value().checked_mul(2), Some(1));
//    println!("Hello, world!");
   println!("{:#?}", reverse2(0x3fffffff));
   println!("{:#?}", reverse2(-2147483412));
   println!("{:#?}", reverse2(-1463847412));
   println!("{:#?}", reverse2(1463847412));
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

pub fn reverse2(x: i32) -> i32 {
    let mut num = x;

    let mut rev:i32 = 0;

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

        if let Some(t) = rev.checked_mul(10) {
            if let Some(t1) = t.checked_add(num % 10) {
                rev = t1
            }else{
                return 0;
            }

        }else {
            return 0;
        }
//        rev = rev * 10 + num % 10;
//        println!("{}", rev);
        num /= 10;
    }

    rev
}