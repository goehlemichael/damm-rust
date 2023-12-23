use std::io;

fn damm_check_digit(sequence: &str) -> char {
    let matrix: [[usize; 10]; 10] = [
        [0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
        [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
        [4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
        [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
        [6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
        [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
        [5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
        [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
        [9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
        [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
    ];

    let mut interim = 0;
    for digit in sequence.chars() {
        let digit_value = digit.to_digit(10).unwrap() as usize;
        interim = matrix[interim][digit_value];
    }

    (interim as u8 + b'0') as char
}

fn generate_with_check_digit(start: u32, end: u32) -> Vec<u32> {
    (start..=end)
        .map(|num| {
            let sequence = num.to_string();
            let check_digit = damm_check_digit(&sequence);
            format!("{}{}", sequence, check_digit).parse().unwrap()
        })
        .collect()
}

fn main() {
   let mut start = String::new();
   let mut end = String::new();

   println!("Enter the start of the range:");
   io::stdin().read_line(&mut start).expect("Failed to read input");
   let start: Result<u32, _> = start.trim().parse();

   println!("Enter the end of the range:");
   io::stdin().read_line(&mut end).expect("Failed to read input");
   let end: Result<u32, _> = end.trim().parse();

   match (start, end) {
       (Ok(start), Ok(end)) if start <= end => {
           let results = generate_with_check_digit(start, end);
           println!("Original Sequence\tWith Damm Check Digit");
           for result in results {
               println!("{: <16}\t\t{: >16}", result - (result % 10), result);
           }
       },
       _ => println!("Invalid range. The start should be less than or equal to the end."),
   }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damm_check_digit() {
        assert_eq!(damm_check_digit("572"), '4');
        assert_eq!(damm_check_digit("105"), '5');
    }

    #[test]
    fn test_generate_with_check_digit() {
        let result = generate_with_check_digit(1, 5);
        assert_eq!(result, vec![13, 21, 37, 45, 59]);
    }
}
