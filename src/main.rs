use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::Ordering;

struct Interviewer {
    number: i32,
    balance: i64,
    rng: ThreadRng,
}

impl Interviewer {
    fn new_game(&mut self) {
        self.number = self.rng.gen_range(1..=100);
        self.balance = 6;
    }

    fn make_guess(&mut self, guess: i32) -> Ordering {
        self.balance -= 1;
        self.number.cmp(&guess)
    }
}

fn play(ballmer: &mut Interviewer) -> i64 {
    ballmer.new_game();

    let mut lower: i32 = 1;
    let mut guess: i32 = 50;
    let mut upper: i32 = 101;

    loop {
        match ballmer.make_guess(guess) {
            Ordering::Less => {
                upper = guess;
            }
            Ordering::Greater => {
                lower = guess;
            }
            Ordering::Equal => {
                break;
            }
        }
        guess = (upper + lower) / 2;
    }
    ballmer.balance
}

fn monte_carlo(iterations: i32) -> f64 {
    let mut ballmer = Interviewer {
        number: 0,
        balance: 0,
        rng: rand::thread_rng(),
    };

    let mut balance: i64 = 0;

    for _ in 0..iterations {
        balance += play(&mut ballmer);
    }

    balance as f64 / iterations as f64
}

fn main() {
    println!("Expected value: ${}", monte_carlo(10_000_000));
}
