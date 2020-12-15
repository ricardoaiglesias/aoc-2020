use crate::helper::file_to_vec;

pub struct DepartureTable {
    start_time: usize,
    bus_ids: Vec<usize>
}

fn earliest_departure_time(start_time: usize, bus_id: usize) -> usize {
    let remainder = start_time % bus_id;
    let can_depart_now : bool = remainder == 0;
    if can_depart_now { return start_time; }

    // Otherwise, start time is "bus_id - remainder"
    start_time + (bus_id - remainder)
}

pub fn silver(data: &DepartureTable) {
    let earliest : (&usize, usize)= data.bus_ids.iter().map(
        |id| (id, earliest_departure_time(data.start_time, *id))
    ).min_by_key(|val| val.1).unwrap();

    println!("(Silver): Earliest departure time: {:?}", earliest);
    println!("ID: {} | Wait Time: {}", earliest.0, earliest.1 - data.start_time );
    println!("Result: {}", earliest.0 * (earliest.1 - data.start_time));
}

fn will_depart_at(time: usize, id: usize) -> bool{
    time % id == 0
}

pub fn gold(data: &[(usize, usize)]) -> usize{
    let start_time: usize = 0;

    let mut step = data[0].1;
    let mut time = start_time;

    loop {
        // On the IDs that haven't been found, check whether the current
        // timestamp will work.
        let bus_state = data.iter().map( |(index, id)| (id, will_depart_at(time + index, *id)));
        let arrivals = bus_state.clone().filter(|(_id, found)| *found);

        if arrivals.clone().any(|(_id, found)| found) {
            step = arrivals.map(|(id, _)| id).product();
        }


        let result: bool = bus_state.clone().all(|(_id, found)| found);
        if result {
            println!("Found a solution at : {}", time);
            return time;
        }

        time += step;
    }
}

fn setup_gold () -> Vec<(usize, usize)> {
    let vec = file_to_vec("src/13_input.txt").unwrap();
    let num_vec: Vec<(usize, usize)> =
        vec[1].split(',').map(|x| x.parse::<usize>())
                         .enumerate()
                         .filter(|(_index, value)| value.is_ok())
                         .map(|(index, value)| (index, value.unwrap())).collect();

    num_vec
}

pub fn setup() -> DepartureTable{
    let vec = file_to_vec("src/13_input.txt").unwrap();
    let start_time = vec[0].parse::<usize>().unwrap();

    let num_vec = vec[1].split(',').filter_map(|x| {
        let val = x.parse::<usize>();
        val.ok()}).collect();

    DepartureTable {
        start_time,
        bus_ids: num_vec
    }
}

pub fn day_13_soln() {
    // println!("{}", earliest_departure_time(939, 7));
    // println!("{}", earliest_departure_time(939, 13));
    // println!("{}", earliest_departure_time(939, 59));
    // println!("{}", earliest_departure_time(939, 31));
    // println!("{}", earliest_departure_time(939, 19));

    let data = setup();
    silver(&data);

    let test_data = vec![(0, 17), (2, 13), (3, 19)];
    gold(&test_data);
    let test_data = vec![(0, 67), (1, 7), (2, 59), (3, 61)];
    gold(&test_data);

    let test_data =vec![(0, 67),(2, 7),(3, 59),(4, 61)];
    assert!(gold(&test_data) == 779210); // 779210.

    let test_data = vec![(0, 67), (1, 7), (3, 59),(4, 61)];
    assert!(gold(&test_data) == 1261476);

    let test_data = vec![(0, 1789),(1, 37),(2, 47),(3, 1889)];
    assert!(gold(&test_data) == 1202161486);

    let data_gold = setup_gold();
    assert!(gold(&data_gold) == 266204454441577);
}
