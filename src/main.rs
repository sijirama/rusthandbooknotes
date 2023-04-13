fn main (){
    let x = 5;
    println!("x is {x}");
    {
        let x = x + 10;
        println!("{x}");
    }
    println!("x is {x}");
}


