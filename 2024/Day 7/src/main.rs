/// A single calibration test.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CalibrationTest {
    result: i64,
    input: Vec<i64>,
}

impl CalibrationTest {
    /// Construct a CalibrationTest from a single line string of the form:
    /// `"RESULT: INPUT1 INPUT2 ..."`
    ///
    /// ## Examples
    ///
    /// ```
    /// let ct = CalibrationTest::from("190: 10 19");
    /// assert_eq!(ct, CalibrationTest{result: 190, input: vec![10, 19]});
    /// ```
    fn from_str(value: &str) -> Result<Self, std::num::ParseIntError> {
        let strings: Vec<&str> = value.splitn(2, ":").collect();
        let result = strings[0].parse()?;

        let maybe_input: Result<Vec<i64>, _> =
            strings[1].split_whitespace().map(|s| s.parse()).collect();
        let input: Vec<i64> = maybe_input?;

        return Ok(CalibrationTest { result, input });
    }

    /// Tests whether adding add and multiply operators between the input values
    /// can produce the result when evaluated left to right.
    fn possibly_true_p1(&self) -> bool {
        let mut reversed_input = self.input.clone();
        reversed_input.reverse();
        target_achievable(&reversed_input, self.result)
    }

    /// Tests whether adding add, multiply, and concatenation operators between
    /// the input values can produce the result when evaluated left to right.
    fn possibly_true_p2(&self) -> bool {
        let mut reversed_input = self.input.clone();
        reversed_input.reverse();
        target_achievable_v2(&reversed_input, self.result)
    }
}

/// Target can be made by applying operations to input.
fn target_achievable(input: &[i64], target: i64) -> bool {
    // eprintln!("Input: {:?}, Target: {}", input, target);
    return match input.len() {
        0 => target == 0,
        1 => target == input[0],
        _ => {
            let mut multiply_stack = input.to_vec();
            let product = multiply_stack.pop().unwrap() * multiply_stack.pop().unwrap();
            multiply_stack.push(product);

            let mut addition_stack = input.to_vec();
            let sum = addition_stack.pop().unwrap() + addition_stack.pop().unwrap();
            addition_stack.push(sum);

            // Terminate early for cases where we have overshot.
            if product > target && sum > target {
                return false;
            }

            // Terminate early if it works.
            return target_achievable(&multiply_stack, target)
                || target_achievable(&addition_stack, target);
        }
    };
}

/// Target can be made by applying operations to input.
fn target_achievable_v2(input: &[i64], target: i64) -> bool {
    // eprintln!("Input: {:?}, Target: {}", input, target);
    return match input.len() {
        0 => target == 0,
        1 => target == input[0],
        _ => {
            let mut stack = input.to_vec();
            let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());

            let product = a * b;
            let mut multiply_stack = stack.clone();
            multiply_stack.push(product);

            let sum = a + b;
            let mut addition_stack = stack.clone();
            addition_stack.push(sum);

            let concatenation = a * i64::pow(10, b.ilog10() + 1) + b;
            let mut concatenation_stack = stack.clone();
            concatenation_stack.push(concatenation);

            // Terminate early for cases where we have overshot.
            if product > target && sum > target && concatenation > target {
                return false;
            }

            // Terminate if it works.
            return target_achievable_v2(&multiply_stack, target)
                || target_achievable_v2(&addition_stack, target)
                || target_achievable_v2(&concatenation_stack, target);
        }
    };
}

fn load(filename: &str) -> Vec<CalibrationTest> {
    let raw_input = std::fs::read_to_string(filename).unwrap();
    let calibration_tests: Vec<CalibrationTest> = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| CalibrationTest::from_str(l).unwrap())
        .collect();
    return calibration_tests;
}

fn part_1(calibration_tests: &[CalibrationTest]) -> usize {
    let total_calibration_result: i64 = calibration_tests
        .iter()
        // .inspect(|ct| eprintln!("Testing {:?}", ct))
        .filter(|ct| ct.possibly_true_p1())
        // .inspect(|ct| eprintln!("{:?} is valid.", ct))
        .map(|ct| ct.result)
        .sum();
    return total_calibration_result.try_into().unwrap();
}

fn part_2(calibration_tests: &[CalibrationTest]) -> usize {
    let total_calibration_result: i64 = calibration_tests
        .iter()
        .filter(|ct| ct.possibly_true_p2())
        .map(|ct| ct.result)
        .sum();
    return total_calibration_result.try_into().unwrap();
}

fn main() {
    let calibration_tests = load("input");

    let p1_total = part_1(&calibration_tests);
    println!("Part 1 total: {p1_total}");

    let p2_total = part_2(&calibration_tests);
    println!("Part 2 total: {p2_total}");
}
