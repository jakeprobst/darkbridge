


const CIPHER_LEN: usize = 521;















pub struct Cipher {
    seed: u32,
    key: [u32; 521],
    block_idx: usize,
}


impl Cipher {
    pub fn new(seed: u32) -> Cipher {
        let mut cipher = Cipher {
            seed: seed,
            key: [0; 521],
            block_idx: 0,
        };
        cipher.init_keys();
        cipher
    }

    fn mix_keys(&mut self) {
        self.block_idx = 0;
        let mut idx1 = self.block_idx;
        let mut idx2 = 489;
        let mut idx3 = self.block_idx;

        while idx2 != CIPHER_LEN {
            let mut r0 = self.key[idx2];
            idx2 += 1;
            let r4 = self.key[idx1];
            r0 ^= r4;
            self.key[idx1] = r0;
            idx1 += 1;
        }

        while idx1 != CIPHER_LEN {
            let mut r0 = self.key[idx3];
            idx3 += 1;
            let r4 = self.key[idx1];
            r0 ^= r4;
            self.key[idx1] = r0;
            idx1 += 1;
        }
    }
    
    fn init_keys(&mut self) {
        let mut seed = self.seed;
        let mut basekey: u32 = 0;
        for _ in 0..=16 {
            for _ in 0..32 {
                seed = seed.wrapping_mul(0x5D588B65);
                basekey = basekey >> 1;
                seed += 1;
                basekey = if seed & 0x80000000 != 0 {
                    basekey | 0x80000000
                }
                else {
                    basekey & 0x7FFFFFFF
                }
            }
            self.key[self.block_idx] = basekey;
            self.block_idx += 1;
        }

        self.block_idx -= 1;
        self.key[self.block_idx] = (self.key[0] >> 9) ^ (self.key[self.block_idx] << 23) ^ self.key[15];

        let mut source1 = 0;
        let mut source2 = 1;
        let mut source3 = self.block_idx;
        
        self.block_idx += 1;
        while self.block_idx != CIPHER_LEN {
            self.key[self.block_idx] = self.key[source3] ^ (((self.key[source1] << 23) & 0xFF800000) ^ ((self.key[source2] >> 9) & 0x007FFFFF));
            self.block_idx += 1;

            source1 = source1 + 1;
            source2 = source2 + 1;
            source3 = source3 + 1;
        }

        self.mix_keys();
        self.mix_keys();
        self.mix_keys();
        self.block_idx = 520;
        
    }

    fn next_key(&mut self) -> [u8; 4]{
        self.block_idx += 1;
        if self.block_idx == CIPHER_LEN {
            self.mix_keys();
        }
        let key = self.key[self.block_idx];
        [key as u8, (key >> 8) as u8, (key >> 16) as u8, (key >> 24) as u8]
    }
    
    pub fn encrypt(&mut self, buf: Vec<u8>) -> Vec<u8> {
        let mut out = Vec::new();
        for chunk in buf.chunks(4) {
            let key = self.next_key();
            for (i, c) in chunk.iter().enumerate() {
                out.push(key[i] ^ c)
            }
        }
        out
    }
}

