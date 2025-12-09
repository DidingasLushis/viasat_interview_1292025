// TODO: Combine your knowledge from all previous exercises to solve this challenge.
//
// Create a function that processes a list of transactions and returns a summary.
// Each transaction is a string in the format "TYPE AMOUNT" where:
// - TYPE is either "DEPOSIT" or "WITHDRAW"
// - AMOUNT is a positive integer
//
// Return a Result containing the final balance, or an error if:
// - The format is invalid
// - A withdrawal would result in a negative balance

fn process_transactions(transactions: &[&str]) -> Result<i32, String> {
    // TODO: Implement this function
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transactions() {
        let transactions = vec!["DEPOSIT 100", "WITHDRAW 30", "DEPOSIT 50"];
        assert_eq!(process_transactions(&transactions), Ok(120));
    }

    #[test]
    fn test_insufficient_funds() {
        let transactions = vec!["DEPOSIT 50", "WITHDRAW 100"];
        assert!(process_transactions(&transactions).is_err());
    }

    #[test]
    fn test_invalid_format() {
        let transactions = vec!["DEPOSIT 50", "INVALID"];
        assert!(process_transactions(&transactions).is_err());
    }

    #[test]
    fn test_empty() {
        let transactions = vec![];
        assert_eq!(process_transactions(&transactions), Ok(0));
    }
}
