// #![feature(slice_group_by)]
mod group_by;
pub use group_by::{GroupByTrait};

const GUESS_NUMBERS_COUNT: usize = 6;

type GuessNumbersType = [u8; GUESS_NUMBERS_COUNT];
type AccountId = u32;
type Winners<AccountId> = Vec<(Bet<AccountId>, u8)>;

#[derive(Debug)]
struct Bet<AccountId> {
  account_id: AccountId,
  guess_numbers: GuessNumbersType,
}

fn main() {
    // mock pot
    let pot= 1000_f64;
    let fees = pot * 0.1;
    let pot_for_rewards = pot - fees;
    println!("Transferring ${} (10%) of fees from the pot of ${}!", fees, pot);
    println!("Pot for the winners ${}!", pot_for_rewards);

    // mock winners
    let w1= Bet { account_id: 1, guess_numbers: [1, 2, 3, 4, 5, 6] };
    let w2= Bet { account_id: 2, guess_numbers: [1, 2, 3, 4, 5, 6] };
    let w3= Bet { account_id: 3, guess_numbers: [1, 2, 3, 4, 5, 6] };
    let w4= Bet { account_id: 4, guess_numbers: [1, 2, 3, 4, 5, 6] };
    let w5= Bet { account_id: 5, guess_numbers: [1, 2, 3, 4, 5, 6] };
    let w6= Bet { account_id: 6, guess_numbers: [1, 2, 3, 4, 5, 6] };

    // winner and their hits
    let winners: Winners<AccountId> = vec![
      (w1, 3),
      (w2, 3),
      (w3, 2),
      (w4, 5),
      (w5, 6),
      (w6, 4),
    ];

    // filter out "w3" because hits < 3
    let winners_to_reward: Winners<AccountId> = winners.into_iter().filter(|&(_, hits) | hits >= 3).collect();

    // group by hits
    let winners_grouped_by_hits = winners_to_reward.group_by(|(_, a_hits), (_, b_hits)| a_hits == b_hits);

    winners_grouped_by_hits.for_each(|winners| {
      let hits = winners[0].1; // eg.[ (Bet, hits), (Bet, hits) ]

      match hits {
        3 => {
          distribute_reward(0.03, winners, pot_for_rewards, hits.into());
        },
        4 => {
          distribute_reward(0.07, winners, pot_for_rewards, hits.into());
        },
        5 => {
          distribute_reward(0.15, winners, pot_for_rewards, hits.into());
        },
        6 => {
          distribute_reward(0.75, winners, pot_for_rewards, hits.into());
        },
        _ => println!("Error handle?"),
      }
    });

   fn distribute_reward(reward_percentage: f64, winners: &[(Bet<u32>, u8)],pot_for_rewards: f64, hits: u32) -> () {
    let reward_from_pot = pot_for_rewards * reward_percentage;
    let winners_count = winners.len() as f64;
    let reward_per_winner = reward_from_pot / winners_count;

    winners.iter().for_each(|winner| {
      let account = winner.0.account_id;
      println!("Account {} won ${} by guessing {} numbers!", account, reward_per_winner, hits);
    });
  }
}

