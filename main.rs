

fn main() {
    println!("1 + 3 ={}",1u32+3);
    println!("1-1={}",1i32-1);
    println!("true and false = {}",true && false );
    println!("true and true = {} ", true && true );
    println!("1010 & 0101 = {:04b}", 0b1010 & 0b0101u32);
    println!("2 <<5 is {}",2u32<<5);
    println!("0xff>>2 is {:x}",0xffu32 >>2);
    println!("{:?}",1..10);
    
}