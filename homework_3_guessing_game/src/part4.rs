use crate::StrategyEnum;
use crate::part3::SimulatedPlayer;
use crate::player::Player;

pub fn experiment(strategy: StrategyEnum, min: u32, max: u32) -> u32 {
    // This is not a good experiment: we always choose min as the secret number.
    // In other words, our secret number is always zero.
    // You need to change this code so it is more representative of how many steps
    // the strategy would take in the worst case.
    //
    // HINT: understand this code before changing it.
    // HINT: instead of fixing number = min, come up with different values of number from [min, max).
    // HINT: consider the endpoints (i.e., min, max-1) as well as points in between (random, midpoint, etc).
    // HINT: if the different choices of numbers result in different number of steps, what should you return from this function?
    //       remember, we want *the worst case*.
    //
    let number = min;
    let player = Player::new(SimulatedPlayer::new(number));
    let steps = strategy.guess_the_number(player, min, max);
    return steps;
}
