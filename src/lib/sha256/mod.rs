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

        test

        Pseudo-Code from wikipedia for an example: https://en.wikipedia.org/wiki/SHA-2
 */

//tranform a string in a binary string
pub fn preprocessing(s: String) -> String {
    let mut data = String::new();
    for character in s.clone().into_bytes() {
        data += &format!("{:08b} ", character);
        data.pop();
    }
    data.push('1');
    let mut len = data.len() - 1;
    while (len % 512) != 448 {
        data.push('0');
        len += 1;
    }
    let length = (s.len() * 8) as u64;
    let bigenlen = length.to_be_bytes();
    for b in bigenlen {
        data += &format!("{:08b}", b);
    }
    data.pop();
    data
}
// function for processing, that's where we do all the job
pub fn processing(s: String) {
    let mut hash_val = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
        0x1f83d9ab, 0x5be0cd19,
    ];
    let round_const = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let len = (s.len() * 8) as u64;
    let nbchunks = len / 512;
    let mut dataarray = [0u32; 64];
    for i in 0..nbchunks {
        for j in 0..16 {
            let mut val = 0u32;
            for k in 0..32 {
                let cur =
                    s.chars().nth((i * 512 + j * 32 + k) as usize).unwrap();
                val += cur.to_digit(2).unwrap() << (31 - k);
            }
            dataarray[j as usize] = val;
        }
        for j in 16..64 {
            let s0 = (dataarray[j - 15] >> 7)
                ^ (dataarray[j - 15] >> 18)
                ^ (dataarray[j - 15] >> 3);
            let s1 = (dataarray[j - 2] >> 17)
                ^ (dataarray[j - 2] >> 19)
                ^ (dataarray[j - 2] >> 10);
            dataarray[j] = dataarray[j - 16] + s0 + dataarray[j - 7] + s1;
        }
        let mut a = hash_val[0];
        let mut b = hash_val[1];
        let mut c = hash_val[2];
        let mut d = hash_val[3];
        let mut e = hash_val[4];
        let mut f = hash_val[5];
        let mut g = hash_val[6];
        let mut h = hash_val[7];
        for j in 0..64 {
            let s1 = (e >> 6) ^ (e >> 11) ^ (e >> 25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h + s1 + ch + round_const[j] + dataarray[j];
            let s0 = (a >> 2) ^ (a >> 13) ^ (a >> 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0 + maj;
            h = g;
            g = f;
            f = e;
            e = d + temp1;
            d = c;
            c = b;
            b = a;
            a = temp1 + temp2;
        }
        hash_val[0] += a;
        hash_val[1] += b;
        hash_val[2] += c;
        hash_val[3] += d;
        hash_val[4] += e;
        hash_val[5] += f;
        hash_val[6] += g;
        hash_val[7] += h;
    }
    let mut res = String::new();
    for i in 0..8 {
        res += &format!("{:08x}", hash_val[i]);
    }
    println!("{}", res);
}
