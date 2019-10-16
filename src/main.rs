extern crate num;
use num::complex::Complex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "newton")]
struct Opt {
    #[structopt(short, long)]
    polynomial: String,

    #[structopt(short, long, default_value = "1.0")]
    real: f64,

    #[structopt(short, long, default_value = "1.0")]
    imaginary: f64,

    #[structopt(short, long, default_value = "20")]
    iterations: usize,
}

fn main() {
    let opt = Opt::from_args();

    let guess = Complex::new(opt.real, opt.imaginary);
    let iterations = opt.iterations;
    let polynomial = parse_poly(&opt.polynomial);

    let roots = newtons(polynomial, guess, iterations);

    for root in roots {
        println!("{:.5}", root);
    }
}

fn parse_poly(polynomial: &String) -> Vec<Complex<f64>> {
    polynomial.split(", ").map(|x| {
        let c: f64 = x.parse().unwrap_or_else(|_| panic!("newtons: error: failure to parse polynomial. expected format of c0, c1, ... , cn where c is a coeficient"));
        Complex::new(c, 0.0)
    })
    .collect()
}

fn newtons(
    polynomial: Vec<Complex<f64>>,
    guess: Complex<f64>,
    iterations: usize,
) -> Vec<Complex<f64>> {
    let error = 0.000000001;

    let mut roots: Vec<Complex<f64>> = vec![];

    find_root(polynomial, guess, iterations, error, &mut roots);

    roots
}

fn find_root(
    polynomial: Vec<Complex<f64>>,
    guess: Complex<f64>,
    iterations: usize,
    error: f64,
    roots: &mut Vec<Complex<f64>>,
) {
    let degree = polynomial.len() - 1;
    let mut root = guess;
    let mut quotient: Vec<Complex<f64>> = Vec::with_capacity(degree + 1);

    quotient.push(polynomial[0]);

    for _ in 0..degree {
        for _ in 0..iterations {
            for (i, c) in polynomial.iter().skip(1).enumerate() {
                quotient.push(quotient[i] * root + c);
            }

            let rem = quotient.last().unwrap();
            if rem.re + rem.im < error {
                roots.push(root);
                quotient.pop();
                break;
            }

            let fx = quotient.last().unwrap();
            let slope = quotient
                .iter()
                .skip(1)
                .fold(quotient[0], |s, c| root * s + c);

            root = (fx / slope) - root;
        }
        quotient.clear();
    }
}
