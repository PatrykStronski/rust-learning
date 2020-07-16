use rand::Rng;

const LOWER_RANGE: usize = 2;
const UPPER_RANGE: usize = 8;

/* Doesn't work
 * fn swap(chromosome_elem1: &mut u8, chromosome_elem2: &mut u8) {
    let mut tmp1 = chromosome_elem1;
    let mut tmp2 = chromosome_elem2;
    chromosome_elem1 = &mut*tmp2;
    chromosome_elem2 = &mut*tmp1;
}*/

fn exchange_chromosomes(chr_1: &mut [u8;10], chr_2: &mut [u8;10]) {
   let mut rng = rand::thread_rng();
   let cutting_point: usize = rng.gen_range(LOWER_RANGE, UPPER_RANGE) as usize; 
   for x in 0..cutting_point {
      let tmp = chr_1[x];
      chr_1[x] = chr_2[x];
      chr_2[x] = tmp;
   }
}

fn read_chromosomes(mut chr1: [u8;10], mut chr2: [u8;10]) {
    chr1[0] = 14;
    chr2[0] = 14;
    println!("{:?}, {:?}", chr1, chr2);
}

fn main() {
    let mut chromosome_1: [u8; 10] = [10,15,88,0,12,9,8,3,5,78];
    let mut chromosome_2: [u8; 10] = [0,111,17,13,14,16,0,9,7,33];
    println!("First chromosome: {:?}", chromosome_1);
    println!("Second chromosome: {:?}", chromosome_2);
    exchange_chromosomes(chromosome_1, chromosome_2);
    println!("After crossover");
    read_chromosomes(chromosome_1, chromosome_2);
    println!("First chromosome: {:?}", chromosome_1);
    println!("Second chromosome: {:?}", chromosome_2);
}
