fn main() {
    let f = vec![1, 0, -30, -36];

    newtons(f);
}

fn f(x: f64, coef: &Vec<i64>) -> f64 {
    coef.iter()
        .rev()
        .enumerate()
        .fold(0.0, |acc, (i, coef)| acc + x.powf(i as f64) * *coef as f64)
}

fn newtons(coef: Vec<i64>) {
    let degree = coef.len() - 1;

    let derivative: Vec<i64> = coef
        .iter()
        .take(degree)
        .enumerate()
        .map(|(i, x)| x * (degree as i64 - i as i64))
        .collect();

    println!("f(x): {:?} \n f'(x): {:?}", coef, derivative);

    let mut guess = 4.0;

    for _ in 0..20 {
        guess = guess - (f(guess, &coef) / f(guess, &derivative));
        println!("{:?}", guess);
    }
}
