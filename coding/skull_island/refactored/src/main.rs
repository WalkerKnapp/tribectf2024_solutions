extern crate crypto;

use std::{
    fs::File,
    io::{self, stdin, stdout, Error, Read, Write},
    process::exit
};
use std::cmp::max;
use std::cmp::min;
use bitvec::bitvec;
use bitvec::order::{Msb0};
use bitvec::vec::BitVec;
use rand::random;
use crypto::{
    aes::{
        ecb_encryptor,
        KeySize::KeySize256
    },
    blockmodes as bm,
    buffer::{
        BufferResult::{
            BufferOverflow as BO,
            BufferUnderflow as BU
        },
        ReadBuffer as bufr,
        RefReadBuffer as rrb,
        RefWriteBuffer as rwb,
        WriteBuffer as wb
    }
};
use ndarray::{s, Array2, Array};

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

fn raise_the_flags(encrypted_message: Vec<u8>) -> Result<Array2<bool>, Error> {
    if encrypted_message.len() != 512 {
        println!("{BAD_LENGTH}");
        exit(0);
    }

    // Convert the message into a 64x64 binary matrix
    let binary_matrix = Array::from_iter(
        BitVec::<u8, Msb0>::from_vec(encrypted_message)
    ).into_shape((64, 64)).unwrap();

    // Create a larger 512x512 matrix and place the message matrix in the middle of it
    let mut ocean = Array2::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);
    ocean
        .slice_mut(s![
            OCEAN_SHAPE / 2 - 32..OCEAN_SHAPE / 2 + 32,
            OCEAN_SHAPE / 2 - 32..OCEAN_SHAPE / 2 + 32
        ])
        .assign(&binary_matrix);

    // Update the binary matrix with `motion` 150 times
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
fn motion(island: &Array2<bool>, seed: usize) -> Array2<bool> {

    let mut continent = Array2::from_elem((OCEAN_SHAPE, OCEAN_SHAPE), false);

    let offset = (seed % HELP_QUANTITY) as i32;

    let mut x = -offset;

    while x < island.shape()[1] as i32 {
        let mut y = -offset;

        while y < island.shape()[0] as i32 {
            let input_block = island.slice(s![
                max(x, 0)..min(x+3, island.shape()[0] as i32),
                max(y, 0)..min(y+3, island.shape()[1] as i32)
            ]);
            let mut output_block = continent.slice_mut(s![
                max(x, 0)..min(x+3, island.shape()[0] as i32),
                max(y, 0)..min(y+3, island.shape()[1] as i32)
            ]);

            let sum = input_block.mapv(|x| x as i32).sum();

            if sum == 4 || sum == 5 {
                output_block.assign(&input_block.mapv(|x| !x))
            } else {
                output_block.assign(&input_block)
            }
            // increment y
            y += HELP_QUANTITY as i32;
        }
        // increment x
        x += HELP_QUANTITY as i32
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

fn check_pool(lava: Array2<bool>) -> bool {
    let mut volcano = File::open("lava").unwrap();

    let mut data = bitvec![u8, Msb0; 0; volcano.metadata().unwrap().len() as usize];
    volcano.read_exact(data.as_raw_mut_slice()).unwrap();

    let volcano_matrix = Array::from_iter(
        data
    ).into_shape((64, 64)).unwrap();

    return volcano_matrix == lava;
}

fn dig_treasure() -> Result<String, io::Error> {
    // seems to get the flag we need.
    // should probably figure out how to call this function on the server.
    let mut package = File::open("flag.txt")?;
    let mut treasure = [0u8; 256];
    package.read(&mut treasure)?;
    Ok(String::from_utf8(treasure.to_vec()).unwrap())
}

fn pancakeify() -> Result<Vec<u8>, io::Error> {
    // Read a message from stdin
    let mut message = String::new();
    stdin().read_line(&mut message)?;
    message = message.replace("\n", "").replace("\r", "");

    // Encode the message as bytes and pad the length to a multiple of 32
    let mut message_bytes = message.as_bytes().to_vec();
    message_bytes.append(&mut vec![0x0; 32 - message_bytes.len() % 32]);

    // Create an AES256 encryptor and buffers to feed data into and out from it
    let mut encryptor = ecb_encryptor(KeySize256, SUPERSECRET.as_bytes(), bm::NoPadding);

    let mut encrypted_message = Vec::<u8>::new();

    let mut input_buffer = rrb::new(&message_bytes);
    let mut output = [0; 256];
    let mut output_buffer = rwb::new(&mut output);

    loop {
        // Encrypt a block of data with AES
        let result = encryptor.encrypt(&mut input_buffer, &mut output_buffer, true);

        // Take the encrypted data from output_buffer and accumulate it in encrypted_message
        encrypted_message.extend(
            output_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&x| x),
        );

        // Break when all data has been consumed
        match result {
            Ok(BO) => (),
            Ok(BU) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    Ok(encrypted_message)
}
