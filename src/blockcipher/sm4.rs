// GM/T 0002-2012 SM4分组密码算法标准（中文版本）
// https://github.com/guanzhi/GM-Standards/blob/master/GMT%E6%AD%A3%E5%BC%8F%E6%A0%87%E5%87%86/GMT%200002-2012%20SM4%E5%88%86%E7%BB%84%E5%AF%86%E7%A0%81%E7%AE%97%E6%B3%95.pdf
// 
// GM/T 0002-2012 SM4 Block Cipher Algorithm （English Version）
// http://www.gmbz.org.cn/upload/2018-04-04/1522788048733065051.pdf
// http://sca.hainan.gov.cn/xxgk/bzhgf/201804/W020180409400793061524.pdf
// 
// NOTE: 
//      1. 硬件加速方面的思路和代码可以参考项目: https://github.com/mjosaarinen/sm4ni
//      2. 代码参考自 https://github.com/citahub/libsm/blob/master/src/sm4/cipher.rs
// 
//          https://github.com/randombit/botan/blob/master/src/lib/block/sm4/sm4_armv8/sm4_armv8.cpp

const FK: [u32; 4]  = [ 0xa3b1_bac6, 0x56aa_3350, 0x677d_9197, 0xb270_22dc ];
const CK: [u32; 32] = [
    0x00070e15, 0x1c232a31, 0x383f464d, 0x545b6269, 0x70777e85, 0x8c939aa1, 0xa8afb6bd, 0xc4cbd2d9,
    0xe0e7eef5, 0xfc030a11, 0x181f262d, 0x343b4249, 0x50575e65, 0x6c737a81, 0x888f969d, 0xa4abb2b9,
    0xc0c7ced5, 0xdce3eaf1, 0xf8ff060d, 0x141b2229, 0x30373e45, 0x4c535a61, 0x686f767d, 0x848b9299,
    0xa0a7aeb5, 0xbcc3cad1, 0xd8dfe6ed, 0xf4fb0209, 0x10171e25, 0x2c333a41, 0x484f565d, 0x646b7279,
];
const SBOX: [u8; 256] = [
    0xd6, 0x90, 0xe9, 0xfe, 0xcc, 0xe1, 0x3d, 0xb7, 0x16, 0xb6, 0x14, 0xc2, 0x28, 0xfb, 0x2c, 0x05,
    0x2b, 0x67, 0x9a, 0x76, 0x2a, 0xbe, 0x04, 0xc3, 0xaa, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
    0x9c, 0x42, 0x50, 0xf4, 0x91, 0xef, 0x98, 0x7a, 0x33, 0x54, 0x0b, 0x43, 0xed, 0xcf, 0xac, 0x62,
    0xe4, 0xb3, 0x1c, 0xa9, 0xc9, 0x08, 0xe8, 0x95, 0x80, 0xdf, 0x94, 0xfa, 0x75, 0x8f, 0x3f, 0xa6,
    0x47, 0x07, 0xa7, 0xfc, 0xf3, 0x73, 0x17, 0xba, 0x83, 0x59, 0x3c, 0x19, 0xe6, 0x85, 0x4f, 0xa8,
    0x68, 0x6b, 0x81, 0xb2, 0x71, 0x64, 0xda, 0x8b, 0xf8, 0xeb, 0x0f, 0x4b, 0x70, 0x56, 0x9d, 0x35,
    0x1e, 0x24, 0x0e, 0x5e, 0x63, 0x58, 0xd1, 0xa2, 0x25, 0x22, 0x7c, 0x3b, 0x01, 0x21, 0x78, 0x87,
    0xd4, 0x00, 0x46, 0x57, 0x9f, 0xd3, 0x27, 0x52, 0x4c, 0x36, 0x02, 0xe7, 0xa0, 0xc4, 0xc8, 0x9e,
    0xea, 0xbf, 0x8a, 0xd2, 0x40, 0xc7, 0x38, 0xb5, 0xa3, 0xf7, 0xf2, 0xce, 0xf9, 0x61, 0x15, 0xa1,
    0xe0, 0xae, 0x5d, 0xa4, 0x9b, 0x34, 0x1a, 0x55, 0xad, 0x93, 0x32, 0x30, 0xf5, 0x8c, 0xb1, 0xe3,
    0x1d, 0xf6, 0xe2, 0x2e, 0x82, 0x66, 0xca, 0x60, 0xc0, 0x29, 0x23, 0xab, 0x0d, 0x53, 0x4e, 0x6f,
    0xd5, 0xdb, 0x37, 0x45, 0xde, 0xfd, 0x8e, 0x2f, 0x03, 0xff, 0x6a, 0x72, 0x6d, 0x6c, 0x5b, 0x51,
    0x8d, 0x1b, 0xaf, 0x92, 0xbb, 0xdd, 0xbc, 0x7f, 0x11, 0xd9, 0x5c, 0x41, 0x1f, 0x10, 0x5a, 0xd8,
    0x0a, 0xc1, 0x31, 0x88, 0xa5, 0xcd, 0x7b, 0xbd, 0x2d, 0x74, 0xd0, 0x12, 0xb8, 0xe5, 0xb4, 0xb0,
    0x89, 0x69, 0x97, 0x4a, 0x0c, 0x96, 0x77, 0x7e, 0x65, 0xb9, 0xf1, 0x09, 0xc5, 0x6e, 0xc6, 0x84,
    0x18, 0xf0, 0x7d, 0xec, 0x3a, 0xdc, 0x4d, 0x20, 0x79, 0xee, 0x5f, 0x3e, 0xd7, 0xcb, 0x39, 0x48,
];


#[inline]
fn tau_trans(input: u32) -> u32 {
    let mut octets = input.to_be_bytes();
    octets[0] = SBOX[octets[0] as usize];
    octets[1] = SBOX[octets[1] as usize];
    octets[2] = SBOX[octets[2] as usize];
    octets[3] = SBOX[octets[3] as usize];
    u32::from_be_bytes(octets)
}
#[inline]
fn l_rotate(x: u32, i: u32) -> u32 {
    (x << (i % 32)) | (x >> (32 - (i % 32)))
}
#[inline]
fn l_trans(b: u32) -> u32 {
    b ^ l_rotate(b, 2) ^ l_rotate(b, 10) ^ l_rotate(b, 18) ^ l_rotate(b, 24)
}
#[inline]
fn t_trans(input: u32) -> u32 {
    l_trans(tau_trans(input))
}
#[inline]
fn l_prime_trans(b: u32) -> u32 {
    b ^ l_rotate(b, 13) ^ l_rotate(b, 23)
}
#[inline]
fn t_prime_trans(input: u32) -> u32 {
    l_prime_trans(tau_trans(input))
}


/// GM/T 0002-2012 SM4分组密码算法
#[derive(Debug, Clone)]
pub struct Sm4 {
    // round key
    rk: [[u32; 4]; Self::NR],
}

impl Sm4 {
    pub const KEY_LEN: usize   = 16;
    pub const BLOCK_LEN: usize = 16;
    
    // Rounds
    const NR: usize = 8;


    pub fn new(key: &[u8]) -> Self {
        assert_eq!(key.len(), Self::KEY_LEN);

        let mut k: [u32; 4] = [
            u32::from_be_bytes([key[ 0], key[ 1], key[ 2], key[ 3]]) ^ FK[0],
            u32::from_be_bytes([key[ 4], key[ 5], key[ 6], key[ 7]]) ^ FK[1],
            u32::from_be_bytes([key[ 8], key[ 9], key[10], key[11]]) ^ FK[2],
            u32::from_be_bytes([key[12], key[13], key[14], key[15]]) ^ FK[3],
        ];

        let mut rk = [[0u32; 4]; Self::NR];
        for i in 0..Self::NR {
            k[0] ^= t_prime_trans(k[1] ^ k[2] ^ k[3] ^ CK[i * 4]);
            k[1] ^= t_prime_trans(k[2] ^ k[3] ^ k[0] ^ CK[i * 4 + 1]);
            k[2] ^= t_prime_trans(k[3] ^ k[0] ^ k[1] ^ CK[i * 4 + 2]);
            k[3] ^= t_prime_trans(k[0] ^ k[1] ^ k[2] ^ CK[i * 4 + 3]);
            rk[i] = k;
        }

        Self { rk }
    }

    pub fn encrypt(&self, block: &mut [u8]) {
        debug_assert_eq!(block.len(), Self::BLOCK_LEN);

        let mut x: [u32; 4] = [
            u32::from_be_bytes([block[ 0], block[ 1], block[ 2], block[ 3]]),
            u32::from_be_bytes([block[ 4], block[ 5], block[ 6], block[ 7]]),
            u32::from_be_bytes([block[ 8], block[ 9], block[10], block[11]]),
            u32::from_be_bytes([block[12], block[13], block[14], block[15]]),
        ];

        for i in 0..Self::NR {
            x[0] ^= t_trans(x[1] ^ x[2] ^ x[3] ^ self.rk[i][0] );
            x[1] ^= t_trans(x[2] ^ x[3] ^ x[0] ^ self.rk[i][1]);
            x[2] ^= t_trans(x[3] ^ x[0] ^ x[1] ^ self.rk[i][2]);
            x[3] ^= t_trans(x[0] ^ x[1] ^ x[2] ^ self.rk[i][3]);
        }

        block[ 0.. 4].copy_from_slice(&x[3].to_be_bytes());
        block[ 4.. 8].copy_from_slice(&x[2].to_be_bytes());
        block[ 8..12].copy_from_slice(&x[1].to_be_bytes());
        block[12..16].copy_from_slice(&x[0].to_be_bytes());
    }

    pub fn decrypt(&self, block: &mut [u8]) {
        debug_assert_eq!(block.len(), Self::BLOCK_LEN);

        let mut x: [u32; 4] = [
            u32::from_be_bytes([block[ 0], block[ 1], block[ 2], block[ 3]]),
            u32::from_be_bytes([block[ 4], block[ 5], block[ 6], block[ 7]]),
            u32::from_be_bytes([block[ 8], block[ 9], block[10], block[11]]),
            u32::from_be_bytes([block[12], block[13], block[14], block[15]]),
        ];

        for i in 0..Self::NR {
            x[0] ^= t_trans(x[1] ^ x[2] ^ x[3] ^ self.rk[Self::NR - i - 1][3]);
            x[1] ^= t_trans(x[2] ^ x[3] ^ x[0] ^ self.rk[Self::NR - i - 1][2]);
            x[2] ^= t_trans(x[3] ^ x[0] ^ x[1] ^ self.rk[Self::NR - i - 1][1]);
            x[3] ^= t_trans(x[0] ^ x[1] ^ x[2] ^ self.rk[Self::NR - i - 1][0]);
        }
        
        block[ 0.. 4].copy_from_slice(&x[3].to_be_bytes());
        block[ 4.. 8].copy_from_slice(&x[2].to_be_bytes());
        block[ 8..12].copy_from_slice(&x[1].to_be_bytes());
        block[12..16].copy_from_slice(&x[0].to_be_bytes());
    }
}


// Tests below
#[test]
fn test_sm4_setup_cipher() {
    let key: [u8; Sm4::KEY_LEN] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 
    ];

    let cipher = Sm4::new(&key);
    
    assert_eq!(cipher.rk[0][0], 0xf12186f9);
    assert_eq!(cipher.rk[Sm4::NR - 1][3], 0x9124a012);
}

#[test]
fn test_sm4_enc_and_dec() {
    let key: [u8; Sm4::KEY_LEN] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 
    ];
    let plaintext: [u8; Sm4::BLOCK_LEN] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
    ];

    let cipher = Sm4::new(&key);

    let mut ciphertext = plaintext.clone();
    cipher.encrypt(&mut ciphertext);
    assert_eq!(&ciphertext[..], &[
        0x68, 0x1e, 0xdf, 0x34, 0xd2, 0x06, 0x96, 0x5e,
        0x86, 0xb3, 0xe9, 0x4f, 0x53, 0x6e, 0x42, 0x46,
    ]);

    cipher.decrypt(&mut ciphertext);
    assert_eq!(&ciphertext[..], &plaintext[..]);
}
