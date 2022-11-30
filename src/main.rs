use multimap::MultiMap;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Advent of Code 2020: https://adventofcode.com/2020
fn main() {
    /*
        let data01 = file_to_numbers("input\\01.txt").expect("data read issue");
        let data02 = file_to_strings("input\\02.txt");
        let data03 = file_to_strings("input\\03.txt");
        let data04 = file_to_strings("input\\04.txt");
        let data05 = file_to_strings("input\\05.txt");
        let data06 = file_to_strings("input\\06.txt");
        let data07 = file_to_strings("input\\07.txt");
        let data08 = file_to_strings("input\\08.txt");
        let data09 = file_to_numbers("input\\09.txt").expect("data read issue");
        let data10 = file_to_numbers("input\\10.txt").expect("data read issue");
        let data11 = file_to_strings("input\\11.txt");
    */
    let data12 = file_to_strings("input\\12.txt");
    let data13 = file_to_strings("input\\13.txt");

    /*
        println!("Day  1.1: {}", day_01_1(&data01));
        println!("Day  1.2: {}", day_01_2(&data01));
        println!("Day  2.1: {}", day_02_1(&data02));
        println!("Day  2.2: {}", day_02_2(&data02));
        println!("Day  3.1: {}", day_03_1(&data03, 3, 1));
        println!("Day  3.2: {}", day_03_2(&data03));
        println!("Day  4.1: {}", day_04_1(&data04));
        println!("Day  4.2: {}", day_04_2(&data04));
        println!("Day  5.1: {}", day_05_1(&data05));
        println!("Day  5.2: {}", day_05_2(&data05));
        println!("Day  6.1: {}", day_06_1(&data06));
        println!("Day  6.2: {}", day_06_2(&data06));
        println!("Day  7.1: {}", day_07_1(&data07));
        println!("Day  7.2: {}", day_07_2(&data07));
        println!("Day  8.1: {}", day_08_1(&data08));
        println!("Day  8.2: {}", day_08_2(&data08));
        println!("Day  9.1: {}", day_09_1(&data09, 25));
        println!("Day  9.2: {}", day_09_2(&data09, 25));
        println!("Day 10.1: {}", day_10_1(&data10));
        println!("Day 10.2: {}", day_10_2(&data10));
        println!("Day 11.1: {}", day_11_1(&data11));
        println!("Day 11.2: {}", day_11_2(&data11));
    */
    println!("Day 12.1: {}", day_12_1(&data12));
    println!("Day 12.2: {}", day_12_2(&data12));
    println!("Day 13.1: {}", day_13_1(&data13));
}

#[allow(dead_code)]
fn file_to_numbers(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename).expect("no such file");
    let br = BufReader::new(file);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

#[allow(dead_code)]
fn file_to_strings(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_01_1(v: &Vec<i64>) -> i64 {
    let mut result = 0;

    'outer: for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i] + v[j] == 2020 {
                result = v[i] * v[j];
                break 'outer;
            }
        }
    }

    result
}

#[allow(dead_code)]
fn day_01_2(v: &Vec<i64>) -> i64 {
    let mut result = 0;

    'outer: for i in 0..v.len() {
        for j in 0..v.len() {
            for k in 0..v.len() {
                if v[i] + v[j] + v[k] == 2020 {
                    result = v[i] * v[j] * v[k];
                    break 'outer;
                }
            }
        }
    }

    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_02_1(v: &Vec<String>) -> i64 {
    let mut result = 0;

    for line in v.iter() {
        let pw: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let bounds: Vec<String> = pw[0].split("-").map(str::to_string).collect();
        let min: usize = bounds[0].parse().unwrap();
        let max: usize = bounds[1].parse().unwrap();
        let count = pw[2].matches(pw[1].chars().nth(0).unwrap()).count();
        if count >= min && count <= max {
            result += 1;
        }
    }

    result
}

#[allow(dead_code)]
fn day_02_2(v: &Vec<String>) -> i64 {
    let mut result = 0;

    for line in v.iter() {
        let pw: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let bounds: Vec<String> = pw[0].split("-").map(str::to_string).collect();
        let min: usize = bounds[0].parse().unwrap();
        let max: usize = bounds[1].parse().unwrap();
        let mut count = 0;

        if pw[2].chars().nth(min - 1).unwrap() == pw[1].chars().nth(0).unwrap() {
            count += 1;
        }
        if pw[2].chars().nth(max - 1).unwrap() == pw[1].chars().nth(0).unwrap() {
            count += 1;
        }
        if count == 1 {
            result += 1;
        }
    }

    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_03_1(v: &Vec<String>, right: usize, down: usize) -> i64 {
    let mut result = 0;
    let mut x = 0;
    let mut y = 0;
    let max_x = v[0].len();
    let max_y = v.len();
    while y != max_y - 1 {
        x += right;
        if x >= max_x {
            x -= max_x;
        }
        y += down;

        if v[y].chars().nth(x).unwrap() == '#' {
            result += 1;
        }
    }

    result
}

#[allow(dead_code)]
fn day_03_2(v: &Vec<String>) -> i64 {
    day_03_1(&v, 1, 1)
        * day_03_1(&v, 3, 1)
        * day_03_1(&v, 5, 1)
        * day_03_1(&v, 7, 1)
        * day_03_1(&v, 1, 2)
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_04_1(v: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut field_count = 0;

    for line in v.iter() {
        if line.len() == 0 {
            if field_count >= 7 {
                result += 1;
            }
            field_count = 0;
        } else {
            let pairs: Vec<String> = line.split_whitespace().map(str::to_string).collect();

            for pair in pairs.iter() {
                match &pair[..3] {
                    "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => field_count += 1,
                    _ => (),
                }
            }
        }
    }
    if field_count >= 7 {
        result += 1;
    }

    result
}

#[allow(dead_code)]
fn day_04_2(v: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut field_count = 0;
    let re_hcl = Regex::new(r"#[0-9A-Fa-f]{6}").unwrap(); //#, six 0-9 a-f
    let re_pid = Regex::new(r"\d{9}").unwrap(); //9 numbers

    for line in v.iter() {
        if line.len() == 0 {
            if field_count >= 7 {
                result += 1;
            }
            field_count = 0;
        } else {
            let pairs: Vec<String> = line.split_whitespace().map(str::to_string).collect();

            for pair in pairs.iter() {
                match &pair[..3] {
                    "byr" => {
                        let year: i32 = pair[4..].parse().unwrap();
                        if year >= 1920 && year <= 2002 {
                            field_count += 1;
                        }
                    }
                    "iyr" => {
                        let year: i32 = pair[4..].parse().unwrap();
                        if year >= 2010 && year <= 2020 {
                            field_count += 1;
                        }
                    }
                    "eyr" => {
                        let year: i32 = pair[4..].parse().unwrap();
                        if year >= 2020 && year <= 2030 {
                            field_count += 1;
                        }
                    }
                    "hgt" => {
                        let len = pair.len();
                        match &pair[len - 2..] {
                            "cm" => {
                                let h: i32 = pair[4..len - 2].parse().unwrap();
                                if h >= 150 && h <= 193 {
                                    field_count += 1;
                                }
                            }
                            "in" => {
                                let h: i32 = pair[4..len - 2].parse().unwrap();
                                if h >= 59 && h <= 76 {
                                    field_count += 1;
                                }
                            }
                            _ => (),
                        }
                    }
                    "hcl" => {
                        if re_hcl.is_match(&pair[4..]) && pair.len() == 11 {
                            field_count += 1;
                        }
                    }
                    "ecl" => match &pair[4..] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => field_count += 1,
                        _ => (),
                    },
                    "pid" => {
                        if re_pid.is_match(&pair[4..]) && pair.len() == 13 {
                            field_count += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    if field_count >= 7 {
        result += 1;
    }

    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_05_1(v: &Vec<String>) -> i64 {
    let mut result = 0;

    for line in v.iter() {
        let mut min = 0;
        let mut max = 127;

        for i in 0..7 {
            if line.chars().nth(i).unwrap() == 'F' {
                max = max - ((max + 1) - min) / 2;
            } else {
                min = min + ((max + 1) - min) / 2;
            }
        }
        let row = min;
        min = 0;
        max = 7;
        for i in 7..10 {
            if line.chars().nth(i).unwrap() == 'L' {
                max = max - ((max + 1) - min) / 2;
            } else {
                min = min + ((max + 1) - min) / 2;
            }
        }
        let col = min;
        let seat = row * 8 + col;
        if seat > result {
            result = seat;
        }
    }

    result
}

#[allow(dead_code)]
fn day_05_2(v: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut seats = Vec::new();

    for line in v.iter() {
        let mut min = 0;
        let mut max = 127;

        for i in 0..7 {
            if line.chars().nth(i).unwrap() == 'F' {
                max = max - ((max + 1) - min) / 2;
            } else {
                min = min + ((max + 1) - min) / 2;
            }
        }
        let row = min;
        min = 0;
        max = 7;
        for i in 7..10 {
            if line.chars().nth(i).unwrap() == 'L' {
                max = max - ((max + 1) - min) / 2;
            } else {
                min = min + ((max + 1) - min) / 2;
            }
        }
        let col = min;
        let seat = row * 8 + col;
        seats.push(seat);
    }
    seats.sort();
    for i in 0..seats.len() - 2 {
        if seats[i] + 1 != seats[i + 1] {
            result = seats[i] + 1;
            break;
        }
    }

    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_06_1(v: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut s: String = String::new();

    for line in v.iter() {
        if line.len() == 0 {
            result += day_06_question_count(&s);
            s.clear();
        } else {
            s.push_str(line);
        }
    }
    result += day_06_question_count(&s);

    result
}

#[allow(dead_code)]
fn day_06_question_count(s: &String) -> i64 {
    let mut questions: Vec<char> = s.chars().collect();
    questions.sort_unstable();
    questions.dedup();
    questions.len() as i64
}

#[allow(dead_code)]
fn day_06_2(v: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut same_questions: HashSet<_> = HashSet::new();
    let mut first = true;

    for line in v.iter() {
        if line.len() == 0 {
            result += same_questions.len() as i64;
            same_questions.clear();
            first = true;
        } else {
            if first {
                same_questions = line.chars().collect();
                first = false;
            } else {
                let these_questions: HashSet<_> = line.chars().collect();
                same_questions = same_questions
                    .intersection(&these_questions)
                    .copied()
                    .collect();
            }
        }
    }
    result += same_questions.len() as i64;

    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_07_1(v: &Vec<String>) -> i64 {
    let mut map: MultiMap<String, String> = MultiMap::new();
    let mut check: Vec<String> = Vec::new();
    let mut list: HashSet<String> = HashSet::new();

    day_07_load_map(&v, &mut map, false);
    check.push(String::from("shinygold"));

    while check.len() > 0 {
        let child: String = check.pop().unwrap();
        if map.contains_key(&child) {
            let parents = map.get_vec(&child).unwrap();
            for parent in parents.iter() {
                if list.insert(parent.clone()) {
                    check.push(parent.to_string());
                }
            }
        }
    }

    list.len() as i64
}

#[allow(dead_code)]
fn day_07_load_map(v: &Vec<String>, m: &mut MultiMap<String, String>, parent_first: bool) {
    for line in v.iter() {
        let parse: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let mut parent: String = parse[0].clone();
        parent.push_str(parse[1].as_str());

        let mut i = 4;
        while i < parse.len() {
            if parse[i] == "no" {
                break;
            } else {
                let mut child: String = parse[i + 1].clone();
                child.push_str(parse[i + 2].as_str());
                if parent_first {
                    for _i in 0..parse[i].parse().unwrap() {
                        m.insert(parent.clone(), child.clone());
                    }
                } else {
                    m.insert(child, parent.clone());
                }
                i += 4;
            }
        }
    }
}

#[allow(dead_code)]
fn day_07_2(v: &Vec<String>) -> i64 {
    let mut map: MultiMap<String, String> = MultiMap::new();

    day_07_load_map(&v, &mut map, true);
    (day_07_count_bags(&map, String::from("shinygold")) - 1) as i64
}

#[allow(dead_code)]
fn day_07_count_bags(map: &MultiMap<String, String>, key: String) -> usize {
    let mut result = 1;

    if map.contains_key(&key) {
        let bags = map.get_vec(&key).unwrap();
        for bag in bags.iter() {
            result += day_07_count_bags(&map, bag.to_string());
        }
    }
    result
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_08_1(v: &Vec<String>) -> i64 {
    let mut abort = false;
    day_08_computer(&v, &mut abort)
}

#[allow(dead_code)]
fn day_08_2(v: &Vec<String>) -> i64 {
    let mut acc = 0;
    let mut abort = false;

    for i in 0..v.len() {
        // Swap this instruction
        let mut instruction: String = String::from("");
        match &v[i][0..3] {
            "jmp" => {
                instruction = String::from("nop");
                instruction.push_str(&v[i][3..]);
            }
            "nop" => {
                instruction = String::from("jmp");
                instruction.push_str(&v[i][3..]);
            }
            _ => (),
        }

        if instruction.len() > 0 {
            // Copy the instructions, swap this line
            let mut v_temp: Vec<String> = v.clone();
            v_temp[i] = instruction;

            // Run the computer and stop trying if successful
            acc = day_08_computer(&v_temp, &mut abort);
            if !abort {
                break;
            }
        }
    }

    acc
}

#[allow(dead_code)]
fn day_08_computer(v: &Vec<String>, abort: &mut bool) -> i64 {
    let mut ip = 0;
    let mut acc: i64 = 0;
    let mut has_exec: HashSet<_> = HashSet::new();

    *abort = false;
    loop {
        if ip >= v.len() {
            break;
        } else if has_exec.insert(ip) {
            match &v[ip][0..3] {
                "jmp" => {
                    let offset: usize = v[ip][5..].parse().unwrap();
                    if &v[ip][4..5] == "+" {
                        ip += offset;
                    } else {
                        ip -= offset;
                    }
                }
                "acc" => {
                    let offset: i64 = v[ip][5..].parse().unwrap();
                    if &v[ip][4..5] == "+" {
                        acc += offset;
                    } else {
                        acc -= offset;
                    }
                    ip += 1
                }
                "nop" => ip += 1,
                _ => (),
            }
        } else {
            *abort = true;
            break;
        }
    }

    acc
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_09_1(v: &Vec<i64>, preamble: usize) -> i64 {
    let mut index = 0;
    day_09_find_bad(v, preamble, &mut index)
}

#[allow(dead_code)]
fn day_09_find_bad(v: &Vec<i64>, preamble: usize, index: &mut usize) -> i64 {
    let mut bad_number = 0;

    'outer: for i in preamble..v.len() {
        for j in i - preamble..i - 1 {
            for k in j + 1..i {
                if v[i] == v[j] + v[k] {
                    // Valid, move on
                    continue 'outer;
                }
            }
        }
        bad_number = v[i];
        *index = i;
        break;
    }

    bad_number
}

#[allow(dead_code)]
fn day_09_2(v: &Vec<i64>, preamble: usize) -> i64 {
    let mut bad_number = 0;
    let mut found = 0;

    day_09_find_bad(v, preamble, &mut found);

    'outer2: for j in 0..found - 1 {
        let mut sum = 0;
        for k in j..found {
            sum += v[k];
            if sum == v[found] {
                let mut min = v[j];
                let mut max = v[j];
                for a in j..k + 1 {
                    min = cmp::min(min, v[a]);
                    max = cmp::max(max, v[a]);
                }
                bad_number = min + max;
                break 'outer2;
            }
        }
    }

    bad_number
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_10_1(input: &Vec<i64>) -> i64 {
    let mut v = input.clone();
    v.sort();
    let mut current = 0;
    let mut one_count = 0;
    let mut three_count = 0;

    for i in 0..v.len() {
        match v[i] - current {
            1 => one_count += 1,
            3 => three_count += 1,
            _ => println!("Unexpected voltage difference!"),
        }
        current = v[i];
    }
    three_count += 1;

    one_count * three_count
}

// To find the combinations, find the differences and count the strings of 1s
// 1 = 2 combinations
// 2 = 4
// 3 = 7
// Mulitply the combinations
// Ex:  11333133311113 is 4 * 2 * 7 = 56
#[allow(dead_code)]
fn day_10_2(input: &Vec<i64>) -> i64 {
    let mut v = input.clone();
    v.sort();
    let mut current = 0;
    let mut one_count = 0;
    let mut combinations = 1;

    for i in 0..v.len() {
        match v[i] - current {
            1 => one_count += 1,
            3 => {
                match one_count {
                    0 | 1 => (),
                    2 => combinations *= 2,
                    3 => combinations *= 4,
                    4 => combinations *= 7,
                    _ => println!(
                        "Unexpected string of one-voltage differences!  {}",
                        one_count
                    ),
                }
                one_count = 0;
            }
            _ => println!("Unexpected voltage difference!"),
        }
        current = v[i];
    }
    match one_count {
        0 | 1 => (),
        2 => combinations *= 2,
        3 => combinations *= 4,
        4 => combinations *= 7,
        _ => println!("Unexpected string of one-voltage differences!"),
    }

    combinations
}
////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_11_1(v: &Vec<String>) -> i64 {
    let mut x: Vec<Vec<Option<bool>>> = vec![vec![None; v[0].len()]; v.len()];
    day_11_load(v, &mut x);
    while day_11_people_move(&mut x) {}
    day_11_count_occupied(&x)
}

#[allow(dead_code)]
fn day_11_2(v: &Vec<String>) -> i64 {
    let mut x: Vec<Vec<Option<bool>>> = vec![vec![None; v[0].len()]; v.len()];
    day_11_load(v, &mut x);
    while day_11_people_move2(&mut x) {}
    //day_11_people_move2(&mut x);
    //day_11_people_move2(&mut x);
    day_11_count_occupied(&x)
}

#[allow(dead_code)]
fn day_11_load(v: &Vec<String>, x: &mut Vec<Vec<Option<bool>>>) {
    for row in 0..v.len() {
        for col in 0..v[0].len() {
            match v[row].chars().nth(col).unwrap() {
                'L' => x[row][col] = Some(false),
                '#' => x[row][col] = Some(true),
                '.' => x[row][col] = None,
                _ => println!(
                    "Unexpected chair descriptor: {}",
                    v[row].chars().nth(col).unwrap()
                ),
            }
        }
    }
}

#[allow(dead_code)]
fn day_11_count_occupied(x: &Vec<Vec<Option<bool>>>) -> i64 {
    let mut count = 0;
    for row in 0..x.len() {
        for col in 0..x[0].len() {
            if x[row][col].is_some() && x[row][col].unwrap() == true {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn day_11_people_move(x: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut change_ocurred = false;
    let o = x.clone();
    for row in 0..x.len() {
        for col in 0..x[0].len() {
            if o[row][col].is_some() {
                // Find adjacent occupied seats, handling board edges
                let mut adjacent = 0;
                for i in {
                    if row > 0 {
                        row - 1
                    } else {
                        0
                    }
                }..cmp::min(x.len(), row + 2)
                {
                    for j in {
                        if col > 0 {
                            col - 1
                        } else {
                            0
                        }
                    }..cmp::min(x[0].len(), col + 2)
                    {
                        // Don't count yourself
                        if !(i == row && j == col) && o[i][j].is_some() && o[i][j].unwrap() == true
                        {
                            adjacent += 1;
                        }
                    }
                }
                //println!("{},{} = {}", row, col, adjacent);
                if o[row][col].unwrap() == true && adjacent >= 4 {
                    x[row][col] = Some(false);
                    change_ocurred = true;
                } else if o[row][col].unwrap() == false && adjacent == 0 {
                    x[row][col] = Some(true);
                    change_ocurred = true;
                }
            }
        }
    }
    change_ocurred
}

#[allow(dead_code)]
fn day_11_people_move2(x: &mut Vec<Vec<Option<bool>>>) -> bool {
    let mut change_ocurred = false;
    let o = x.clone();
    for row in 0..x.len() {
        for col in 0..x[0].len() {
            if o[row][col].is_some() {
                let mut adjacent = 0;

                // Find adjacent occupied seats at any distance, handling board edges
                adjacent += day_11_find_seat(&o, row as i32, col as i32, -1, 0);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, -1, 1);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, 0, 1);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, 1, 1);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, 1, 0);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, 1, -1);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, 0, -1);
                adjacent += day_11_find_seat(&o, row as i32, col as i32, -1, -1);

                if o[row][col].unwrap() == true && adjacent >= 5 {
                    x[row][col] = Some(false);
                    change_ocurred = true;
                } else if o[row][col].unwrap() == false && adjacent == 0 {
                    x[row][col] = Some(true);
                    change_ocurred = true;
                }
            }
        }
    }
    change_ocurred
}

#[allow(dead_code)]
fn day_11_find_seat(x: &Vec<Vec<Option<bool>>>, row: i32, col: i32, drow: i32, dcol: i32) -> i64 {
    let mut adjacent = 0;
    let mut wrow = row;
    let mut wcol = col;
    let mut lastrow = wrow;
    let mut lastcol = wcol;
    let mut looper = 0;
    loop {
        // Move as long as you are in the matrix
        wrow = cmp::min(cmp::max(0, wrow + drow), x.len() as i32 - 1);
        wcol = cmp::min(cmp::max(0, wcol + dcol), x[0].len() as i32 - 1);
        // If you didn't move, bail  OPERATOR PRECIDENCE!!
        if ((drow != 0) && (wrow == lastrow)) || ((dcol != 0) && (wcol == lastcol)) {
            break;
        }
        lastrow = wrow;
        lastcol = wcol;

        // If you find a seat, evaluate it, then bail
        if x[wrow as usize][wcol as usize].is_some() {
            if x[wrow as usize][wcol as usize].unwrap() == true {
                adjacent += 1;
            }
            break;
        }
        looper += 1;
        if looper > 50 {
            break;
        }
    }

    adjacent
}

////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn day_12_1(v: &Vec<String>) -> i64 {
    let mut d: i64 = 0; //east, positive clockwise
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for line in v.iter() {
        let key = &line[0..1];
        let dist = &line[1..].parse().unwrap();

        match key {
            "N" => y += dist,
            "S" => y -= dist,
            "E" => x += dist,
            "W" => x -= dist,
            "L" => d = (d - dist) % 360,
            "R" => d = (d + dist) % 360,
            "F" => match d {
                0 => x += dist,
                90 | -270 => y -= dist,
                180 | -180 => x -= dist,
                270 | -90 => y += dist,
                _ => println!("Unexpected direction."),
            },
            _ => println!("Unexpected key."),
        }
    }

    i64::abs(x) + i64::abs(y)
}

#[allow(dead_code)]
fn day_12_2(v: &Vec<String>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut wx: i64 = 10;
    let mut wy: i64 = 1;

    for line in v.iter() {
        let key = &line[0..1];
        let dist = &line[1..].parse().unwrap();

        match key {
            "N" => wy += dist,
            "S" => wy -= dist,
            "E" => wx += dist,
            "W" => wx -= dist,
            "L" => match dist {
                0 => (),
                90 | -270 => {
                    let tmp = wx;
                    wx = -wy;
                    wy = tmp;
                }
                180 | -180 => {
                    wx = -wx;
                    wy = -wy;
                }
                270 | -90 => {
                    let tmp = wx;
                    wx = wy;
                    wy = -tmp;
                }
                _ => println!("Unexpected direction."),
            },
            "R" => match dist {
                0 => (),
                90 | -270 => {
                    let tmp = wx;
                    wx = wy;
                    wy = -tmp;
                }
                180 | -180 => {
                    wx = -wx;
                    wy = -wy;
                }
                270 | -90 => {
                    let tmp = wx;
                    wx = -wy;
                    wy = tmp;
                }
                _ => println!("Unexpected direction."),
            },
            "F" => {
                x += wx * dist;
                y += wy * dist;
            }

            _ => println!("Unexpected key."),
        }
    }

    i64::abs(x) + i64::abs(y)
}

#[allow(dead_code)]
fn day_13_1(v: &Vec<String>) -> i64 {
    let depart: i64 = v[0].parse().unwrap();
    let busmap: Vec<String> = v[1].split(",").map(str::to_string).collect();
    let mut valid_buses: Vec<i64> = Vec::new();
    let mut min_wait: i64 = i64::MAX;
    let mut best_bus = 0;

    for bus in busmap {
        if bus != "x" {
            valid_buses.push(bus.parse::<i64>().unwrap());
        }
    }
    for bus_num in valid_buses {
        let divisor: i64 = depart / bus_num;
        let base = divisor * bus_num;
        if base == bus_num {
            min_wait = 0;
            best_bus = bus_num;
        } else {
            let this_wait: i64 = base + bus_num - depart;
            if this_wait < min_wait {
                min_wait = this_wait;
                best_bus = bus_num;
            }
        }
    }

    best_bus * min_wait
}

#[allow(dead_code)]
fn day_13_2(v: &Vec<String>) -> i64 {
    let depart: i64 = v[0].parse().unwrap();
    let busmap: Vec<String> = v[1].split(",").map(str::to_string).collect();
    let mut valid_buses: Vec<i64> = Vec::new();
    let mut min_wait: i64 = i64::MAX;
    let mut best_bus = 0;

    for bus in busmap {
        if bus != "x" {
            valid_buses.push(bus.parse::<i64>().unwrap());
        }
    }
    for bus_num in valid_buses {
        let divisor: i64 = depart / bus_num;
        let base = divisor * bus_num;
        if base == bus_num {
            min_wait = 0;
            best_bus = bus_num;
        } else {
            let this_wait: i64 = base + bus_num - depart;
            if this_wait < min_wait {
                min_wait = this_wait;
                best_bus = bus_num;
            }
        }
    }

    best_bus * min_wait
}
/*
fn day_05_1(v: &Vec<String>) -> i64 {
    0
}
fn day_05_2(v: &Vec<String>) -> i64 {
    0
}
*/
