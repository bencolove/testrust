struct WithDrop {
    name: String
}

impl Drop for WithDrop {
    fn drop(&mut self) {
        println!("{} is dropping", self.name)
    }
}

fn main() {
    // test dropping functioning as expected
    {
        let toDrop = WithDrop {
            name: String::from("roger"),
        };
    }

    // will var be dropped before program exits? YES!!! it drops
    
    let var = WithDrop {
        name: String::from("jim"),
    };
    panic!("I panic here, dont know whether var got a chance to drop or not");
    
    // println!("out of main");
}