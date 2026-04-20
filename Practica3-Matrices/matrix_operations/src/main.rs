use colored::Colorize;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/  

fn main() {
    let matrix_0 = [5 , 17, 20, 9, 23, 3, 2, 11, 13];
    println!("{}", "Matriz original 0 :".blue());
    print_matrix(&matrix_0);
    println!("{}", "Matriz 0  transpuesta:".blue());
    print_matrix(&matrix_transposed(&matrix_0));
    let determinant = matrix_determinant(&matrix_0);
    println!("{} {}","Determinante 1:".blue(), determinant);
    println!("{}", "Matriz 0  cofactor 0,0:".blue());
    print_matrix(&matrix_cofactor(&matrix_0, 0,0));
    println!("{} {}","Determinante 1:".blue(), matrix_determinant(&matrix_cofactor(&matrix_0, 0,0)));
    print_matrix(&matrix_inverse_module(&matrix_0, 27));
    print_matrix_f(&matrix_inverse(&matrix_0));
    matrix_addition_module(&matrix_0,&matrix_0,5);
    print_matrix(&matrix_multiplication_matrix_module(&matrix_0, &matrix_inverse_module(&matrix_0, 27), 27));
    /*
    println!();
    let matrix_1 = [0, 1, 2, 3];
    println!("{}", "Matriz original 1 :".blue());
    print_matrix(&matrix_1);
    println!("{}", "Matriz 1 transpuesta:".blue());
    print_matrix(&matrix_transposed(&matrix_1));
    let determinant = matrix_determinant(&matrix_1);
    println!("{} {}", "Determinante 2:".blue(), determinant);

    println!();
    let matrix_2 = [1 , 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{}", "Matriz original 2 :".blue());
    print_matrix(&matrix_2);
    println!("{}", "Matriz 2 + Matriz 1 :".blue());
    print_matrix(&matrix_addition(&matrix_2,&matrix_0));
    println!("{}", "Matriz 2 * 5 :".blue());
    print_matrix(&matrix_multiplication_escalar(&matrix_2,5));
    println!("{}", "Matriz 2 * 5 mod 20 :".blue());
    print_matrix(&matrix_module(&matrix_multiplication_escalar(&matrix_2, 5), 20));
    */
}

/*  
    --------------------------------------------------------------------------------
    Main code : Algebra lineal pura
    --------------------------------------------------------------------------------
*/  

// -> Función para calcular la matriz inversa
fn matrix_inverse ( matrix : &[i32] ) -> Vec<f64> {
    let _tmp_determinant = matrix_determinant(&matrix);
    if _tmp_determinant == 0 {
        error("The inverse matrix does not exist. The determinant is equal to 0.");
        let result : Vec<f64> = Vec::new();
        return result;
    }
    let _tmp_matrix : Vec<i32> = Vec::from(matrix_adjugate(&matrix));
    let _tmp_determinant : f64 = 1.0 / (matrix_determinant(&matrix)) as f64;
    let result : Vec<f64> = Vec::from(matrix_multiplication_escalar_f(&_tmp_matrix, _tmp_determinant));
    return result;
}




// -> Funcion para determinar la determinante de una matriz, solo si esta es 3x3 o 2x2
fn matrix_determinant( matrix : &[i32] ) -> i32 {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    // corrección de errores :
    // En caso de que su raiz cuadrada tenga decimales (no es un numero cuadrado) regresa un error
    if size != size.trunc() {
        error("This function only accept square matrices");
        return -1;
    }

    // cambia el float
    let size = size as u32;

    // Corrección de errores : 
    // si esta es de mayor tamaño a 3 la función termina
    if size > 3 {
        error("The function only acceps square matrices of 1 to 2");
        return -1;
    }

    if size == 1 {
        return matrix[0];
    }

    // Determinante si esta es una matriz 2x2
    if size == 2 {
        let a = matrix[0];
        let b = matrix[1];
        let c = matrix[2];
        let d = matrix[3];
        return (a * d) - (b * c);
    }


    // Determinante si esta es una matriz 3x3

    let _size = size as i32;
    let mut determinant : i32 = 0;  // inicializa el valor del determinante
    let mut tmp_det : i32;          // Guarda el valor de la multipliocación en cada diagonal

    let x : i32 = 0;

    // Diagonales positivas
    for i in 0.._size {
        tmp_det = 1;
        for j in 0.._size {
            tmp_det *= find_in_matrix( matrix, size, i + j, x + j);
        }
        determinant += tmp_det;
    }

    // Diagonales negativas
    for i in 0.._size {
        tmp_det = 1;
        for j in 0.._size {
            tmp_det *= find_in_matrix( matrix, size, i + j, x - j);
        }
        determinant -= tmp_det;
    }

    return determinant;
}


// -> Función para calcular la matriz transpuesta de una matriz, solo si es 3x3 y 2x2
fn matrix_transposed( matrix : &[i32] ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();
    let size = matrix.len();
    if !matrix_validation(size) { 
        tmp_matrix.push(-1);
        return tmp_matrix;
    }
    let size = (size as f64).sqrt() as i32;
    let size = size as u32;
    let _size = size as i32;
    for i in 0.._size {
        for j in 0.._size {
            tmp_matrix.push(find_in_matrix( matrix, size, j, i));
        }
    }
    
    return tmp_matrix;
}

// -> Función para calcular la matriz adjunta de una matriz, solo si es 3x3 o 2x2 
fn matrix_adjugate(matrix : &[i32] ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();
    let size = matrix.len();
    if !matrix_validation(size) { 
        error("The size of the matrix is not the size of a square matrix");
        tmp_matrix.push(-1);
        return tmp_matrix;
    }
    let size = (size as f64).sqrt() as i32;

    debug("2x2", "llegada a matriz 2 x 2");

    let base : i32 = -1;
    for i in 0..size {
        for j in 0..size {
            let _tmp_cofactor = matrix_cofactor(matrix,i,j);
            let _tmp_determinant = matrix_determinant(&_tmp_cofactor);
            let _result = base.pow(((i + 1) + (j + 1)) as u32) * _tmp_determinant;
            tmp_matrix.push(_result);
        }
    }
    return tmp_matrix;
}

fn matrix_cofactor(matrix : &[i32], i_coef : i32, j_coef : i32) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();
    let size = matrix.len();
    if !matrix_validation(size) { 
        error("The size of the matrix is not the size of a square matrix");
        tmp_matrix.push(-1);
        return tmp_matrix;
    }
    let size = (size as f64).sqrt() as i32;

    if (i_coef >= size) || (j_coef >= size) {
        error("The index of cofactor is out of reach.");
    }

    let _size = size as u32;

    for i in 0..size {
        for j in 0..size {
            if i == i_coef {continue};
            if j == j_coef {continue};
            let _new_coef = find_in_matrix(&matrix, _size, j, i);
            tmp_matrix.push(_new_coef);
        }
    }

    return tmp_matrix;
}

// -> Funcion para sumar una matriz con otra matriz
fn matrix_addition( matrix_a : &[i32] , matrix_b : &[i32] ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();

    let _size_a = matrix_a.len();
    let _size_b = matrix_b.len();

    // corrección de errores :
    // Checa si el tamaño de las matrices es identico
    if !( _size_a == _size_b ) {
        error("The dimensions of the matrices muss be the same.");
        tmp_matrix.push(-1);
        return tmp_matrix;
    }

    for i in 0.._size_a {
        tmp_matrix.push( matrix_a[i] + matrix_b[i] );
    }

    return tmp_matrix;
}


// -> Funcion para multiplicar una matriz con otra mnatriz solo si es 3x3 o 2x2
fn matrix_multiplication_matrix( matrix_a : &[i32] , matrix_b : &[i32] ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();

    let _size_a = matrix_a.len();
    let _size_b = matrix_b.len();

    // Reutilizamos tu lógica de validación (basada en matrix_addition)
    if !( _size_a == _size_b ) {
        println!("Error: The dimensions of the matrices must be the same.");
        tmp_matrix.push(-1);
        return tmp_matrix;
    }

    // Validamos que sea 2x2 o 3x3 como pide tu comentario
    if !matrix_validation(_size_a) {
        tmp_matrix.push(-1);
        return tmp_matrix;
    }

    // Calculamos n (la raíz cuadrada) para saber si es 2 o 3
    let n = (_size_a as f64).sqrt() as u32;

    // Algoritmo de multiplicación
    for row in 0..n {
        for col in 0..n {
            let mut sum = 0;
            for k in 0..n {
                // Usamos tu función find_in_matrix para obtener los valores
                let val_a = find_in_matrix(matrix_a, n, row as i32, k as i32);
                let val_b = find_in_matrix(matrix_b, n, k as i32, col as i32);
                sum += val_a * val_b;
            }
            tmp_matrix.push(sum);
        }
    }
    
    return tmp_matrix;
}

// -> Funcion para multiplicar una matriz con un numero escalar
fn matrix_multiplication_escalar( matrix : &[i32] , escalar : i32 ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();

    let _size = matrix.len();
    for i in 0.._size {
        tmp_matrix.push( matrix[i] * escalar );
    }
    return tmp_matrix;
}

fn matrix_multiplication_escalar_f( matrix : &[i32] , escalar : f64 ) -> Vec<f64> {
    let mut tmp_matrix : Vec<f64> = Vec::new();

    let _size = matrix.len();
    for i in 0.._size {
        tmp_matrix.push( (matrix[i] as f64) * escalar );
    }
    return tmp_matrix;
}

/*  
    --------------------------------------------------------------------------------
    Main Code : Algebra lineal modular
    --------------------------------------------------------------------------------
*/

// -> Funcion para sacar el modulo de una matriz con un modulo m
fn matrix_module( matrix : &[i32] , m : u32 ) -> Vec<i32> {
    let mut tmp_matrix : Vec<i32> = Vec::new();

    let _size = matrix.len();

    for i in 0.._size {
        tmp_matrix.push( module(matrix[i] ,  m) as i32);
    }

    return tmp_matrix;
}

fn matrix_inverse_module(matrix : &[i32] , m : u32) -> Vec<i32> {
    let mut _tmp_determinant = matrix_determinant(&matrix);
    let mut result : Vec<i32> = Vec::new();
    if _tmp_determinant == 0 {
        error("The inverse matrix does not exist. The determinant is equal to 0.");
        return result;
    }
    _tmp_determinant = module(_tmp_determinant, m) as i32;
    _tmp_determinant = euclid_extended(m as i32, _tmp_determinant);
    result = matrix_adjugate(&matrix);
    result = matrix_multiplication_escalar_module(&result, _tmp_determinant, m);
    return result;
}

fn matrix_addition_module( matrix_a : &[i32] , matrix_b : &[i32] , m : u32 ) -> Vec<i32> {
    let tmp_matrix = matrix_addition(&matrix_a, &matrix_b);
    let tmp_matrix = matrix_module(&tmp_matrix, m);
    return tmp_matrix;
}

fn matrix_multiplication_matrix_module( matrix_a : &[i32], matrix_b : &[i32], m : u32 ) -> Vec<i32> {
    let tmp_matrix = matrix_multiplication_matrix(&matrix_a, &matrix_b);
    let tmp_matrix = matrix_module(&tmp_matrix, m);
    return tmp_matrix;
}

fn matrix_multiplication_escalar_module( matrix : &[i32] , escalar : i32 , m : u32 ) -> Vec<i32> {
    let tmp_matrix = matrix_multiplication_escalar(&matrix, escalar);
    let tmp_matrix = matrix_module(&tmp_matrix, m);
    return tmp_matrix;
}

fn euclid_extended( n_length : i32, alpha : i32 ) -> i32 {
    let a = if n_length > alpha { n_length } else { alpha };
    let b = if n_length > alpha { alpha } else { n_length };

    let (d, x, y) = xgcd_rec(a, b);

    let out : i32;
    if ((x*alpha) + (y*n_length)) == d {
        out = x;
    }
    else {
        out = y;
    }

    if out > 0 {
        return out;
    }
    else {
        return module(out, n_length as u32) as i32;
    }
}

fn xgcd_rec( a : i32 , b : i32 ) -> ( i32, i32, i32 ) {
    if b == 0 {
        return ( a, 1, 0 );
    }
    else {
        let (d, x1, y1) = xgcd_rec(b, a%b);

        let q = a / b;

        let x = y1;
        let y = x1 - y1 * q;

        assert!( a*x + b*y == d, "La ecuación de Bezout no se cumple");
        return (d, x, y);
    }
}


/*  
    --------------------------------------------------------------------------------
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
    --------------------------------------------------------------------------------
*/  

// -> Función para devolver un valor a partir de solo sus cordenadas, ademas si las cordenadas se pasan o son negativas se les aplica un modulo
fn find_in_matrix( matrix: &[i32] , size : u32 , row : i32 , column : i32 ) -> i32 {
    let tmp_row : u32 = module(row, size as u32);
    let tmp_col : u32 = module(column, size as u32);
    return matrix[((size * tmp_row) + tmp_col) as usize];
} 

fn find_in_matrix_f( matrix: &[f64] , size : u32 , row : i32 , column : i32 ) -> f64 {
    let tmp_row : u32 = module(row, size as u32);
    let tmp_col : u32 = module(column, size as u32);
    return matrix[((size * tmp_row) + tmp_col) as usize];
} 

// -> Función para que el modulo siempre salga positivo, esta es necesaria porque rust no devuelve el modulo siempre positivo.
fn module( a : i32, n : u32) -> u32 {
    let _n = n as i32; 
    return (((a % _n) + _n) % _n ) as u32;
}

// -> Función para imprimir en terminal las matrices para que sean mas faciles de visualizar.
fn print_matrix(matrix: &[i32]) {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return;
    }
    
    let size = size as i32;

    for i in 0..size {
        print!("{}","|".yellow().bold());
        for j in 0..size {
            print!("\t{}\t", find_in_matrix(matrix, size as u32, i, j));
        }
        println!("{}","|".yellow().bold());
    }
}

fn print_matrix_f(matrix: &[f64]) {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return;
    }
    
    let size = size as i32;

    for i in 0..size {
        print!("{}","|".yellow().bold());
        for j in 0..size {
            print!("\t{:.4}\t", find_in_matrix_f(matrix, size as u32, i, j));
        }
        println!("{}","|".yellow().bold());
    }
}

// -> Función para verificar el tamaño de una matriz, que sea 3x3 o 2x2
fn matrix_validation(size : usize) -> bool {
    let size = size as f64;
    let size = size.sqrt();

    // corrección de errores :
    // En caso de que su raiz cuadrada tenga decimales (no es un numero cuadrado) regresa un error
    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return false;
    }

    // cambia el float
    let size = size as u32;

    // Corrección de errores : 
    // si esta es de mayor tamaño a 3 la función termina
    if size > 3 {
        println!("{} {}","Error:".red().bold(), "The function only acceps square matrices of 1 to 2".red());
        return false;
    }

    return true;
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