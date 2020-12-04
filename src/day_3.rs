use crate::helper::file_to_vec;

fn line_to_chars(line : &String) -> Vec<char>
{
    let mut result : Vec<char> = Vec::new();
    for c in line.chars() {
        result.push(c);
    }
    result
}

struct ToboganPos {
    x : usize, y : usize
}

fn is_tree(grid : &[Vec<char>], pos : &ToboganPos)  -> bool{
    grid[pos.y][pos.x] == '#'
}

fn move_tobogan(pos : ToboganPos, dx : usize, dy : usize, n_cols : usize) -> ToboganPos{
    ToboganPos {
        x : (pos.x + dx) % n_cols,
        y : pos.y + dy
    }
}

fn get_trees_from_slope(dx: usize, dy : usize, grid : &[Vec<char>])  -> usize{
    let mut curr_pos : ToboganPos = ToboganPos { x : 0, y : 0 };
    let mut num_trees = 0;

    let n_lines = grid.len();
    let n_cols = grid[0].len();


    while curr_pos.y < n_lines {
        if is_tree(&grid, &curr_pos) {
            num_trees += 1;
        }
        curr_pos = move_tobogan(curr_pos, dx, dy, n_cols);
    }
    num_trees
}

pub fn day_2_solve() -> Option<()>
{
    let lines : Vec<String> = file_to_vec("src/day_2.txt".to_owned())?;
    let grid : Vec<Vec<char>> = lines.iter().map( |l| line_to_chars(l) ).collect();

    let part_1 = get_trees_from_slope(3, 1, &grid);

    // Part 2: Use part 1's solution, but map + filter.
    let slopes : Vec<(usize, usize)> = vec![ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) ];
    let part_2 : usize =
        slopes.iter()
              .map( |pair| get_trees_from_slope(pair.0, pair.1, &grid) )
              .product();

    println!("(Silver) Num Trees hit: {}", part_1);
    println!("(Gold) Num Trees hit: {}", part_2);
    Some(())
}
