// cooks are threads?
// bowl is the receiver
// have a file for tomatoes, cilantro, avocado, onion, lime juice and chop them up into "byte" size pieces. put them in the bowl
// read into buffer [1000] 1000 bytes
// use std::io;
use std::io::prelude::*;
use std::fs::{ File };
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc;
use std::error::Error;

// const CHOP_BLOCK_SIZE: usize = 1000;

pub struct Bowl {
  // pico_de_gallo: Vec<u8>
  pico_de_gallo: usize
}

impl Bowl {
  pub fn new() -> Self {
    Bowl {
      // pico_de_gallo: Vec::new()
      pico_de_gallo: 0
    }
  }

  // pub fn mix(&mut self, receiver: &Receiver<Vec<u8>>) -> Result<(), mpsc::RecvError> {
  pub fn mix(&mut self, receiver: &Receiver<usize>) -> Result<(), mpsc::RecvError> {
    loop {
      match receiver.recv() {
        Ok(bytes) => { 
          println!("got somethin!!");
          // bytes.iter().for_each(|byte| self.pico_de_gallo.push(*byte))
          // self.pico_de_gallo.append(&mut bytes.clone());
          self.pico_de_gallo += bytes;
        },
        Err(_) => { 
          println!("All done!");
          return Ok(())
        }
      }
    }
  }

  // pub fn dip(&mut self) -> u8 {
  pub fn dip(&mut self) -> usize {
    // self.pico_de_gallo.pop().expect("WE'RE OUT OF PICO!!!!")
    self.pico_de_gallo -= 200;
    200
  }

  pub fn pico_left(&self) -> usize {
    // self.pico_de_gallo.len()
    self.pico_de_gallo
  }
}


// pub fn cook(path: &str, sender: &Sender<Vec<u8>>) -> Result<usize, Box<Error>> {
pub fn cook(path: &str, sender: &Sender<usize>) -> Result<usize, Box<Error>> {
  let mut f = File::open(path)?;
  // let mut buffer = [0; CHOP_BLOCK_SIZE];

  // let mut bytes_chopped = 0;

  // loop {
  //   let chopped = f.read(&mut buffer[..])?;
  //   println!("{} bytes chopped from {}!!!", chopped, path);
  //   bytes_chopped += chopped;

  //   sender.send(buffer)?;

  //   if chopped == 0 {
  //     return Ok(bytes_chopped);
  //   }
  // }
  let mut buffer = Vec::new();

  let chopped = f.read_to_end(&mut buffer)?;

  // sender.send(buffer)?;
  sender.send(chopped)?;

  Ok(chopped)
}