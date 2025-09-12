pub fn euclidean_distance(x: [u8; 3], y: [u8; 3]) -> u128 {
    /*See https://en.m.wikipedia.org/wiki/Euclidean_distance#Squared_Euclidean_distance*/
    let r: u128 = ((x[0] as i16 - y[0] as i16) as isize).pow(2) as u128;
    let g: u128 = ((x[1] as i16 - y[1] as i16) as isize).pow(2) as u128;
    let b: u128 = ((x[2] as i16 - y[2] as i16) as isize).pow(2) as u128;

    r + g + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_compare() {
        assert_eq!(euclidean_distance([195, 40, 10], [0, 0, 0]), 39725);
        assert_eq!(euclidean_distance([10, 40, 255], [195, 40, 10]), 94250);
    }
}
