fn main(){
    let a:i32 = 5;
    //&a先转成raw指针，然后再把指针转成usize，这样就可以显示地址了。
    let addr = &a as *const i32 as usize;
    println!("addr：0x{:X}",addr);
	let p =  addr as *const u32 ;
	unsafe{ 
      let x = *p;
      println!("addr:{:?}",x);
    }
}
