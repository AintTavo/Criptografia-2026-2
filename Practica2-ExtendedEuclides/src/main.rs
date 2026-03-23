fn main() {
    let n_length = 239;
    let alpha = 97;
    let beta = 240;
    if euclid_valid(n_length, alpha) {
        println!("Es numero coprimo");
        
        let neg_alpha = euclid_extended(n_length,alpha);
        if (beta%n_length) != 0 {
            let neg_beta = beta%n_length;
            let neg_beta = n_length - neg_beta;
            println!("(EK)  C = {} p + {} mod {} ", alpha, beta%n_length , n_length);
            println!("(Dk)  p = {} [ C + {} ] mod {}", neg_alpha, neg_beta, n_length);
            println!("(Dk)  p = {} C + ({})({}) mod {}", neg_alpha, neg_alpha, neg_beta, n_length);
        }
        else {
            println!("(EK)  C = {} p mod {} ", alpha, n_length);
            println!("(Dk)  p = {} [ C ] mod {}", neg_alpha, n_length);
        }
        

    }
    else{
        println!("No es numero coprimo");
    }
}

fn euclid( a : i32, b: i32 ) -> i32 {
    if b == 0 {
        return a;
    }
    else {
        return euclid(b, a%b);
    }
}

fn euclid_valid( n_length : i32 , alpha : i32 ) -> bool {
    let a = if n_length > alpha { n_length } else { alpha };
    let b = if n_length > alpha { alpha } else { n_length };

    let mcd : i32 = euclid(a, b);
    if mcd == 1 {
        return true;
    }
    return false;
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
        return out%n_length;
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