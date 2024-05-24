use std::io;

fn main() {
    println!("Hello, world!");

    test_get_mut();
}

fn test_get_mut() -> io::Result<u32> {
    let mut s = Struct {
        values: vec![1, 2, 3, 4],
        color: 1,
    };
    let s1 = &mut s;
    let val1 = s1.get(1)?;
    let val2 = s1.get(2)?;

    let v = s1.get(1)?.unwrap();
    let c = &s1.color;

    Ok(1)
}

struct Struct {
    values: Vec<u32>,
    color: u32,
}

impl Struct {
    pub fn get(&mut self, idx: usize) -> io::Result<Option<&u32>> {
        Ok(self.values.get(idx))
    }
}
