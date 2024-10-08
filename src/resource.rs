pub const CURTAIN_H: i32 = 350;

pub const GRADIENT: [u8; 4] = [0b11101010, 0b01010000, 0b11101000, 0b11010000];
pub const GRADIENT_W: i32 = 4;
pub const GRADIENT_H: i32 = 8;

pub const ARROW_HEAD: [u8; 6] = [0xfd, 0x5f, 0x51, 0xd4, 0x15, 0x01];
pub const ARROW_HEAD_W: u32 = 6;
pub const ARROW_HEAD_H: u32 = 4;

pub const ARROW_TAIL: [u8; 4] = [0b01110111, 0b00110011, 0b00010001, 0b00000000];
pub const ARROW_TAIL_W: u32 = 4;
pub const ARROW_TAIL_H: u32 = 8;

pub const MOTA_W: u32 = 48;
pub const MOTA_H: u32 = 42;
pub const MOTA: [u8; 504] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x00, 0x00, 0x00, 0xaa, 0x50, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xa5, 0x00, 0x00, 0x00, 0xaa, 0x50, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x0a, 0x55, 0xfa,
    0xaa, 0x55, 0x55, 0x50, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x0a, 0x55, 0xfa, 0xaa, 0x55, 0x55, 0x50,
    0x00, 0x00, 0x00, 0x0a, 0x55, 0xfa, 0x55, 0x5f, 0xa5, 0x5a, 0x55, 0x50, 0x00, 0x00, 0x00, 0x0a,
    0x55, 0xfa, 0x55, 0x5f, 0xa5, 0x5a, 0x55, 0x50, 0x00, 0x00, 0x00, 0xf5, 0x5f, 0xff, 0xaa, 0x55,
    0xff, 0xfa, 0x55, 0x50, 0x00, 0x00, 0x00, 0xf5, 0x5f, 0xff, 0xaa, 0x55, 0xff, 0xfa, 0x55, 0x50,
    0xf5, 0x00, 0x00, 0xfa, 0xa5, 0x5f, 0xa0, 0x05, 0x5f, 0xa5, 0x55, 0x00, 0xf5, 0x00, 0x00, 0xfa,
    0xa5, 0x5f, 0xa0, 0x05, 0x5f, 0xa5, 0x55, 0x00, 0xff, 0xf5, 0x00, 0x50, 0x0f, 0x5f, 0xa5, 0x55,
    0x5a, 0x55, 0xa0, 0x00, 0xff, 0xf5, 0x00, 0x50, 0x0f, 0x5f, 0xa5, 0x55, 0x5a, 0x55, 0xa0, 0x00,
    0xff, 0xaa, 0xaf, 0x00, 0x5a, 0xf5, 0xf5, 0x55, 0x55, 0xaa, 0x00, 0x00, 0xff, 0xaa, 0xaf, 0x00,
    0x5a, 0xf5, 0xf5, 0x55, 0x55, 0xaa, 0x00, 0x00, 0xfa, 0xaa, 0xff, 0x00, 0xa0, 0xff, 0xaa, 0xa5,
    0x55, 0xa0, 0x00, 0x00, 0xfa, 0xaa, 0xff, 0x00, 0xa0, 0xff, 0xaa, 0xa5, 0x55, 0xa0, 0x00, 0x00,
    0xfa, 0xa0, 0xff, 0x50, 0x05, 0x5f, 0xa0, 0x00, 0x55, 0x55, 0x50, 0x00, 0xfa, 0xa0, 0xff, 0x50,
    0x05, 0x5f, 0xa0, 0x00, 0x55, 0x55, 0x50, 0x00, 0xfa, 0x55, 0xff, 0x50, 0x05, 0x5f, 0xa5, 0x55,
    0x55, 0x55, 0x00, 0x00, 0xfa, 0x55, 0xff, 0x50, 0x05, 0x5f, 0xa5, 0x55, 0x55, 0x55, 0x00, 0x00,
    0xfa, 0xaa, 0xff, 0x00, 0x5a, 0xf5, 0xf5, 0x55, 0x55, 0xa0, 0x00, 0x00, 0xfa, 0xaa, 0xff, 0x00,
    0x5a, 0xf5, 0xf5, 0x55, 0x55, 0xa0, 0x00, 0x00, 0xaa, 0xaa, 0xaf, 0x00, 0xa0, 0xff, 0xaa, 0xa5,
    0x55, 0x5a, 0x00, 0x00, 0xaa, 0xaa, 0xaf, 0x00, 0xa0, 0xff, 0xaa, 0xa5, 0x55, 0x5a, 0x00, 0x00,
    0xaa, 0xa5, 0x00, 0x50, 0x0f, 0x5f, 0xa0, 0x05, 0x5a, 0x55, 0x50, 0x00, 0xaa, 0xa5, 0x00, 0x50,
    0x0f, 0x5f, 0xa0, 0x05, 0x5a, 0x55, 0x50, 0x00, 0xaf, 0xf0, 0x00, 0xfa, 0xa5, 0x5f, 0xa5, 0x55,
    0x5f, 0xa5, 0x50, 0x00, 0xaf, 0xf0, 0x00, 0xfa, 0xa5, 0x5f, 0xa5, 0x55, 0x5f, 0xa5, 0x50, 0x00,
    0xff, 0xf0, 0x00, 0xf5, 0x5f, 0xfa, 0xf5, 0x55, 0xff, 0xfa, 0x5a, 0x00, 0xff, 0xf0, 0x00, 0xf5,
    0x5f, 0xfa, 0xf5, 0x55, 0xff, 0xfa, 0x5a, 0x00, 0xff, 0xaf, 0x00, 0x0a, 0x55, 0xfa, 0x55, 0x5f,
    0xaa, 0xaa, 0x55, 0x00, 0xff, 0xaf, 0x00, 0x0a, 0x55, 0xfa, 0x55, 0x5f, 0xaa, 0xaa, 0x55, 0x00,
    0xff, 0xaa, 0x00, 0x00, 0xa5, 0x0a, 0x55, 0xfa, 0xaa, 0xa5, 0x55, 0x00, 0xff, 0xaa, 0x00, 0x00,
    0xa5, 0x0a, 0x55, 0xfa, 0xaa, 0xa5, 0x55, 0x00, 0x0f, 0xaa, 0xa5, 0x00, 0x00, 0x00, 0xa5, 0x00,
    0x00, 0xa5, 0x50, 0x00, 0x0f, 0xaa, 0xa5, 0x00, 0x00, 0x00, 0xa5, 0x00, 0x00, 0xa5, 0x50, 0x00,
    0x0f, 0xaa, 0x55, 0x55, 0x50, 0x00, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00, 0x0f, 0xaa, 0x55, 0x55,
    0x50, 0x00, 0x00, 0x00, 0x00, 0xaa, 0x00, 0x00, 0x00, 0xfa, 0x55, 0x50, 0x50, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0x55, 0x50, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xa5, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x55,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const C: [u8; 3] = [0b11111000, 0b00110011, 0b00110000];
pub const O: [u8; 3] = [0b11111001, 0b00100010, 0b00100000];
pub const N: [u8; 3] = [0b11110001, 0b00100010, 0b00100010];
pub const T: [u8; 3] = [0b11110000, 0b10011001, 0b10011001];
pub const I: [u8; 3] = [0b11111001, 0b10011001, 0b10011001];
pub const U: [u8; 3] = [0b11110010, 0b00100010, 0b00100001];
pub const E: [u8; 3] = [0b11111000, 0b00110001, 0b00110000];
pub const CHAR_W: u32 = 4;
pub const CHAR_H: u32 = 6;
