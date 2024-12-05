fn main() {
    let (ordering_rules, updates) = load("../input");

    for rule in &ordering_rules {
        println!("{}|{}", rule.former, rule.latter);
    }
    for update in &updates {
        println!("{:?}", update);
    }

    let mut total = 0;
    for update in updates {
        if check_rules(&update, &relevant_rules(&update, &ordering_rules)) {
            let middle_value = update[(update.len()) / 2];
            println!("{}", middle_value);
            total += middle_value;
        }
    }

    println!("Total: {total}");
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
        let latter_index = update.iter().position(|x: &i32| x == &rule.latter).unwrap();
        if former_index > latter_index {
            return false;
        }
    }
    return true;
}
