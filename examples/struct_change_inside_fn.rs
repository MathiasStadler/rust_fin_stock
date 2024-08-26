// FROM HERE
// https://rust-unofficial.github.io/patterns/idioms/ctor.html

pub struct Second {
    value_1: u64,
    value_2: u64,
}

impl Second {
    /// Returns the value in seconds.
    pub fn value_1(&mut self) -> u64 {
        self.value_1
    }

    pub fn value_2(&self) -> u64 {
        self.value_2
    }

    fn  add(&mut self,add:u64){

        self.value_1 = self.value_1 + add;

    } 
}

impl Default for Second {
    fn default() -> Self {
        Self { value_1: 0, value_2: 0 }
    }
}



    



fn main(){
    let mut s = Second::default();

    println!("default => {}",s.value_1());
    println!("default => {}",s.value_2());

    s.value_1 = 69;
    s.value_2 = 23;
    
    s.add(4);
    // assert_eq!(0, s.value());


    println!("{}",s.value_1());
    println!("{}",s.value_2());

    s.value_1 = 96;
    s.value_2 = 32;
    
    // assert_eq!(0, s.value());


    println!("{}",s.value_1());
    println!("{}",s.value_2());

    
}