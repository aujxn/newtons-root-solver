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

    #[structopt(long, default_value = "20")]
    iter: usize,
}

fn main() {
    let opt = Opt::from_args();

    let guess = Complex::new(opt.real, opt.imaginary);
    let iterations = opt.iter;
    let polynomial = parse_poly(&opt.polynomial);

    println!("{:?}", polynomial);

    let roots = newtons(polynomial, guess, iterations);

    for root in roots {
        println!("{:.5}", root);
    }
}

fn parse_poly(polynomial: &String) -> Vec<Complex<f64>> {
    polynomial.split(",").map(|x| {
        let c: f64 = x.parse().unwrap_or_else(|_| panic!("newtons: error: failure to parse polynomial. expected format of c0,c1,...,cn where c is a coeficient"));
        Complex::new(c, 0.0)
    })
    .collect()
}

fn newtons(
    polynomial: Vec<Complex<f64>>,
    guess: Complex<f64>,
    iterations: usize,
) -> Vec<Complex<f64>> {
    let error = 0.0000000001;

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
    let mut deflated = vec![polynomial];

    for pass in 0..degree {
    match (0..iterations).into_iter().find(|_| {
        quotient.push(deflated[pass][0]);
            for (i, c) in deflated[pass].iter().skip(1).enumerate() {
                quotient.push(quotient[i] * root + c);
            }
            println!("{:?}", quotient);

            let rem = quotient.last().unwrap();
            println!("{:.8} rem: {:.5}", root, rem);
            if num::abs(rem.re) + num::abs(rem.im) < error {
                println!("found! {:?}", root);
                roots.push(root);
                quotient.pop();
                deflated.push(quotient.clone());
                quotient.clear();
                true
            } else {

            let fx = quotient.pop().unwrap();
            let slope = quotient
                .iter()
                .skip(1)
                .fold(quotient[0], |s, c| (root * s) + c);

            root = root - (fx / slope);
            quotient.clear();
            false
            }
        }) {
            Some(_) => (),
            None => panic!("Failed to evaluate. Roots found: {:?}", roots),
        }
    }

    }
