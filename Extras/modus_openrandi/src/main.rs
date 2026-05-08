use colored::Colorize;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/  
fn main() {
    let block = [10, 50, 9];
    let key = [ 1, 2, 3, 4, 5, 6, 11, 9, 8];
    let m = 256;
    let mesage = [10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10];

    let cipher_text = hill_cypher(&block, &key, m);
    debug_block("Cipher Text", &cipher_text);

    let cipher_text_ecb = modus_ecb_hc(&mesage, 3, &key, m);
    debug_block("Cipher Text ECB", &cipher_text_ecb);

}

/*  
    --------------------------------------------------------------------------------
    Main : Función principal del programa, aquí se ejecuta el código principal del programa, y se llama a las funciones necesarias para el desarrollo del programa.
    --------------------------------------------------------------------------------
*/  

fn hill_cypher( block: &[i32] , key: &[i32], m: i32) -> Vec<i32>{
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block.len() * block.len()) {
        cipher_text.push(-1);
        error("The key must be a square matrix of the same size as the block");
        return cipher_text;
    }

    for i in 0..block.len() {
        let mut sum = 0;

        for j in 0..block.len() {
            //debug("Block Element", &block[j].to_string());
            //debug("Key Element", &key[j * block.len() + i].to_string());
            sum += block[j] * key[j * block.len() + i];
        }
        //debug("Sum", &sum.to_string());
        sum = module(sum, m);
        //debug("Sum", &sum.to_string());
        cipher_text.push(sum);
    }

    return cipher_text;
}

fn modus_ecb_hc( msg : &[i32] , block_size : usize , key : &[i32] , m : i32 ) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();
    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        debug("ECB", "Adding 1 to matrix to complete the block");
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }
    debug_block("ECB", &_tmp_msg);

    let msg_blocks = _tmp_msg.chunks(block_size);
    debug("ECB", "Message blocks created, starting encryption");

    for i in msg_blocks { 
        let block = i.to_vec();
        debug_block("ECB", &block);
        let cipher_block = hill_cypher(&block, key, m);
        debug_block("ECB", &cipher_block);
        cipher_text.extend(cipher_block);
        debug("ECB", "Adding block");
        debug_block("ECB", &cipher_text);
    }


    debug("ECB", "Cypher text with padding complete");
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len() );


    debug("ECB", "Cypher text complete");
    debug_block("ECB", &cipher_text);
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

fn block_xor( block_1 : &[i32] , block_2 : &[i32] ) -> Vec<i32> {
    
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
