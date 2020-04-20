use rand::prelude::*;

pub fn monte_carlo_pi(iterations: usize) -> f32 {
    let mut inside_circle = 0;
    for _ in 0..iterations {

        let x: f32 = random::<f32>();
        let y: f32 = random::<f32>();

        if x.powi(2) + y.powi(2) <= 1_f32 {
            inside_circle +=1;
        }
    }
    (4_f32 * inside_circle as f32) / iterations as f32

}


#[cfg(test)]
mod tests {

    use super::*;

    fn is_reasonably_pi(pi : f32) -> bool {
        pi >= 3_f32 && pi <= 4.5_f32
    }
    #[test]
    fn test_monte_carlo_pi_1() {
        let pi = monte_carlo_pi(1);
        assert!(pi == 0_f32 || pi == 4_f32);
    }

    #[test]
    fn test_monte_carlo_pi_500() {
        let pi = monte_carlo_pi(500);
        assert!(is_reasonably_pi(pi));
    }

    #[test]
    fn test_monte_carlo_pi_1000() {
        let pi = monte_carlo_pi(1000);
        assert!(is_reasonably_pi(pi));
    }
}
