use rand::Rng;

extern crate rand;

fn main() {
    let vezes = 10;

    for _ in 0..vezes {
        let numero: f64 = rand::thread_rng().gen_range(1.0, 200.0);
        println!("O fahenrait foi: {} e celcius {}", numero, f_p_c(numero));
    }
}

fn f_p_c(temp: f64) -> f64 {
    let celsius = (temp - 32.0) / 1.8;
    celsius
}
