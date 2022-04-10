use crate::Model;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

fn play(model: &Model, rng: &mut XorShiftRng) -> u32 {
    const MAX_ROUNDS: u32 = 10000;
    let mut score: u32 = 0;
    let mut rounds: u32 = 0;
    let mut left: i32 = 100;
    while left > 0 && rounds < MAX_ROUNDS {
        // 50% chance of cheater
        let actually_fair: bool = rng.gen();
        let gen_mask: u8 = if actually_fair {
            0b01
        } else {
            // 75% when not fair
            0b11
        };
        let mut heads: u32 = 0;
        let mut tails: u32 = 0;
        left -= 1;
        if {
            let gen: u8 = rng.gen();
            gen & gen_mask == 0
        } {
            tails += 1;
        } else {
            heads += 1;
        }
        if left == 0 {
            if (3 * heads <= 5 * tails) == actually_fair {
                score += 1;
                left += 15;
            } else {
                left -= 30
            }
        }
        while left > 0 && rounds < MAX_ROUNDS {
            rounds += 1;
            // if heads < 16 && tails < 16 && model.0[tails as usize] as u32 > heads {
            // FFFT
            // FFFF
            // TTFF
            // FFTT
            // then FTTT
            // if heads < 4 && tails < 4 && model.0 & (1 << (heads | (tails << 2))) != 0 {
            // FFFTF
            // FFFFT
            // TTFFT
            // FFTTT
            //  01234
            // 0+++C
            // 1++++C
            // 2FF++C
            // 3  FF
            // Stop when:
            // 4 heads
            // 3 tails
            // 3 heads & 0 tails
            // 2 tails & 0-1 heads
            //
            if tails < 4 && heads < 5 && if heads < 4 {
                tails < 3 && if tails < 2 {
                    tails != 0 || heads < 3
                } else {
                    tails > 1
                }
            } else { model.0 & (1 << tails) != 0 } {
                if {
                    let gen: u8 = rng.gen();
                    gen & gen_mask == 0
                } {
                    tails += 1;
                } else {
                    heads += 1;
                }
                left -= 1;
                if left != 0 {
                    continue;
                }
            }
            // h/(h+t)<5/8
            // 8h<5(h+t)
            // 8h<5h+5t
            // 3h<5t
            if (3 * heads <= 5 * tails) == actually_fair {
                score += 1;
                left += 15;
            } else {
                left -= 30
            }
            break;
        }
    }
    return score
}

pub(crate) fn eval_model(model: &Model) -> u32 {
    // if model.0[0] == 0 {
    if model.0 == 0 {
        return 0;
    }
    let mut rng = XorShiftRng::from_seed([0; 16]);
    let mut result: u32 = 0;
    for _ in 0..200 {
        result += play(model, &mut rng);
    }
    return result;
}
