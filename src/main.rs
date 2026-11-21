mod arena;

use arena::Arena;
use std::io;
use rand::RngExt;

fn main(){
    let mut rng= rand::rng();
    println!("Iniziamo il progetto Arena. Versione 1. \n");
    let mut Arena1 = Arena::new(16);
    println!("Vuoi allocare memoria? \n");
    let mut finput=String::new();
    io::stdin()
        .read_line(&mut finput)
        .expect(" Errore nella lettura. Fanculo \n");
    while finput.trim()== "si" {
        println!("Quanto spazio allocare? \n");
        let mut num= String::new();
        io::stdin()
            .read_line(&mut num)
            .expect(" Errore nella lettura. Fanculo \n");
        let numero: usize = num
            .trim()
            .parse()
            .expect(" Inserire un valore valido zio. \n");
        let value= Arena::alloc(&mut Arena1,numero);
        for i in 0..numero {
            let ran: u8 = rng.random();
            value[i] = ran;
            println!(" Value i-esimo {i} = {} \n",value[i]);
        }
    println!("Vuoi ancora allocare memoria? \n");
    finput.clear();
    io::stdin()
        .read_line(&mut finput)
        .expect(" Errore nella lettura. Fanculo \n");
    }

    Arena::reset(&mut Arena1);
}