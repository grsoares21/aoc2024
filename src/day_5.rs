use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn is_order_valid(rules: &HashMap<i32, HashSet<i32>>, page_order: &Vec<i32>) -> bool {
    let reversed_page_order: Vec<&i32> = page_order.iter().rev().collect();
    for (index, page) in reversed_page_order.iter().enumerate() {
        let get_page_rules = rules.get(page);

        match get_page_rules {
            Some(page_rules) => {
                for page_before in reversed_page_order[index + 1..page_order.len()].iter() {
                    if page_rules.contains(page_before) {
                        return false;
                    }
                }
            }
            _ => (),
        }
    }

    return true;
}

pub fn parse_input(input: String) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules_input = sections[0];
    let pages_input = sections[1];

    // for each number, stores which other numbers have to come *after* it
    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    rules_input.lines().for_each(|line| {
        let rule_parts = line.split('|').collect::<Vec<&str>>();
        let before_number = rule_parts[0].parse::<i32>().unwrap();
        let after_number = rule_parts[1].parse::<i32>().unwrap();

        let rule_set = rules_map.get_mut(&before_number);

        match rule_set {
            Some(set) => {
                set.insert(after_number);
            }
            None => {
                rules_map.insert(before_number, HashSet::from([after_number]));
            }
        }
    });

    let pages: Vec<Vec<i32>> = pages_input
        .split('\n')
        .map(|page| {
            page.split(',')
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    return (rules_map, pages);
}

pub fn part_1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let (rules_map, pages) = parse_input(input);

    let mut total_sum = 0;

    pages.iter().for_each(|page_order| {
        if is_order_valid(&rules_map, page_order) {
            total_sum += page_order[page_order.len() / 2]
        }
    });

    Ok(total_sum)
}

pub fn part_2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    reader.get_ref().seek(SeekFrom::Start(0))?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    // the initial parsing here is just the same as part 1,
    let (rules_map, pages) = parse_input(input);

    let mut total_sum = 0;

    pages.iter().for_each(|page_order| {
        if !is_order_valid(&rules_map, page_order) {
            let mut new_order = page_order.clone();
            new_order.sort_by(|a, b| {
                let rule = rules_map.get(a);

                match rule {
                    Some(pages) => {
                        if pages.contains(b) {
                            return Ordering::Less;
                        }

                        return Ordering::Greater;
                    },
                    _ => return Ordering::Greater
                }
            });

            total_sum += new_order[new_order.len() / 2];
        }

    });

    Ok(total_sum)
}
