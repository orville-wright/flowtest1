use rand::Rng;

fn main() {
    println!("========== start ===========");
    // gen a rand num, create vector @ len of rand_num
    // 

    let rndlen = rand::thread_rng().gen_range(3..=25);
    // let mut vect1 = vec![0; rndlen];
    let mut vect1: Vec<u8> = vec![7; rndlen];

    println!("Chosen random vector len: {}", rndlen);

    // cycle through vector and populate with random data
    // I really dont like this methodology
    // for i in 1..rndlen {
    //for i in &vect1 {
   
    let mut vcx: usize = 0; 
    // let data = &vect1[0];

    // while let Some(thing) = vect1.get(1) {    // this did not gen an error
    println!("Prepare to populate vector of current length: {0}", vect1.len() );
    for valv in &mut vect1 {    // valv <= contents of vector cell
        let rpopulate = rand::thread_rng().gen_range(0..=100);     // select a random num
        // ZZlet data = &vect1[vcx];
        //let vcx = vect1.get(1);
        println!("populating vector : {0} : {1} / {2}", vcx, valv, rpopulate);
        //vect1.push(rpopulate);
	vcx += 1;
        *valv = rpopulate;
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
