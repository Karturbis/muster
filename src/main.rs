use std::io;// for user input
use num_bigint::BigUint;// to use BigUint
use num_traits::FromPrimitive;// to convert Primitive datatypes to BigUint

/**
 * Die Hauptfunktion, sie wirdmit Start des Programmes ausgefuehrt.
 */
fn main() {
    //how to assign BIGUINT:  //let bignumber: BigUint = BigUint::from(variable:u128);
    
    let mut inputn = String::new();
    println!("Please type the row of the Pascla-triangle, to that you want to calculate the amounts of 7 dividable numbers..");
    io::stdin()
        .read_line(&mut inputn)
        .expect("Could not read input from stdin.");

    let n:u128 = inputn.trim().parse::<u128>().unwrap();

    
    


    calcseven(n);
    /*
     * outprints all lines of the Pascal triangle, up to the number given in the while statement.
     */
    if n < 42{
        let mut i:u128 = 0;
        print!("|");

        while i <= n{
            binomkoeff(i);
            i +=1;
            print!("\n|")
        }
    }        
}



fn calcseven(reihen_zahl:u128) {

    println!("LOG: reihen_zahl = {}", reihen_zahl);

    let x:u128 = reihen_zahl%7;
    println!("LOG: x = {}", x);

    let y:u128 = (reihen_zahl-x)/7;
    println!("LOG: y = {}", y);

    let mut result:u128 = 21*(y-1)/2*y;

    let mut j:u128 = 6;

    
        //result = 21*(y-1)+6;
    
    if x == 6 {
        result = 21*(y)
    }
    else {
        while j >= (6-x) {

            result += j;
            println!("LOG: es wird {} addiert", j*y);
            println!("LOG: result: {}", result);
            j-=1;
            
        }
    }

    


    println!("LOG: Final result: {}", result)
    }

/**
 * Berechnet die Fakultaet der uebergebenen Zahl.
*/
fn fakul(faknum:u128) -> BigUint{

        let mut result:BigUint = BigUint::from_u64(1).unwrap();

        let mut i:u128 = 1;
        /*
        Fakultäten:
        0! = 1          = 1
        1! = 1          = 1
        2! = 1*2        = 2
        3! = 1*2*3      = 6
        4! = 1*2*3*4    = 24
        5! = 1*2*3*4*5  = 120
        daher x*(x-1)! = x!
        */ 
        while i <= faknum{
            
            let j:BigUint = BigUint::from_u128(i).unwrap();
            result = result * j;

            i += 1;
        }
        return result;
}

/**
 * berechnet den Binomialkoeffizienten der uebergebenen Zahlen 'n' und 'k' und gibt das Ergebnis ueber die Konsole aus.
*/
fn binomkoeff(n:u128,) {
   
    let mut k:u128 = 0;
    let mut result:BigUint = BigUint::from_u8(1).unwrap();
    //n ueber k = n!/(k!*(n-k)!)

    while k <= n {
        
        result = fakul(n)/(fakul(k)*fakul(n-k));
        print!("{}|", result);
        k += 1;
    }
    
}
