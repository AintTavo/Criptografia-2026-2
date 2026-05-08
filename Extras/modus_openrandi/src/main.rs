use colored::Colorize;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/  
fn main() {
    let block = [10, 50, 9];
    let block_2 = [14, 100, 197];
    let init = [9, 99, 11];
    let key = [ 1, 2, 3, 4, 5, 6, 11, 9, 8];
    let m = 256;
    let mesage = [10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10];

    let cipher_text = hill_cipher(&block, &key, m);
    debug_block("cipher Text", &cipher_text);

    let cipher_text_ecb = modus_ecb_hc(&mesage, 3, &key, m);
    debug_block("cipher Text ECB", &cipher_text_ecb);
    
    let cipher_text_cbc = modus_cbc_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text CBC", &cipher_text_cbc);

    let cipher_text_cfb = modus_cfb_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text CFB", &cipher_text_cfb);

    let cipher_text_ofb = modus_ofb_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text OFB", &cipher_text_ofb);

    let tmp_block = block_xor(&block, &block_2);
    debug_block("Xor", &tmp_block);

}

/*  
    --------------------------------------------------------------------------------
    Main : Función principal del programa, aquí se ejecuta el código principal del programa, y se llama a las funciones necesarias para el desarrollo del programa.
    --------------------------------------------------------------------------------
*/  


fn hill_cipher( block: &[i32] , key: &[i32], m: i32) -> Vec<i32>{
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block.len() * block.len()) {
        cipher_text.push(1);
        error("The key must be a square matrix of the same size as the block");
        return cipher_text;
    }

    for i in 0..block.len() {
        let mut sum = 0;

        for j in 0..block.len() {
            sum += block[j] * key[j * block.len() + i];
        }
        sum = module(sum, m);
        cipher_text.push(sum);
    }

    return cipher_text;
}

pub fn modus_ecb_hc(
    msg : &[i32], 
    block_size : usize, 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {

    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        //debug("ECB", "Adding 1 to matrix to complete the block");
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }
    //debug_block("ECB", &_tmp_msg);

    let msg_blocks = _tmp_msg.chunks(block_size);
    //debug("ECB", "Message blocks created, starting encryption");

    for i in msg_blocks { 
        let block = i.to_vec();
        //debug_block("ECB", &block);
        let cipher_block = hill_cipher(&block, key, m);
        //debug_block("ECB", &cipher_block);
        cipher_text.extend(cipher_block);
        //debug("ECB", "Adding block");
        //debug_block("ECB", &cipher_text);
    }


    //debug("ECB", "cipher text with padding complete");
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len() );


    debug("ECB", "cipher text complete");
    debug_block("ECB", &cipher_text);
    return cipher_text;
}


pub fn modus_cbc_hc( 
    msg : &[i32], 
    block_size : usize, 
    c_0 :&[i32], 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {

    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();

    for i in msg_blocks {
        let block = i.to_vec();

        let _xor_result = block_xor(&block, &tmp_block);
        let _cipher_block = hill_cipher(&_xor_result, &key, m);
        tmp_block = _cipher_block.to_vec().clone();

        cipher_text.extend(_cipher_block);
    }

    
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len());

    return cipher_text;
}

pub fn modus_cfb_hc(
    msg : &[i32], 
    block_size : usize, 
    c_0 :&[i32], 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();

    for i in msg_blocks {
        let block = i.to_vec();

        let _cipher_block = hill_cipher(&tmp_block, &key, m);
        let _xor_result = block_xor(&block, &_cipher_block);
        tmp_block = _xor_result.to_vec().clone();

        cipher_text.extend(_xor_result);
    }

    
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len());

    return cipher_text;
}

pub fn modus_ofb_hc(
    msg : &[i32], 
    block_size : usize, 
    c_0 :&[i32], 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();

    for i in msg_blocks {
        let block = i.to_vec();

        let _c_0_ek = hill_cipher(&tmp_block, &key, m);
        let _cipher_block = block_xor(&_c_0_ek, &block);
        
        tmp_block = _c_0_ek.to_vec().clone();

        cipher_text.extend(_cipher_block);
    }

    
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len());

    return cipher_text;
}

/*  
    --------------------------------------------------------------------------------
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
    --------------------------------------------------------------------------------
*/  

fn module( a: i32 , m : i32 ) -> i32 {
    return ( ( a % m ) + m ) % m ;
}

fn block_xor( block_1 : &[i32], block_2 : &[i32]) -> Vec<i32> {
    let mut result : Vec<i32> = Vec::new();

    if block_1.len() != block_2.len() {
        error("The two blocks need to be the same size");
        result.push(1);
        return result;
    }

    for i in 0..block_1.len() {
        result.push(block_1[i] ^ block_2[i]);
    }

    return result;
}

// -> Función para imprimir una saldia formateada para error
fn error( message : &str ) {
    println!("{} {}", "Error:".red().bold(), message.red());
} 


// -> Función para imprimir una salida formateada para debug
fn debug(label : &str,  message : &str ) {
    print!("{}", "Debug [".yellow().bold());
    print!("{}", label.yellow().italic());
    print!("{} ", "]: ".yellow().bold());
    println!("{}", message.yellow());
}

fn debug_block(label : &str, block : &[i32]) {
    print!("{}", "Debug [".yellow().bold());
    print!("{}", label.yellow().italic());
    print!("{} ", "]: ".yellow().bold());
    print!("{}", "[".yellow());
    for i in block {
        print!("\t{}", i.to_string());
    }
    println!("{}", "]".yellow());
}
