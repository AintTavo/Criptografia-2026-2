
fn main() {
    println!("Hello, world!");

    encrypt(0x4469616D616E7465, 0x4173656775726172);
    decrypt();
}

fn encrypt(
    text : u64,
    key : u64,
) {
    let (l_0, r_0) = initial_permutation(text);
    println!("Debug[L_0 Encrypt] : {:08X}", l_0);
    println!("Debug[R_0 Encrypt] : {:08X}", r_0);
    
    pc1(key);
    
}

fn decrypt() {
    
}

fn initial_permutation(
    text : u64
) -> ( u32 , u32 ){
    let mut l_0 : u32 = 0;
    let mut r_0 : u32 = 0;

    let bytes = text.to_be_bytes();
    let mut selection_bit : u8 = 0b0000_0001;
    
    for i in 0..4 {
        for j in 0..8{
            if (bytes[j] & selection_bit) == selection_bit {
                l_0 = l_0 | 0x80000000;
            }
            if j != 7 || i != 3 {l_0 >>= 1};

        }
        selection_bit <<= 2; 
    }
    
    let mut selection_bit : u8 = 0b0000_0010;
    for i in 0..4 {
        for j in 0..8{
            if (bytes[j] & selection_bit) == selection_bit {
                r_0 = r_0 | 0x80000000;
            }
            if j != 7 || i != 3 {r_0 >>= 1};

        }
        selection_bit <<= 2; 
    }

    (l_0, r_0)
}




fn pc1(
    key: u64
) -> (u32, u32) {
    let mut c_0 : u32 = 0;     // Inicialización de vector de salida
    let mut d_0 : u32 = 0;

    const PC1_C_0 : [u8; 28] = [
        57, 49, 41, 33, 25, 17, 9,
        1, 58, 50, 42, 34, 26, 18,
        10, 2, 59, 51, 43, 35, 27,
        19, 11, 3, 60, 52, 44, 36
    ]; 

    const PC1_D_0 : [u8; 28] = [
        63, 55, 47, 39, 31, 23, 15,
        7, 62, 54, 46, 38, 30, 22,
        14, 6, 61, 53, 45, 37, 29,
        21, 13, 5, 28, 20, 12, 4
    ];
    
    let bytes = key.to_be_bytes();
    let bytes = bytes.to_vec().clone();

    if bytes.len() != 8 {
        eprintln!("Error: The key is not 8 bytes long");
        return (0, 0);
    }

    

    println!("Debug[PC1_D_0] : {:02X?}", result);
    (c_0,d_0)
}

fn pc2() {
    const PC_2 : [u8; 48] = [
        14, 17, 11, 24, 1, 5,
        3, 28, 15, 6, 2, 10,
        23, 19, 12, 4, 26, 8,
        16, 7, 27, 20, 13, 2,
        41, 52, 31, 37, 47, 55,
        30, 40, 51, 45, 33, 48,
        44, 49, 39, 56, 34, 53,
        46, 42, 50, 36, 29, 32
    ];
}