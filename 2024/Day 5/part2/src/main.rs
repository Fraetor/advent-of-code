use std::cmp::Ordering;

fn main() {
    let (ordering_rules, updates) = load("../input");

    // for rule in &ordering_rules {
    //     println!("{}|{}", rule.former, rule.latter);
    // }
    // for update in &updates {
    //     println!("{:?}", update);
    // }

    let mut p1_total = 0;

    let (ordered, unordered) = separate_updates(&updates, &ordering_rules);

    for update in ordered {
        let middle_value = update[(update.len()) / 2];
        // println!("{}", middle_value);
        p1_total += middle_value;
    }
    println!("Part 1 total: {p1_total}");

    let mut p2_total = 0;
    for update in unordered {
        let fixed = fix_update(&update, &relevant_rules(&update, &ordering_rules));
        let middle_value = fixed[(update.len()) / 2];
        // println!("{}", middle_value);
        p2_total += middle_value;
    }

    println!("Part 2 total: {p2_total}");
}

#[derive(Debug, Clone)]
struct OrderingRule {
    former: i32,
    latter: i32,
}

/// Load ordering rules and updates from file.
fn load(filename: &str) -> (Vec<OrderingRule>, Vec<Vec<i32>>) {
    let raw_input = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = raw_input.lines().collect();

    let mut ordering_rules = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.contains("|") {
            let numbers: Vec<_> = line.split("|").collect();
            ordering_rules.push(OrderingRule {
                former: numbers[0].parse().unwrap(),
                latter: numbers[1].parse().unwrap(),
            });
        } else if line.contains(",") {
            updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    return (ordering_rules, updates);
}

/// Returns a vector of rules relevant to the update.
fn relevant_rules(update: &Vec<i32>, rules: &Vec<OrderingRule>) -> Vec<OrderingRule> {
    let collect: Vec<OrderingRule> = rules
        .into_iter()
        .filter(|rule| update.contains(&rule.former) && update.contains(&rule.latter))
        .cloned()
        .collect();
    return collect;
}

/// Checks the given update satisfied the rules.
fn check_rules(update: &Vec<i32>, rules: &Vec<OrderingRule>) -> bool {
    for rule in rules {
        let former_index = update.iter().position(|x| x == &rule.former).unwrap();
        let latter_index = update.iter().position(|x| x == &rule.latter).unwrap();
        if former_index > latter_index {
            return false;
        }
    }
    return true;
}

/// Separates updates into correctly ordered and non-correctly ordered.
fn separate_updates(
    updates: &Vec<Vec<i32>>,
    rules: &Vec<OrderingRule>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut ordered = Vec::new();
    let mut unordered = Vec::new();

    for update in updates {
        if check_rules(&update, &relevant_rules(&update, &rules)) {
            ordered.push(update.clone());
        } else {
            unordered.push(update.clone());
        }
    }

    return (ordered, unordered);
}

/// Fixes an unordered update according to the relevant rules.
fn fix_update(update: &Vec<i32>, rules: &Vec<OrderingRule>) -> Vec<i32> {
    fn cmp(a: &i32, b: &i32, rules: &Vec<OrderingRule>) -> Ordering {
        let values = vec![*a, *b];
        match relevant_rules(&values, rules).first() {
            None => return Ordering::Equal,
            Some(rule) => {
                if rule.former == *a {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        }
    }
    let mut fixed_update = update.clone();
    fixed_update.sort_by(|a, b| cmp(a, b, rules));
    return fixed_update;
}
