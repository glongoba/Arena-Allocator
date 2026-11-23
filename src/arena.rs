use rand::RngExt;

pub struct Arena{
    zone: Vec<u8>,
    offset: usize,
}

impl Arena{

    pub fn new(capacity: usize)-> Self{
        Self{
            zone: Vec::with_capacity(capacity),
            offset: 0,
        }
    }

    pub fn reset(&mut self){ //&mut perchè dobbiamo modificare Self
        self.offset = 0;
        self.zone.clear();
    }

    pub fn alloc(&mut self, reqsiz:usize)-> Result<&mut [u8],String>{
        let mut rng= rand::rng();
        let inizio: usize = self.offset;
        let fine = match self.offset.checked_add(reqsiz){
            Some(value) => value,
            None => return Err(String::from("Offset nel valore. \n")),
        };
        self.offset = fine;
        if fine > self.zone.capacity() {
            let randm: usize = rng.random_range(1..=64);
            self.zone.reserve((fine-self.zone.len())+randm);
        }   
        self.zone.resize(fine,0);

        Ok(&mut self.zone[inizio..fine])
    }

    pub fn cpacity(&self) -> usize{
        self.zone.capacity()
    }

    pub fn lenf(&self) -> usize{
        self.zone.len()
    }   
}