extern crate crypto; use std::{ fs::File as pip, io::{self, stdin, stdout, Error, Read, Write}, process::exit, }; use rand::random; use crypto::{ aes::{ecb_encryptor as sha256, KeySize::KeySize256 as SHA256DigestSize}, blockmodes as bm, buffer::{ BufferResult::{BufferOverflow as BO, BufferUnderflow as BU}, ReadBuffer as bufr, RefReadBuffer as rrb, RefWriteBuffer as rwb, WriteBuffer as wb, }, }; use ndarray::{s as info, Array2 as Act1}; type Strange = Act1<bool>;
const OCEAN_SHAPE: usize = 512;
const SUPERSECRET: &str = "ThisIsASuperSecretAndHiddenPaswd";
const HELP_QUANTITY: usize = 3;
const BANNER: &str = r#"                                                                                                    
                                                                                                    
                                          .::^^^~~~~^^::..                                          
                                  .^!?YPGB#&&@@@@@@@@@&&##GPY?!^:                                   
                             .^7YG#&@@@@@@@@@@@@@@@@@@@@@@@@@@@@&B5?~:                              
                          ^75#&@@@@@@@@@@@@&&&#######&&@@@@@@@@@@@@@@#GJ~.                          
                       ^JG&@@@@@@@@&#GPJ7!~^:.........:^!7?YPB&@@@@@@@@@@#5!.                       
                    .7G&@@@@@@@&GY7^.                         :^?5B&@@@@@@@@BY^                     
                  :J#@@@@@@@#57:                                  .^?P&@@@@@@@&P!                   
                :Y&@@@@@@#5~.                                         :!P&@@@@@@@G!                 
              .J#@@@@@@B?:                .~!J5PPGGPP5J7~:               ^Y#@@@@@@@P^               
             ~B@@@@@@G7.               :?P#@@@@@@@@@@@@@@&GJ^              :J#@@@@@@&J.             
           .J&@@@@@#7.               ~5&@@@@@@@@@@@@@@@@@@@@&P!              :5@@@@@@@G^            
          .P@@@@@@5:               .5@@@@@@@&&@@@@@@@@@&&&@@@@@P^              ~B@@@@@@#~           
         .G@@@@@@?                :G@@@@@#?^:^!P&@@@@#?^::!G@@@@#~              .5@@@@@@&!          
        .G@@@@@#!     ^7?!:      :G@@@@@@!      5@@@&^      #@@@@#~               J@@@@@@&!         
        5@@@@@&~    .Y&@@@&?     5@@@@@@@5:   .!#@@@@Y:   .?&@@@@@G       :7J?~    Y@@@@@@#^        
       7@@@@@@7     ~@@@@@@#.   .#@@@@@@@@&BPG#@@@@@@@&GPG#@@@@@@@&^     ?&@@@@P:  .P@@@@@@G        
      :#@@@@@P     .^G@@@@@J    .#@@@P7?P@@@@@@@@@@@@@@@@@@&Y7?B@@@~    .B@@@@@@!   :B@@@@@@7       
      ?@@@@@&^   :5#&&@@@@@#P?^. G@@@B^.B@@@@@@@@@@@@@@@@@@@5 7#@@&:     ?@@@@@G!^.  ?@@@@@@B.      
      G@@@@@P    Y@@@@@@@@@@@@@#5P@@@@G.Y@@@@@@@@@@@@@@@@@@@!.B@@@Y  :!YG#@@@@@&@&G: .#@@@@@@~      
     :&@@@@@?    7&@@@@&YP#@@@@@@@@@@@@P:?#@@@@@@@@@@@@@@@G~^B@@@B?YG&@@@@@@@@@@@@@5  P@@@@@@J      
     ~@@@@@@~     ^?55?^  .~JG&@@@@@@@@@#?^?G&@@@@@@@@@&57^J&@@@@@@@@@@@@#5?!B@@@@&!  ?@@@@@@5      
     !@@@@@@~                .:!YB&@@@@@@@#5?7?JJYYYJJ?7?P&@@@@@@@@@&B5?^.   :7JY?^   !@@@@@@P      
     !@@@@@@~                    .:75#@@@@@@@&#BGGGGGB&@@@@@@@@@&GY!^                 7@@@@@@5      
     ^&@@@@@7                        .~?P#@@@@@@@@@@@@&#&@@@#GY7^                     P@@@@@@J      
     .B@@@@@P                            .~JG#@@@@@@@@#YJJ?~.                        .#@@@@@@!      
      J@@@@@&^                         .^7YP5JYPB&@@@@@@&B57^.                       7@@@@@@#.      
      :&@@@@@5                     .~?5B@@@@@@&GJ~75#@@@@@@@&B57:                   .B@@@@@@J       
       J@@@@@@!       ^7?7^    .~?5#@@@@@@@&B5?^.   .^JG&@@@@@@@&GJ~:   .^!~:       5@@@@@@B.       
       .G@@@@@#^     ?&@@@@Y~JP#@@@@@@@&GY7^.           :!JG&@@@@@@@#P?7B@@@&J     J@@@@@@@!        
        ^#@@@@@#~   :#@@@@@@@@@@@@@&GY!:                    :!5B@@@@@@@@@@@@@#:   ?@@@@@@@?         
         ~#@@@@@&7   7#@@@@@@@&#PJ!:                           .^?P#&@@@@#&#G!  .Y@@@@@@@J          
          ^#@@@@@@Y.  .~7B@@@@B^                                   .Y@@@@G^..  ^G@@@@@@@J           
           :P@@@@@@G!   7@@@@@@Y                                    P@@@@@~  .J&@@@@@@&!            
            .J&@@@@@@P~ ^#@@@@@7                                    7&@@@P  7B@@@@@@@P^             
              ^G@@@@@@@P!^?YYJ~                                      ^?J!:?B@@@@@@@#7               
             .  !G@@@@@@@BJ^                                          .~Y#@@@@@@@#J:                
                 .!G@@@@@@@@GJ~.                                   :!Y#@@@@@@@@#J:                  
                    ~5#@@@@@@@@#PJ!:.                         .^7YG#@@@@@@@@&G?.  ..                
                      :7P&@@@@@@@@@&B5J?~^::.         .:^^!?YG#@@@@@@@@@@@BJ^                       
                         :75#@@@@@@@@@@@@@&&#BGGGPGGGB#&&@@@@@@@@@@@@@#G?^.                         
                            .^75B&@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&BPJ~:                             
                                 :~7J5G#&@@@@@@@@@@@@@@@@@&#BPY?!:.                                 
                                       .:^^!!7???????77!~^:.                                        
                                                                                                    
                                                                                                    "#;

const INTRO: &str = "Welcome to skull island, a small island in the center of the Bermuda Triangle, teeming with life.
You have come to placate the diety that inhabits the islands volcanic left eye.

Upon arrival you see a sign telling you that, in order to do so, you must pass in a message on a slip of paper.
This message will determine your fate.";
const BAD_LENGTH: &str = "The diety is displeased...
The length of the message is not what it expected...
The volcano erupts in a fiery column, destroying your ship and stranding you on skull island... FOREVER";

#[allow(non_upper_case_globals)]
const gen_random_integer: fn() -> i32 = || {random::<i32>()*0};

fn raise_the_flags(flintlock: Vec<u8>) -> Result<Strange, Error> {
    // ensures flintlock bore is correct
    if flintlock.len() != 512 {
        println!("{BAD_LENGTH}");
        exit(0);
    }

    // builds a shipyard and a new fleet
    let mut ships = Strange::from_elem((64, 64), false);
    let mut shipyard = 0;
    for crew in flintlock {
        for finger in 0..8 {
            ships[(shipyard / 64, shipyard % 64)] = crew >> 7 - finger & 1 == 1;
            shipyard += 1;
        }
    }

    // places the fleet in the ocean
    let mut ocean = Act1::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);
    ocean
        .slice_mut(info![
            OCEAN_SHAPE / 2 - 32..OCEAN_SHAPE / 2 + 32,
            OCEAN_SHAPE / 2 - 32..OCEAN_SHAPE / 2 + 32
        ])
        .assign(&ships);

    // calculates the next step of the ocean and updates it each time.
    for i in 0..150 {
        ocean = motion(&ocean, i);
    }
    Ok(ocean)
}

// only real comment in this entire program...

/// **Moves the ocean to the next step.**
///
/// For more info, see:
/// https://en.wikipedia.org/wiki/Block_cellular_automaton
fn motion(island: &Strange, seed: usize) -> Strange {

    let mut continent = Act1::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);

    // gets a random number based on seed and another random number
    let random = (seed % HELP_QUANTITY) as i32 + gen_random_integer();
    print!(
        "\r{}{}",
        ". ".repeat(random as usize + 1),
        "  ".repeat(HELP_QUANTITY - random as usize - 1)
    );
    stdout().flush().unwrap();
    // sets x to a random number plus another random number
    let mut x = -random + gen_random_integer();

    while x < island.shape()[1] as i32 {
        // sets quantity to the random number plus another random number
        let mut quantity = -random + gen_random_integer();
        while quantity < island.shape()[0] as i32 {
            let mut uuuuh = 0;
            for dx in 0..HELP_QUANTITY {
                for change in 0..HELP_QUANTITY {
                    if quantity + change as i32 >= 0 && x + dx as i32 >= 0 {
                        match island.get((
                            (x + dx as i32) as usize,
                            (quantity + change as i32) as usize,
                        )) {
                            // if it is alive, he adds 1 to uuuuh
                            Some(&is_alive) => {
                                if is_alive {
                                    uuuuh += 1
                                }
                            }
                            None => (),
                        };
                    }
                }
            }

            // it just does the same loop again...
            // why didn't he just combine it?
            for dx in 0..HELP_QUANTITY {
                for change in 0..HELP_QUANTITY {
                    if quantity + change as i32 >= 0 && x + dx as i32 >= 0 {
                        match continent.get_mut((
                            (x + dx as i32) as usize,
                            (quantity + change as i32) as usize,
                        )) {
                            Some(nospot) => {
                                if uuuuh == 4 || uuuuh == 5 {
                                    *nospot = !island[(
                                        (x + dx as i32) as usize,
                                        (quantity + change as i32) as usize,
                                    )];
                                } else {
                                    *nospot = island[(
                                        (x + dx as i32) as usize,
                                        (quantity + change as i32) as usize,
                                    )];
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
            // increments the quantity
            quantity += 6 - HELP_QUANTITY as i32;
        }
        // and then the x
        x += 6 - HELP_QUANTITY as i32
    }

    continent
}

// this seems to be where the program begins
fn main() -> Result<(), io::Error> {
    // prints banner and intro
    println!("{}", BANNER);
    println!("{}", INTRO);
    // asks for message
    println!("What is your message?");

    // pankacifies...? to the jolly roger.
    let jolly_roger = pancakeify()?;
    println!("\nYou write your message down on a slip of paper and toss it into the volcano.\n");
    println!("The slip of paper slowly flutters down into the volcano...");

    // uses the jolly roger to raise the flag and set to the sea
    let sea = raise_the_flags(jolly_roger)?;
    println!();
    println!();

    // checks the magic pool to sea if the see is right
    if check_pool(sea) {
        println!("A chest of gold with a note nailed to it appears at your feet:");
        println!("{}", dig_treasure()?);
    } else {
        println!(
            r#"You hear a rumbling, and the diety shouts: 
        "YOU HAVE FAILED ME"
        The volcano erupts, destroying your ship and leaving you stranded on the island."#
        );
    }
    Ok(())
}

fn check_pool(lava: Strange) -> bool {
    // opens the pip lava library
    let mut volcano = pip::open("lava").unwrap();

    // builds a new ship
    let mut ship = [0u8];
    for (i, &bit) in lava.iter().enumerate() {
        if i % 8 == 0 {
            volcano.read_exact(&mut ship).unwrap()
        }

        if ((ship[0] >> (7 - (i % 8))) & 1 == 1) ^ bit {
            return false;
        }
    }
    // returns true!
    true
}

fn dig_treasure() -> Result<String, io::Error> {
    // seems to get the flag we need.
    // should probably figure out how to call this function on the server.
    let mut package = pip::open("flag.txt")?;
    let mut treasure = [0u8; 256];
    package.read(&mut treasure)?;
    Ok(String::from_utf8(treasure.to_vec()).unwrap())
}

fn pancakeify() -> Result<Vec<u8>, io::Error> {
    // reads a line from a file into a string
    let mut what_he_said = String::new();
    stdin().read_line(&mut what_he_said)?;
    what_he_said = what_he_said.replace("\n", "").replace("\r", "");

    // dtill don't get this part
    let mut batter = what_he_said.as_bytes().to_vec();
    batter.append(&mut vec![0x0; 32 - batter.len() % 32]);

    // creates a new sha-256 digester to hash the previous string
    let mut pancake = sha256(SHA256DigestSize, SUPERSECRET.as_bytes(), bm::NoPadding);

    // creates some variables. still not sure what they do
    let mut breakfast = Vec::<u8>::new();
    let mut attack = rrb::new(&batter);
    let mut syrup = [0; 256];
    let mut defence = rwb::new(&mut syrup);

    loop {
        // seems to get the digest of the sha256 hash
        // this guy had terrible naming conventions... smh
        let a_aaah = pancake.encrypt(&mut attack, &mut defence, true);
        breakfast.extend(
            defence
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&x| x),
        );

        // if all the data has been digested, leave
        match a_aaah {
            Ok(BO) => (),
            Ok(BU) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    // return... breakfast?
    Ok(breakfast)
}
