use rand::Rng;

fn main() {
    println!("========== start ===========");

    // gen a rand num, create vector @ len of rand_num
    // gen a 2nd rand num and use it to populate every cell with same num during init
    let mut rng = rand::thread_rng();
    let y: u8 = rng.gen();
    let rndlen = rand::thread_rng().gen_range(3..=25);
    let mut vect1: Vec<u8> = vec![y; rndlen];

    println!("Chosen random vector len: {}", rndlen);
    println!("Populating vector with random info");

    // load each cell in the vector with a set of random data
    for value in &mut vect1 {
        let y: u8 = rng.gen();
        *value = y;
        println!("Vector:<{}>", &value);
    }

    // cycle through vector and populate with random data
    // I really dont like this methodology
    // for i in 1..rndlen {
    //for i in &vect1 {
    // while let Some(thing) = vect1.get(1) {    // this did not gen an error

    let mut vcx: usize = 0; 
    println!("Populate vector cells with new random data @ Vector length: {0}", vect1.len() );
    for valv in &mut vect1 {    // valv <= contents of vector cell
        let rpopulate = rand::thread_rng().gen_range(0..=100);     // select a random num
        println!("populating vector - cell: {:03} / orig: {:03} / new: {:03}", vcx, valv, rpopulate);
    	vcx += 1;
        *valv = rpopulate;
    }

    println!("==============================");
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
