use colored::Colorize;

/*
    Test Area : Funcion Main
*/

fn main() {
    let matrix_0 = [5 , 17, 20, 9, 23, 3, 2, 11, 13];
    println!("{}", "Matriz original 0 :".blue());
    print_matrix(&matrix_0);
    println!("{}", "Matriz 0  transpuesta:".blue());
    print_matrix(&matrix_transposed(&matrix_0));
    let determinant = matrix_determinant(&matrix_0);
    println!("{} {}","Determinante 1:".blue(), determinant);


    println!();
    let matrix_1 = [0, 1, 2, 3];
    println!("{}", "Matriz original 1 :".blue());
    print_matrix(&matrix_1);
    println!("{}", "Matriz 1 transpuesta:".blue());
    print_matrix(&matrix_transposed(&matrix_1));
    let determinant = matrix_determinant(&matrix_1);
    println!("{} {}", "Determinante 2:".blue(), determinant);

}

/*
    Main code : Funciones principales
*/


// -> Funcion para determinar la determinante de una matriz, solo si esta es 3x3 o 2x2
fn matrix_determinant( matrix : &[i32] ) -> i32 {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    // corrección de errores :
    // En caso de que su raiz cuadrada tenga decimales (no es un numero cuadrado) regresa un error
    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return -1;
    }

    // cambia el float
    let size = size as u32;

    // Corrección de errores : 
    // si esta es de mayor tamaño a 3 la función termina
    if size > 3 {
        println!("{} {}","Error:".red().bold(), "The function only acceps square matrices of 1 to 2".red());
        return -1;
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


/*
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
*/

// -> Función para devolver un valor a partir de solo sus cordenadas, ademas si las cordenadas se pasan o son negativas se les aplica un modulo
fn find_in_matrix( matrix: &[i32] , size : u32 , row : i32 , column : i32 ) -> i32 {
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