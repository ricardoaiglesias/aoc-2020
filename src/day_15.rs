use std::collections::HashMap;

const SILVER_N_TURNS: usize = 30000000;
// const SILVER_N_TURNS : usize = 2020;

struct GameInfo {
    // Spoken Number -> (has it been said before, Turn it was last spoken).
    last_seen: HashMap<usize, (usize, Option<usize>)>,
}

type GameRecord = HashMap<usize, (usize, Option<usize>)>;

fn seen_before(last_seen: usize, map: &GameRecord) -> bool {
    map.get(&last_seen).is_some()
}

fn get_num_to_shout(last_seen: usize, game: &GameRecord) -> usize {
    match seen_before(last_seen, game) {
        true => {
            let (newer, older) = game.get(&last_seen).unwrap();

            match older {
                None => 0,
                Some(older_turn) => newer - older_turn,
            }
        }
        false => panic!(),
    }
}

fn speak(to_speak: usize, current_turn: usize, game: &mut GameRecord) {
    let updated_value = match game.contains_key(&to_speak) {
        true => {
            let (newer_turn, _older_turn) = *game.get(&to_speak).unwrap();
            (current_turn, Some(newer_turn))
        }
        false => (current_turn, None),
    };
    game.insert(to_speak, updated_value);
}

pub fn silver(data: &[usize]) {
    let mut game = GameInfo {
        last_seen: HashMap::new(),
    };

    for (turn_num, number) in data.iter().enumerate() {
        game.last_seen.insert(*number, (turn_num + 1, None));
    }

    let mut turn = data.len() + 1;
    let mut last_num = data[data.len() - 1];
    let mut shout_num = 0;
    while turn <= SILVER_N_TURNS {
        // if (turn % 100_000) == 0 { println!("{}", turn);}
        shout_num = get_num_to_shout(last_num, &game.last_seen);
        println!("{}", shout_num);
        speak(shout_num, turn, &mut game.last_seen);

        //alternate solution:
        // let j         let j = map.entry(last).or_insert(i);
        // last = i - *j;
        // *j = i;

        last_num = shout_num;
        turn += 1;
    }
    println!("shout_num: {}", shout_num);
}

pub fn gold() {}

pub fn setup() -> Vec<usize> {
    vec![15, 12, 0, 14, 3, 1]
    // vec![0,3,6]
}

pub fn day_15_soln() {
    let data = setup();
    silver(&data);
}
