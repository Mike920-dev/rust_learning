pub fn fahren_cels(choice: bool, fahr: u32, cel: u32) -> u32{
    let result: u32 = if choice {((cel * 9) / 5) + 32} else {((fahr - 32) * 5) / 9}; // true Fahr to Cel
    result
}   