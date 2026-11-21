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

    pub fn alloc(&mut self, reqsiz:usize)->&mut [u8]{
        let mut rng= rand::rng();
        let inizio: usize = self.offset;
        self.offset = self.offset + reqsiz;
        let fine: usize = self.offset;
        while fine > self.zone.capacity() {
            let randm: usize = rng.random_range(1..=64);
            self.zone.reserve((fine-self.zone.len())+randm);
        }   
        self.zone.resize(fine,0);

        &mut self.zone[inizio..fine]
    }

}