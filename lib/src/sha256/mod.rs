// transforms a string into a binary vector
fn preprocessing<T>(input: T) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    let input = input.as_ref();
    let mut data = input.to_vec();
    let bit_len = (data.len() * 8) as u64;
    data.push(0x80);
    while (data.len() * 8) % 512 != 448 {
        data.push(0x00);
    }

    data.extend_from_slice(&bit_len.to_be_bytes());
    return data;
}

// function for processing, that's where we do all the job
fn processing(data: &Vec<u8>) -> [u32; 8] {
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
    let nbchunks = data.len() / 64;
    let mut dataarray = [0u32; 64];
    for i in 0..nbchunks {
        for j in 0..16 {
            dataarray[j] = u32::from_be_bytes([
                data[i * 64 + j * 4],
                data[i * 64 + j * 4 + 1],
                data[i * 64 + j * 4 + 2],
                data[i * 64 + j * 4 + 3],
            ]);
        }
        for j in 16..64 {
            let s0 = dataarray[j - 15].rotate_right(7)
                ^ dataarray[j - 15].rotate_right(18)
                ^ (dataarray[j - 15] >> 3);
            let s1 = dataarray[j - 2].rotate_right(17)
                ^ dataarray[j - 2].rotate_right(19)
                ^ (dataarray[j - 2] >> 10);
            dataarray[j] = dataarray[j - 16]
                .wrapping_add(s0)
                .wrapping_add(dataarray[j - 7])
                .wrapping_add(s1);
        }
        let mut a: u32 = hash_val[0];
        let mut b: u32 = hash_val[1];
        let mut c: u32 = hash_val[2];
        let mut d: u32 = hash_val[3];
        let mut e: u32 = hash_val[4];
        let mut f: u32 = hash_val[5];
        let mut g: u32 = hash_val[6];
        let mut h: u32 = hash_val[7];
        for j in 0..64 {
            let s1 =
                e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(round_const[j])
                .wrapping_add(dataarray[j]);
            let s0 =
                a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        hash_val[0] = hash_val[0].wrapping_add(a);
        hash_val[1] = hash_val[1].wrapping_add(b);
        hash_val[2] = hash_val[2].wrapping_add(c);
        hash_val[3] = hash_val[3].wrapping_add(d);
        hash_val[4] = hash_val[4].wrapping_add(e);
        hash_val[5] = hash_val[5].wrapping_add(f);
        hash_val[6] = hash_val[6].wrapping_add(g);
        hash_val[7] = hash_val[7].wrapping_add(h);
    }

    return hash_val;
}

pub fn sha256<T>(s: T) -> [u8; 32]
where
    T: AsRef<[u8]>,
{
    let s = s.as_ref();
    let preprocessed_data = preprocessing(s);
    let hash = processing(&preprocessed_data);

    let mut result = [0u8; 32];
    for i in 0..8 {
        result[i * 4] = (hash[i] >> 24) as u8;
        result[i * 4 + 1] = (hash[i] >> 16) as u8;
        result[i * 4 + 2] = (hash[i] >> 8) as u8;
        result[i * 4 + 3] = hash[i] as u8;
    }
    return result;
}
