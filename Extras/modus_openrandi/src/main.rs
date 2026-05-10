// Librerias estandar
use std::collections::HashMap;
use std::iter::*;

// Crates externas
use colored::Colorize;
use matrix_operations::matrix_inverse_module;
use rand::random;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/  
fn main() {
    let init = [9, 99, 11];
    let key = [ 1, 2, 3, 4, 5, 6, 11, 9, 8];
    let m = 256;
    let mesage = [10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50];
    debug_block("Original", &mesage);

    let _ = modus_pcbc_hc_cipher(&mesage, 3, &init, &key, m);
    let _ = modus_ctr_hc_cipher(&mesage, 3, &key, m);

    // Prueba ECB
    let (_p,cipher_text) = modus_ecb_hc_cipher(&mesage, 3, &key, m);
    debug_block("ECB_C", &cipher_text);
    let plain = modus_ecb_hc_decipher(&cipher_text, &key, _p, m);
    debug_block("ECB_D", &plain);

    // Prueba CBC
    let (_p,cipher_text) = modus_cbc_hc_cipher(&mesage, 3, &init, &key, m);
    debug_block("CBC_C", &cipher_text);
    let plain = modus_cbc_hc_decipher(&cipher_text, &init, &key, _p, m);
    debug_block("CBC_D", &plain);

    let (_p,cipher_text) = modus_cfb_hc_cipher(&mesage, 3, &init, &key, m);
    debug_block("CFB_C", &cipher_text);
    let plain = modus_cfb_hc_decipher(&cipher_text, &init, &key, _p, m);
    debug_block("CFB_D", &plain);

    let (_p, cipher_text) = modus_ofb_hc_cipher(&mesage, 3, &init, &key, m);
    debug_block("OFB_C", &cipher_text);
    let plain = modus_ofb_hc_decipher(&cipher_text, &init, &key, _p, m);
    debug_block("OFB_D", &plain);
}

/*  
    --------------------------------------------------------------------------------
    Main : Función principal del programa, aquí se ejecuta el código principal del programa, y se llama a las funciones necesarias para el desarrollo del programa.
    --------------------------------------------------------------------------------
*/  

// ##########################
// Sub main : Hill 
// ##########################

// -> Hill Cipher : Aplicacion del cifrador en hill basado en algebra lineal.
fn hill_cipher( 
    block: &[i32], 
    key: &[i32], 
    m: i32
) -> Vec<i32> {

    // Variable de retorno
    let mut cipher_text : Vec<i32> = Vec::new();

    // # Correción de errores HC_CIPHER(1):
    // La clave tiene que ser al menos del tamaño del blocke al cuadrado, sino el cifrado es imposible de realizar
    if key.len() != (block.len() * block.len()) {
        cipher_text.push(1);
        error("The key must be a square matrix of the same size as the block");
        return cipher_text;
    }

    // Ciclo: Realiza una multiplicación de matrices
    for i in 0..block.len() {
        let mut sum = 0;

        for j in 0..block.len() {
            sum += block[j] * key[j * block.len() + i];
        }
        sum = module(sum, m);
        cipher_text.push(sum);
    }

    // Regresa el texto cifrado
    return cipher_text;
}


// ##########################
// Sub main : Modus Cipher 
// ##########################

// -> Modo de operación Electronic Code Book (ECB) con Hill  cipher: Se aplica el cifrado directamente en cada bloque
// C = Ek(p)
pub fn modus_ecb_hc_cipher(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque a encriptar
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> (usize, Vec<i32>) {
    let mut cipher_text : Vec<i32> = Vec::new();                            // Variable de retorno

    // # Correción de errores ECB(1):
    // Si la llave no corresponde con el tamaño correcto para el cifrado hill retorna.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return (0,cipher_text);
    }

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores ECB(2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    // Mapa para guardar datos calculados y ahorrar operaciónes
    let mut cipher_map : HashMap< Vec<i32>, Vec<i32> > = HashMap::new(); 
    
    // Ciclo : Electronic CodeBook main loop, da una pasada por todos los bloques del cifrado
    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        let cipher_block : Vec<i32>;                    // Se crea un cipher block temporal

        match cipher_map.get(&block) {                  // Se busca en el hash map si ya se ha calculado el valor y si:
            Some(p) => cipher_block = p.to_vec(),       // Si existe se pasa directo como cipher_block
            None => {                                   // Si no existe, se calcula y añade a la tabla hash
                cipher_block = hill_cipher(&block, key, m);
                cipher_map.entry(block).or_insert(cipher_block.clone());
            }
        }
        cipher_text.extend(cipher_block);   // Se agrega el bloque cifrado al final del texto cifado
    }

    return (padding, cipher_text);
}


// -> Modo de operación Cipher Block Chaining (CBC) :
// Cn = Ek(p xor Cn-1)
pub fn modus_cbc_hc_cipher( 
    msg : &[i32],           // Mensaje a encriptar
    block_size : usize,     // Tamaño del bloque
    c_0 :&[i32],            // Bloque de cifrado original
    key : &[i32],           // Llave
    m : i32                 // Modulo
) -> (usize, Vec<i32>)  {

    let mut cipher_text : Vec<i32> = Vec::new();     // Variable de retorno

    // Correción de errores CBC (1):
    // Si la llave para el cifrado hill no es el cuadrado del tamaño del bloque retorna 1.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return (0,cipher_text);
    }

    // Correción de errores CBC (2):
    // Si el bloque de cifrado inicial no corresponde con el tamaño correcto retorna 1.
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return (0,cipher_text);
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores CBC (2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    let mut tmp_block : Vec<i32> = c_0.to_vec(); // Se inicializa el valor del bloque temporal con el del bloque inicial

    // # Ciclo : Main Loop de Cipher Block Chaining 
    for i in msg_blocks {
        let block = i.to_vec();                                 // i es un apuntador entonce se convierte a un vector

        let _xor_result = block_xor(&block, &tmp_block);        // Se aplica una xor con el bloque anterior 
        let _cipher_block = hill_cipher(&_xor_result, &key, m); // Se aplica el cifrado al bloque
        tmp_block = _cipher_block.clone();                      // se pasa el nuevo bloque al bloque actual

        cipher_text.extend(_cipher_block);                      // Se guarda el bloque cifrado en el texto final
    }

    return (padding, cipher_text);
}


// -> Modo de operación Cipher Feedback (CFB) : 
// Cn = Ek(Cn-1) xor p 
pub fn modus_cfb_hc_cipher(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> (usize, Vec<i32>) {
    let mut cipher_text : Vec<i32> = Vec::new(); // Variable de retorno

    // # Corrección de errores CFB(1):
    // Verificación de integridad de la llave para el cifrado Hill
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return (0,cipher_text);
    }

    // # Corrección de errores CFB(2):
    // Verificación de que el IV tenga el tamaño de bloque correspondiente
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return (0,cipher_text);
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                            // Variable para reajustar el mensaje
    let padding = ( block_size - ( msg.len() % block_size ) ) % block_size;                // Cálculo del relleno necesario

    _tmp_msg = msg.to_vec().clone();                                    // Clonación del mensaje original

    // # Corrección de errores CFB(3):
    // Relleno de bloques incompletos con 1's si no es múltiplo del tamaño de bloque
    _tmp_msg.extend(repeat(1).take(padding));

    // División del mensaje en bloques del tamaño especificado
    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();                // Registro temporal para encadenamiento

    // # Ciclo : Main Loop de Cipher Feedback
    for i in msg_blocks {
        let block = i.to_vec();

        let _c_ek = hill_cipher(&tmp_block, &key, m);                   // Se cifra el bloque anterior o IV
        let _cipher_block = block_xor(&block, &_c_ek);                  // XOR entre texto plano y salida del cifrador
        tmp_block = _cipher_block.to_vec().clone();                     // La salida cifrada alimenta el siguiente bloque

        cipher_text.extend(_cipher_block);                              // Se anexa al resultado final
    }

    return (padding, cipher_text);
}


// -> Modo de operación Output Feedback (OFB) :
// C = Ek(Co) xor p
pub fn modus_ofb_hc_cipher(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> (usize, Vec<i32>) {
    let mut cipher_text : Vec<i32> = Vec::new(); // Variable de retorno

    // # Corrección de errores OFB(1):
    // Verificación de tamaño de llave
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return (0,cipher_text);
    }

    // # Corrección de errores OFB(2):
    // Verificación de tamaño del bloque inicial
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return (0,cipher_text);
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                            // Variable auxiliar para el mensaje
    let padding = ( block_size - ( msg.len() % block_size ) ) % block_size;                // Variable de ajuste de tamaño

    _tmp_msg = msg.to_vec().clone();

    // Relleno manual de bloques si el mensaje no es múltiplo del bloque
    _tmp_msg.extend(repeat(1).take(padding));

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();                // Bloque para retroalimentación de salida

    // # Ciclo : Main Loop de Output Feedback
    for i in msg_blocks {
        let block = i.to_vec();

        let _c_0_ek = hill_cipher(&tmp_block, &key, m);                 // Se cifra el flujo de salida independientemente del mensaje
        let _cipher_block = block_xor(&_c_0_ek, &block);                // XOR entre el flujo cifrado y el texto plano
        
        tmp_block = _c_0_ek.clone();                                    // La salida del cifrador (antes de XOR) se retroalimenta

        cipher_text.extend(_cipher_block);
    }


    return (padding, cipher_text);
}

// -> Modo de operación Propagating Cipher Block Chaining (PCBC)
// n = 0    |   C = Ek ( p xor Co )
// n = 1    |   C = Ek ( (p n-1 xor C n-1) xor p )
pub fn modus_pcbc_hc_cipher(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Módulo
) -> (usize, Vec<i32>) {
    let mut cipher_text : Vec<i32> = Vec::new();     // Variable de retorno

    // Correción de errores CBC (1):
    // Si la llave para el cifrado hill no es el cuadrado del tamaño del bloque retorna 1.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return (0,cipher_text);
    }

    // Correción de errores CBC (2):
    // Si el bloque de cifrado inicial no corresponde con el tamaño correcto retorna 1.
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return (0,cipher_text);
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores CBC (2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    let mut tmp_block : Vec<i32> = c_0.to_vec(); // Se inicializa el valor del bloque temporal con el del bloque inicial

    // # Ciclo : Main Loop de Propagating Cipher Block Chaining 
    for i in msg_blocks {
        let block = i.to_vec();                                 // i es un apuntador entonce se convierte a un vector

        let _xor_result = block_xor(&block, &tmp_block);        // Se aplica una xor con el bloque anterior 
        let _cipher_block = hill_cipher(&_xor_result, &key, m); // Se aplica el cifrado al bloque
        tmp_block = block_xor(&block, &_cipher_block);          // Se aplica la funcion de propagación

        cipher_text.extend(_cipher_block);                      // Se guarda el bloque cifrado en el texto final
    }

    return (padding, cipher_text);
}

// -> Modo de operación Counter (CTR)
// C = Ek(Nonce * Counter) xor p
pub fn modus_ctr_hc_cipher(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> ( usize, Vec<i32>, Vec<i32> ) {
    let mut nonce : Vec<i32> = Vec::new();          // Variable de nonce
    let mut cipher_text : Vec<i32> = Vec::new();     // Variable de retorno

    // Correción de errores CTR (1):
    // Si la llave para el cifrado hill no es el cuadrado del tamaño del bloque retorna 1.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        nonce.push(1);
        return (0,nonce, cipher_text);
    }

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores CTR (2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    let _tmp_it = (block_size - 1) % block_size;

    nonce.extend(       // Se agregan numeros aleatorios y se le aplica un modulo, 
        repeat_with(
            || module(random::<i32>(),m)
        ).take(_tmp_it) // Repite la generación de estos hasta 1 menos el tamaño del bloque
    );
    nonce.push(0);

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);


    // # Ciclo : Main Loop de Counter
    for i in msg_blocks {
        let block = i.to_vec();                             // Como i es apuntador se pasa a Vector

        let _ctr_ek = hill_cipher(&nonce, &key, m);         // cifra el contador actual
        let _cipher_block = block_xor(&_ctr_ek, &block);    // Se aplica xor con plain text


        if let Some(counter) = nonce.last_mut() {           // Se le suma uno al contador
            *counter += 1;          
        }

        cipher_text.extend(_cipher_block);
    }

    nonce.pop();    // se elimina el contador, se mandan unicamente los numeros aleatorios

    return (padding, nonce, cipher_text);
}


// ##########################
// Sub main : Modus Decipher 
// ##########################

// -> Modo de operación Electronic CodeBook decifrado
// C = Dk(p)
pub fn modus_ecb_hc_decipher(
    cipher_text : &[i32],
    key : &[i32],
    padding : usize,
    m : i32
) -> Vec<i32> {
    let mut plain_text : Vec<i32> = Vec::new();     // Variable de retorno
    let block_size = (key.len() as f64).sqrt();     // Se saca el tañaño y se hace una raíz cuadrada

    // Corección de errores ECB_D(1):
    // Si la llave no es del tamaño correcto no se decodifica, retorna
    if block_size != block_size.trunc() {
        println!("Error: The key size muss be the square of a number");
        plain_text.push(1);
        return plain_text;
    }

    let block_size = block_size as usize;   // Se pasa el resultado a usize

    // Corrección de errores ECB_D(2):
    // Si el mensaje no se puede dividir perfectamente entre el tamaño del bloque, esto retorna
    if (cipher_text.len() % block_size) != 0 {
        println!("Error: The message size does not correspond with the block size");
        plain_text.push(1);
        return plain_text;
    }

    // Preparación del mensaje
    let _tmp_msg : Vec<i32> = cipher_text.to_vec().clone();                 // Se clona mensaje
    let msg_blocks = _tmp_msg.chunks(block_size);                           // Se divide en bloques del tamaño exacto

    // Generación de llave inversa para decifrado
    let inverse_key = matrix_inverse_module(&key, m as u32);                // Inversa de la matriz

    // Creación de diccionario
    let mut decipher_map : HashMap< Vec<i32>, Vec<i32> > = HashMap::new();  // Mapa hash para facilitar decifrado

    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        let decipher_block : Vec<i32>;                    // Se crea un cipher block temporal

        match decipher_map.get(&block) {                  // Se busca en el hash map si ya se ha calculado el valor y si:
            Some(p) => decipher_block = p.to_vec(),       // Si existe se pasa directo como cipher_block
            None => {                                   // Si no existe, se calcula y añade a la tabla hash
                decipher_block = hill_cipher(&block, &inverse_key, m);
                decipher_map.entry(block).or_insert(decipher_block.clone());
            }
        }
        plain_text.extend(decipher_block);   // Se agrega el bloque cifrado al final del texto cifa
    }


    let _msg_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    plain_text.drain(( _msg_size - padding ).._msg_size);    // Se le recortan los datos de holgura

    return plain_text;
}

pub fn modus_cbc_hc_decipher(
    cipher_text : &[i32],
    c_0 : &[i32],
    key : &[i32],
    padding : usize,
    m : i32
) -> Vec<i32> {

    let mut plain_text : Vec<i32> = Vec::new();     // Variable de retorno
    let block_size = (key.len() as f64).sqrt();     // Se saca el tañaño y se hace una raíz cuadrada

    // Corección de errores ECB_D(1):
    // Si la llave no es del tamaño correcto no se decodifica, retorna
    if block_size != block_size.trunc() {
        println!("Error: The key size muss be the square of a number");
        plain_text.push(1);
        return plain_text;
    }

    let block_size = block_size as usize;   // Se pasa el resultado a usize

    // Corrección de errores ECB_D(2):
    // Si el mensaje no se puede dividir perfectamente entre el tamaño del bloque, esto retorna
    if (cipher_text.len() % block_size) != 0 {
        println!("Error: The message size does not correspond with the block size");
        plain_text.push(1);
        return plain_text;
    }

    // Preparación del mensaje
    let _tmp_msg : Vec<i32> = cipher_text.to_vec().clone();                 // Se clona mensaje
    let msg_blocks = _tmp_msg.chunks(block_size);                           // Se divide en bloques del tamaño exacto

    // Generación de llave inversa para decifrado
    let inverse_key = matrix_inverse_module(&key, m as u32);                // Inversa de la matriz

    let mut tmp_block = c_0.to_vec();   // C inicial es un apuntador entonces se necesita clonar

    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        
        let dk_block = hill_cipher(&block, &inverse_key, m);    // Decifrado del bloque

        let decipher_block = block_xor(&dk_block, &tmp_block);     // Xor del decifrado y el anterior cifrado

        tmp_block = block;                  // Se cuarda el ultimo bloque
        plain_text.extend(decipher_block);   // Se agrega el bloque cifrado al final del texto cifa
    }


    let _msg_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    plain_text.drain(( _msg_size - padding ).._msg_size);    // Se le recortan los datos de holgura

    return plain_text;
}

pub fn modus_cfb_hc_decipher(
    cipher_text : &[i32],
    c_0 : &[i32],
    key : &[i32],
    padding : usize,
    m : i32
) -> Vec<i32> {

    let mut plain_text : Vec<i32> = Vec::new();     // Variable de retorno
    let block_size = (key.len() as f64).sqrt();     // Se saca el tañaño y se hace una raíz cuadrada

    // Corección de errores ECB_D(1):
    // Si la llave no es del tamaño correcto no se decodifica, retorna
    if block_size != block_size.trunc() {
        println!("Error: The key size muss be the square of a number");
        plain_text.push(1);
        return plain_text;
    }

    let block_size = block_size as usize;   // Se pasa el resultado a usize

    // Corrección de errores ECB_D(2):
    // Si el mensaje no se puede dividir perfectamente entre el tamaño del bloque, esto retorna
    if (cipher_text.len() % block_size) != 0 {
        println!("Error: The message size does not correspond with the block size");
        plain_text.push(1);
        return plain_text;
    }

    // Preparación del mensaje
    let _tmp_msg : Vec<i32> = cipher_text.to_vec().clone();                 // Se clona mensaje
    let msg_blocks = _tmp_msg.chunks(block_size);                           // Se divide en bloques del tamaño exacto


    let mut tmp_block = c_0.to_vec();   // C inicial es un apuntador entonces se necesita clonar

    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        
        let ek_block = hill_cipher(&tmp_block, &key, m);    // Decifrado del bloque

        let decipher_block = block_xor(&ek_block, &block);     // Xor del decifrado y el anterior cifrado

        tmp_block = block;                  // Se cuarda el ultimo bloque
        plain_text.extend(decipher_block);   // Se agrega el bloque cifrado al final del texto cifa
    }


    let _msg_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    plain_text.drain(( _msg_size - padding ).._msg_size);    // Se le recortan los datos de holgura

    return plain_text;
}

pub fn modus_ofb_hc_decipher(
    cipher_text : &[i32],
    c_0 : &[i32],
    key : &[i32],
    padding : usize,
    m : i32
) -> Vec<i32> {
    
    let mut plain_text : Vec<i32> = Vec::new();     // Variable de retorno
    let block_size = (key.len() as f64).sqrt();     // Se saca el tañaño y se hace una raíz cuadrada

    // Corección de errores ECB_D(1):
    // Si la llave no es del tamaño correcto no se decodifica, retorna
    if block_size != block_size.trunc() {
        println!("Error: The key size muss be the square of a number");
        plain_text.push(1);
        return plain_text;
    }

    let block_size = block_size as usize;   // Se pasa el resultado a usize

    // Corrección de errores ECB_D(2):
    // Si el mensaje no se puede dividir perfectamente entre el tamaño del bloque, esto retorna
    if (cipher_text.len() % block_size) != 0 {
        println!("Error: The message size does not correspond with the block size");
        plain_text.push(1);
        return plain_text;
    }

    // Preparación del mensaje
    let _tmp_msg : Vec<i32> = cipher_text.to_vec().clone();                 // Se clona mensaje
    let msg_blocks = _tmp_msg.chunks(block_size);                           // Se divide en bloques del tamaño exacto


    let mut tmp_block = c_0.to_vec();   // C inicial es un apuntador entonces se necesita clonar

    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        
        let ek_block = hill_cipher(&tmp_block, &key, m);    // Decifrado del bloque

        let decipher_block = block_xor(&ek_block, &block);     // Xor del decifrado y el anterior cifrado

        tmp_block = ek_block;                  // Se cuarda el ultimo bloque
        plain_text.extend(decipher_block);   // Se agrega el bloque cifrado al final del texto cifa
    }


    let _msg_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    plain_text.drain(( _msg_size - padding ).._msg_size);    // Se le recortan los datos de holgura

    return plain_text;
}

pub fn modus_pcbc_hc_decipher() {

}

pub fn modus_ctr_hc_decipher() {

}

/* --------------------------------------------------------------------------------
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
    --------------------------------------------------------------------------------
*/  

// -> Función Modulo, pero siempre positivo
#[inline(always)]   // Sugerencia al compilador para expansión inline
fn module( a: i32 , m : i32 ) -> i32 {
    return ( ( a % m ) + m ) % m ;  // Garantiza que el resultado esté en el rango [0, m-1]
}


// -> Función Block XOR : Realiza una operación XOR bit a bit entre dos vectores
fn block_xor( block_1 : &[i32], block_2 : &[i32]) -> Vec<i32> {
    let  result : Vec<i32> = block_1.to_vec().iter()
        .zip(block_2.to_vec().iter())
        .map(|(x , y)| x ^ y)
        .collect();
    return result;
}


// -> Función para imprimir una saldia formateada para error
fn error( message : &str ) {
    println!("{} {}", "Error:".red().bold(), message.red());
} 


// -> Función Debug Block : Imprime en consola un bloque con formato de depuración
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
