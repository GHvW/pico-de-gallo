
use pico_de_gallo::{ Bowl, cook };
use std::sync::mpsc;
use std::thread;
use std::time::{Instant};

fn main() {
    let now = Instant::now();
    println!("Hello, world!");

    // let num_cooks = 1;

    let tomatoes = "../tomatoes.txt";
    let onions = "../onions.txt";
    let cilantro = "../cilantro.txt";
    let jalapenos = "../jalapenos.txt";

    let ingredients = [tomatoes, onions, cilantro, jalapenos];

    let mut bowl = Bowl::new();
    let mut cooks = Vec::new();

    let (sender, receiver) = mpsc::channel();

    
    for ingredient in 0..ingredients.len() {
        let send = sender.clone();

        let cook = thread::spawn(move || {
            let thread_now = Instant::now();
            let chopped = cook(&ingredients[ingredient], &send).expect("something went very, very wrong");

            println!("Cook {}, chopped {} bytes of {}", ingredient, chopped, ingredients[ingredient]);
            println!("This cook took {} millis to chop {}!", thread_now.elapsed().as_millis(), ingredients[ingredient]);
        });

        cooks.push(cook);
    }
    // do this now or else the bowl is gonna block!
    drop(sender);

    bowl.mix(&receiver).expect("mixing accident!!!");

    for cook in cooks {
        cook.join().unwrap();
    }


    println!("Number of bytes in the bowl: {}", bowl.pico_left());
    println!("This program took {} milliseconds to run", now.elapsed().as_millis());
}

// fn main() {
//      let now = Instant::now();
//      println!("Hello, world!");


//     let tomatoes = "../tomatoes.txt";
//     let onions = "../onions.txt";
//     let cilantro = "../cilantro.txt";
//     let jalapenos = "../jalapenos.txt";

//     let ingredients = [tomatoes, onions, cilantro, jalapenos];

//     let mut bowl = Bowl::new();

//     let (sender, receiver) = mpsc::channel();

//     thread::spawn(move || {
//         let thread_now = Instant::now()
//         for ingredient in 0..ingredients.len() {

//             let chopped = cook(&ingredients[ingredient], &sender).expect("something went very, very wrong");

//             println!("Cook {}, chopped {} bytes of {}", ingredient, chopped, ingredients[ingredient]);
//         }
//         println!("This cook took {} milliseconds to complete the job!", thread_now.elapsed().as_millis());
//     });

//     bowl.mix(&receiver).expect("mixing accident!!!");

//     // sender.join().unwrap();


//     println!("Number of bytes in the bowl: {}", bowl.pico_left());
//     println!("This program took {} milliseconds to run", now.elapsed().as_millis());
//  }