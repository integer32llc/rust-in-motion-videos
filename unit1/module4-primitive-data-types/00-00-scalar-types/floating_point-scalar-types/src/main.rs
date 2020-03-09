
/*
floats are not integer numbers(numbers with a dot or fractions)
In RUSt we have Example f64 and f32

As a lot of other programming languages, Floating-point numbers are represented in computer
hardware as base 2 (binary) fractions.


f64 offers more precision than f32
f64 offers by default 64 bits of precision
Usually the number represented is just an aproximation of the binary
we do not "see"  the exact value we have store in the float
IN python we use Decimal module to see this

twitter:  https://lnkd.in/e2ffbrp
web:     https://www.pymiami.org
linkedIn: https://lnkd.in/e6hvX8Y


*/

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    //we will get 0 here as x is by default i32
    let x = 1/3;
    println!("print x=1/3 is represented {}", x);
    let y = 1/2;
    println!("y = 1/2 is represented as {}", y);

    // Now both as float so we will get 0.3333333333333333
    let a = 1.0/3.0;
    println!("print x=1/3 is represented {}", a);


    // integer/float => ERROR

    // let b = 1/3.0;
    // println!("print x=1/3.0 is represented {}", b);
    // ERROR : no implementation for `{integer} / {float} `{integer} / {float}

    // float/integer => ERROR

    //let c = 1.0/3;
    //println!("print x=1.0/3 is represented {}", c);
    // ERROR : no implementation for `{integer} / {float} `{integer} / {float}

    /*
       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       -"The 0.1 representation problem "
       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       (See in Python https://docs.python.org/3.8/tutorial/floatingpoint.html)

       The "0.1" representation error in PYTHON also happens in RUST for the same reason:
       "Floating-point numbers are represented in computer hardware as base 2 (binary) fractions"

       So 0.1 = 1/10 decimal = 1/1010 binary
       But 1/1010 binary is not an exact number in base 2.
       When we execute the 'binary long division' operation 1/1010 we get and infinite sequence
       0.000110011001100110011001100110011001100110011001100110011001…

       When we take this number back to decimal we get not 0.1 but
       0.1000000000000000055511151231257827021181583404541015625

       That is why in RUST like in In Python 0.1 + 0.2 will show 0.30000000000000004
       The point is that 0.1 is just 1/10 and Python have it with a binary approximation to decimal
       0.1000000000000000055511151231257827021181583404541015625. The same occurs for 0.2

       And actually it happens with almost all floats numbers !!

        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       "53 bits long representation problem"
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        Another issues is that we can represent floats with up to 53 bits only in binary(Python)
        so any decimal we need to use with more than 53 bits, will be truncated in its
        binary representation

        Example Decimal: 0.500000000000000166533453693773481063544750213623046875

        So it will be rounded to


    */

    //
    let d = 0.1 + 0.2;
    println!("print 0.1 + 0.2  is represented {}", d);


    let f = 0.1;
    let g = 0.2;
    let h = f + g;
    println!("The sum of {} + {} = {}", f,g,h)

}
