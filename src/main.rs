use rand::Rng;

fn main() {
    println!("========== start ===========");
    // gen a rand num, create vector @ len of rand_num
    // 

    let rndlen = rand::thread_rng().gen_range(3..=25);
    // let mut vect1 = vec![0; rndlen];
    let mut vect1: Vec<u8> = vec![];

    println!("Chosen random vector len: {}", rndlen);

    // cycle through vector and populate with random data
    // I really dont like this methodology
    // for i in 1..rndlen {
    //for i in &vect1 {
    let mut vcx: u8 = 0;
    
    for vcell in &vect1 {
	   let rpopulate = rand::thread_rng().gen_range(0..=100);     // select a random num
	   println!("populating vector: {0}/{1}", vect1.get(vcx), rpopulate);
	   vect1.push(rpopulate);
    }


    println!("Vector len: {}", vect1.len() );

    // and odd way to access the data in a vector
    for value in &vect1 {
        println!("Vector:<{}>", &value);
    }


    // playing with checking the len
    if vect1.len() > 5 {
        println!("List is longer than five items");
    }

    // Pattern matching on len
    match vect1.len() {
        0 => println!("Empty - NO data"),
        1 => println!("Small - ONE value"),
        2..=5 => println!("Medium - TWO ~ FIVE values"),
        6..=15 => println!("Big - 6 ~ 15 values"),
        _ => println!("Verg big - Many values exceeds 15"),
    };

    // while loop with predicate and pattern matching using let
    while let Some(value) = vect1.pop() {
        println!("value = {value}"); // using curly braces to format a local variable
    }
}
