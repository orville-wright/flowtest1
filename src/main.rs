use rand::Rng;

fn main() {
    println!("========== start phase 1 ===========");

    /* gen a rand num, create vector @ len of rand_num
      gen a 2nd rand num and use it to populate every cell with same num during init */
    let mut rng = rand::thread_rng();
    let y: u8 = rng.gen();
    let rndlen = rand::thread_rng().gen_range(3..=25);
    let mut vect1: Vec<u8> = vec![y; rndlen];       // crate new vector @ rndlen, cells filled with default values = y
    println!("Chosen random vector len of: {}", rndlen);
    
    /* when the vector was created, the create loaded all cells with the same value.
       that crappy. So now we want to update/overwrite each cell with unique + random data.
    */
    println!("========== start phase 2 ===========");
    println!("\nPopulating vector with NEW random data...");
    let mut neednl: u8 = 1;     // new to print a NL after the 5th data element
    for value in &mut vect1 {
        let y: u8 = rng.gen();  // generate random num
        *value = y;             // insert random number into this cell via *derefencing
        match neednl {          // match every 5 cycles, sets-up 5 nice columns for printing
            5 | 10 | 15 | 20 | 25 => println!("/ Vector: <{:03}>", &value),     // print rows with 5 collumns only
	    _ => print!("Vector: <{:03}> ", &value),
	};
	neednl += 1;                // column counter
    }
    
   println!("\n============= phase 3 ==========");

    // OLD learning method
    // cycle through vector and populate with random data
    /* save as ref for old way the solve the problem...
    for i in 1..rndlen {
       for i in &vect1 {
       while let Some(thing) = vect1.get(1) {    // this did not gen an error
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
        print!("Slice vector to populate new data - cell: {:03} / OLD: {:03} / NEW: {:03}", i, item, rpopulate);
        *item = rpopulate;      // deref item as new value
        println!(" ...updated!")
    }

    println!("\n============= phase 4 ==========");
    println!("Vector len: {}", vect1.len() );

    // Simple basic loop to access/read the data in a vector/ No iters
    for value in &vect1 {                  // a &refernce borrow, i.e. no ownership
        println!("Vector: <{:03}>", &value);    // &value is a borrowed &referecne -> the data vect1 points to
    }

    // playing with checking the len
    println!("\n============= phase 5 ==========");
    if vect1.len() > 5 {
        println!("Vector random length is longer than FIVE items...");
    }

    // Pattern matching on len
    print!("Vector capcity is... ");
    match vect1.len() {
        0 => println!("Empty - NO data"),
        1 => println!("Small - only ONE value"),
        2..=5 => println!("Medium - TWO ~ FIVE values"),
        6..=15 => println!("Big - 6 ~ 15 values"),
        _ => println!("Verg big - many values exceeding 15"),
    };

    // while loop with predicate and pattern matching using let
    println!("\n============= phase 6 ==========");
    while let Some(value) = vect1.pop() {
        println!("value = {:03}", value); // using curly braces to format a local variable
    }
}
