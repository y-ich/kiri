#![feature(test)]

extern crate test;
extern crate rand;
extern crate arrayvec;
extern crate go_board;

pub mod rule;
pub mod position;


#[cfg(test)]
mod tests {
    use go_board::*;
    use position::*;
    use rand::{thread_rng, Rng};
    use rule::Rule;

    fn rollout() -> (u32, f32) {
        let mut rng = thread_rng();
        let mut game = Position19::new();
        let mut num_consecutive_passes = 0;
        let mut num_moves = 0;

        while num_consecutive_passes < 2 {
            let candidates = game.empties();
            let mut played = false;
            if candidates.len() > 0 {
                let start_index = rng.gen_range(0, candidates.len());
                let mut i = start_index;
                loop {
                    let pt = candidates[i];
                    if game.is_eye(pt) != game.get_turn().to_pointstate() {
                        if let Ok(_) = game.play(Move::Linear(pt)) {
                            played = true;
                            break;
                        }
                    }
                    i += 1;
                    if i >= candidates.len() {
                        i = 0;
                    }
                    if i == start_index {
                        break;
                    }
                }
            }
            if played {
                num_consecutive_passes = 0;
            } else {
                let _ = game.play(Move::Pass);
                num_consecutive_passes += 1;
            }
            num_moves += 1;
            if num_moves > 1000 {
                println!("suspicious game with > 1000 moves");
                break;
            }
        }
        return (num_moves, game.score());
    }

    #[test]
    fn is_all_empties_on_board() {
        let pos = Position19::new();
        let empties = pos.empties();
        assert!(empties.into_iter().all(|pt| pos.is_on_board(pt)));
    }

    #[test]
    fn test_is_eye() {
        let mut pos = Position19::new();
        let point = pos.xy_to_linear(1, 2);
        pos.set_state(point, PointState::Black);
        let point = pos.xy_to_linear(2, 1);
        pos.set_state(point, PointState::Black);
        println!("{}", pos);
        let point = pos.xy_to_linear(1, 1);
        println!("{}", pos.is_eye(point));
        assert!(pos.is_eye(point) == PointState::Black);
    }

    #[test]
    fn test_rollout() {
        assert!(rollout().0 < 1000);
    }

    use test::Bencher;
    #[bench]
    fn bench_rollout(b: &mut Bencher) {
        b.iter(rollout);
    }
}
