use rand::Rng;
use device_query::{DeviceQuery, DeviceState, Keycode};

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

    // read keyboard
    println!("\n============= phase 7 ==========");
    println!("Runnign keybd device reader loop...");
    let keys_pressed = key_reader();
    println!("\n============= DONE ! =============" );
    println!("The numnber of keys pressed: {:03}", keys_pressed );
    basic_kbdr();

}


fn key_reader() -> i32 {
    // This code doesn't compile on Linux as it needs a speicfic X11 lib
    #![doc ="It seems device_query:DeviceState for Linux needs specific libs"]
    #![doc = "I havent figurred out the Linux issue yet"]
    // Hopefully its not too large a set of libs needed

    println!("\n============= phase 7 ==========");
    let device_state = DeviceState::new();
    let mut prev_keys = vec![Keycode::A];

    let mut cx = 1;     // counter
    loop {                   // loop = a RUST keyword, not a free label
        let keys = device_state.get_keys();
        let keys_c = &keys;      // borrow via ref &key otherwise keys gets invalidated via move due to its type!
        if keys != prev_keys {
            if keys_c.last() != None {
                if keys.contains(&Keycode::Q) == true {
                    print!("Q key pressed : {} / {:?}", cx, keys.last().unwrap() );
                    break;
                } else {
                    println!("Key pressed : {} / {:?}", cx, keys.last().unwrap() );
                    cx += 1;
                }
            }
        }
        prev_keys = keys;
    }
    cx      // return value (weird but true. Nor return keyword needed)
}

fn basic_kbdr () {
    let device_state = DeviceState::new();
    //let keys_c &Vec<Keycode> = &keys;      // borrow via ref &key otherwise keys gets invalidated via move due to its type!
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            match keys.len() {
                0 => continue,
                3 => println!("3 keys pressed: {} + {} + {}", keys[0], keys[1], keys[2] ),
                2 => println!("2 keys pressed : {} + {}", keys[0], keys[1] ),
                1 => if keys.starts_with(&[Keycode::Escape]) {  //press escape to exit code
                    println!{"Escape exit key pressed...!"};
                    break;
                    } else {
                        println!("1 key pressed : {:#?}", keys[0] );
                    },
                _ => println!("Too many keys pressed!" )
            };
            //println!("Same key...d : /{:#?}/", keys[0] );
        }
        prev_keys = keys;
    }
    // todo:
    // bug 1 if the same single key is press multiple times, thie code does nothing
}