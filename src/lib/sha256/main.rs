// STEPS TO FOLLOW : //
/*
check(https://blog.boot.dev/cryptography/how-sha-2-works-step-by-step-sha-256/) for clear steps
        Pre-Processing :
        -Convert string to binary
)       -Append a single 1
        -Fill with 0 until data is a multiple of 512 (- 64bits(for the end)) (adapt whether the string is HUGE or not)
        -Append the 64 bits we left empty at the step before where the 64 are bigendian integer representing the length of the original input in binary

        Process :
        -Initialize 8 hash values(first 32 bits of fractionnal parts of square roots of the first 8 primes 2 3 5 7 11 13 17 19)
        -Initialize round const (64 of them,
        each value (0-63) is the first 32 bits of the fractional parts of the cube roots of the first 64 primes (2 - 311))
        -Loop for each 512 bits chunk
        -Copy the input data from step 1 into a new array where each entry is a 32-bit word,
        Add 48 more words initialized to zero, such that we have an array w[0…63],
        Modify the zero-ed indexes at the end of the array using the following algorithm,
        For i from w[16…63]:
            s0 = (w[i-15] rightrotate 7) xor (w[i-15] rightrotate 18) xor (w[i-15] rightshift 3)
            s1 = (w[i- 2] rightrotate 17) xor (w[i- 2] rightrotate 19) xor (w[i- 2] rightshift 10)
            w[i] = w[i-16] + s0 + w[i-7] + s1
        This leaves us with 64 words in our message schedule (w):
        -Initialize variables a, b, c, d, e, f, g, h and set them equal to the current hash values respectively. h0, h1, h2, h3, h4, h5, h6, h7
        Run the compression loop. The compression loop will mutate the values of a…h. The compression loop is as follows:
        for i from 0 to 63
            S1 = (e rightrotate 6) xor (e rightrotate 11) xor (e rightrotate 25)
            ch = (e and f) xor ((not e) and g)
            temp1 = h + S1 + ch + k[i] + w[i]
            S0 = (a rightrotate 2) xor (a rightrotate 13) xor (a rightrotate 22)
            maj = (a and b) xor (a and c) xor (b and c)
            temp2 := S0 + maj
            h = g
            g = f
            f = e
            e = d + temp1
            d = c
            c = b
            b = a
            a = temp1 + temp2
        Let’s go through the first iteration, all addition is calculated modulo 2^32,
        That entire calculation is done 63 more times, modifying the variables a-h throughout. We won’t do it by hand but we would have ender with:
        -After the compression loop, but still, within the chunk loop, we modify the hash values by adding their respective variables to them,
        a-h. As usual, all addition is modulo 2^32.
        h0 = h0 + a, h1 = h1 + b, ...
        -Concatenate final hash, a simple string concatenation will do.



        Pseudo-Code from wikipedia for an example: https://en.wikipedia.org/wiki/SHA-2
 */
use num_primes::Generator;
use num_traits::ToPrimitive;

//NEED TO TEST
pub fn preprocessing(s: String) -> String {
    let mut data = String::new();
    for character in s.clone().into_bytes() {
        data += &format!("0{:b} ", character);
    }
    data.push('1');
    let mut len = data.len();
    while len < 64 || len % 512 != 0 {
        data.push('0');
        len += 1;
    }
    let length = (s.len() * 8) as u64;
    let bigenlen = length.to_be_bytes();
    for b in bigenlen {
        data += &format!("{:08b}", b);
    }
    data
}
// function for processing, that's where we do all the job
pub fn processing(s: String) {
    let mut hash_val = [0u32; 8];
    let mut round_const = [0u32; 64];
    let mut primes = Vec::new();
    while primes.len() < 64 {
        let prime = Generator::new_prime(8);
        if let Some(prime_u64) = prime.to_u64() {
            if !primes.contains(&prime_u64) {
                primes.push(prime_u64);
            }
        }
    }
    for i in 0..64 {
        let sqrt_prime = (primes[i] as f64).sqrt();
        let fractionnalpart = sqrt_prime - sqrt_prime.floor();
        if i < 8 {
            hash_val[i] = (fractionnalpart * (1u64 << 32) as f64) as u32;
            round_const[i] = (fractionnalpart * (1u64 << 32) as f64) as u32;
        } else {
            round_const[i] = (fractionnalpart * (1u64 << 32) as f64) as u32;
        }
    }
    let len = (s.len() * 8) as u64;
    let nbchunks = len / 512;
    let dataarray = [0u32; 64];
    for i in 0..nbchunks {
        for j in 0..16 {
            dataarray[j] = &data[j * 32..(j + 1) * 32];
        }
    }
}
