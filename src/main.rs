use rand::Rng;

fn main() {
    println!("========== start ===========");

    /* gen a rand num, create vector @ len of rand_num
      gen a 2nd rand num and use it to populate every cell with same num during init */
    let mut rng = rand::thread_rng();
    let y: u8 = rng.gen();
    let rndlen = rand::thread_rng().gen_range(3..=25);
    let mut vect1: Vec<u8> = vec![y; rndlen];
    println!("Chosen random vector len of: {}", rndlen);
    

    // load each cell in the vector with a set of random data
    println!("\nPopulating vector with ORIGNAL random data...");
    let mut neednl: u8 = 1;
    for value in &mut vect1 {
        let y: u8 = rng.gen();
        *value = y;     // insert random number into this cell via *derefencing
        match neednl {  // print rows with 5 collumns only
            5 | 10 | 15 | 20 | 25 => println!("/ Vector: <{:03}>", &value),
	    _ => print!("Vector: <{:03}> ", &value),
	};
	neednl += 1;
    }
    
   println!("\n============= ==========");


    // OLD learning method
    // cycle through vector and populate with random data
    // I really dont like this methodology
    /* for i in 1..rndlen {
       for i in &vect1 {
       while let Some(thing) = vect1.get(1) {    // this did not gen an error */
    /*
    let mut vcx: usize = 0;     // simple counter to track which vector vell we're working on - messy/sloppy
    println!("Populate vector cells with new random data @ Vector length: {0}", vect1.len() );
    for valv in &mut vect1 {    // valv <= contents of vector cell
        let rpopulate = rand::thread_rng().gen_range(0..=100);     // select a random num
        println!("populating vector - cell: {:03} / orig: {:03} / new: {:03}", vcx, valv, rpopulate);
    	vcx += 1;
        *valv = rpopulate;  // *deference, to take ownerhip and mutate the reference mutatable value
    }
    */

    // new method, more mature
    // avoids sloppy unintegrated counters
    for (i, item) in vect1.iter_mut().enumerate() {
        let rpopulate = rand::thread_rng().gen_range(0..=100);     // select a random num
        print!("NEW sliced vector populate - cell: {:03} / orig: {:03} / NEW: {:03}", i, item, rpopulate);
        *item = rpopulate;      // deref item as new value
        println!(" ...updated!")
    }

    println!("==============================");
    println!("Vector len: {}", vect1.len() );

    // and odd way to access the data in a vector
    // this is a read operation, not a write
    for value in &vect1 {              // a &refernce borrow, i.e. no ownership
        println!("Vector: <{:03}>", &value);    // &value is a borrowed &referecne -> the data vect1 points to
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
        println!("value = {:03}", value); // using curly braces to format a local variable
    }
}
