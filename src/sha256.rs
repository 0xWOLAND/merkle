use crate::bitops;

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn preprocess(message: &mut String) -> Vec<[u32; 64]> {
    message.push_str("1");
    let len: u32 = message.len() as u32 * 8;
    let missing: u32 = (512 - len) / 8;
    let chunk_count: u32 = (len + 512) / 512;

    message.push_str(&"0".repeat(missing as usize));

    let mut chunks: Vec<[u32; 64]> = vec![[0; 64]; chunk_count as usize];

    for (i, chunk) in chunks.iter_mut().enumerate() {
        let block: String = String::from(&message[i * 64..(i + 1) * 64]);
        for (j, entry) in chunk[0..16].iter_mut().enumerate() {
            let mut input: u32 = 0;
            String::from(&block[j * 4..(j + 1) * 4])
                .chars()
                .enumerate()
                .for_each(|(i, c)| input |= (c as u32) << (8 * i));
            *entry = input;
        }
    }

    chunks
}

fn message_schedule(chunks: &mut Vec<[u32; 64]>) -> [u32; 8] {
    let mut H: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in chunks {
        for i in 17..64 {
            chunk[i] = chunk[i - 16]
                .wrapping_add(chunk[i - 7])
                .wrapping_add(bitops::sigma_0(&chunk[i - 15]))
                .wrapping_add(bitops::sigma_1(&chunk[i - 2]));
        }

        let mut a: u32 = H[0];
        let mut b: u32 = H[1];
        let mut c: u32 = H[2];
        let mut d: u32 = H[3];
        let mut e: u32 = H[4];
        let mut f: u32 = H[5];
        let mut g: u32 = H[6];
        let mut h: u32 = H[7];

        for t in 0..64 {
            let t1: u32 = bitops::T1(&h, &e, &f, &g, &K[t], &chunk[t]);
            let t2: u32 = bitops::T2(&a, &b, &c);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);

            H[0] = H[0].wrapping_add(a);
            H[1] = H[1].wrapping_add(b);
            H[2] = H[2].wrapping_add(c);
            H[3] = H[3].wrapping_add(d);
            H[4] = H[4].wrapping_add(e);
            H[5] = H[5].wrapping_add(f);
            H[6] = H[6].wrapping_add(g);
            H[7] = H[7].wrapping_add(h);
        }
    }
    H
}

pub fn hash(message: &mut String) {
    let mut chunks: Vec<[u32; 64]> = preprocess(message);
    let H: [u32; 8] = message_schedule(&mut chunks);
    dbg!(H);
}
