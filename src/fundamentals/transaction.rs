//! # Data type for commercial transactions.
use chrono::NaiveDate;
use std::{cmp::PartialEq, cmp::PartialOrd, fmt::Display};

#[derive(Debug)]
pub struct Transaction {
    who: String,
    when: NaiveDate,
    amount: f64,
}

impl Transaction {
    pub fn new(who: &str, when: NaiveDate, amount: f64) -> Self {
        Transaction {
            who: String::from(who),
            when,
            amount,
        }
    }

    pub fn from(transaction: &str) -> Self {
        let mut a = transaction.split_whitespace();
        let who = a.next().unwrap();
        let when = NaiveDate::parse_from_str(a.next().unwrap(), "%m/%d/%Y").unwrap();
        let amount = a.next().unwrap().parse::<f64>().unwrap();
        Transaction::new(who, when, amount)
    }

    pub fn who(&self) -> &str {
        self.who.as_str()
    }

    pub fn when(&self) -> &NaiveDate {
        &self.when
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<10}{:>10}{:>8.2}", self.who, self.when, self.amount)
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let t1 = Transaction::from("Turing 6/17/1990 644.08");
        let t2 = Transaction::from("Dijkstra 1/11/1991 4409.74");
        assert!(t1 < t2);
    }

    #[test]
    fn sort() {
        let mut v = vec![
            Transaction::from("Turing   6/17/1990  644.08"),
            Transaction::from("Tarjan   3/26/2002 4121.85"),
            Transaction::from("Knuth    6/14/1999  288.34"),
            Transaction::from("Dijkstra 8/22/2007 2678.40"),
        ];
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            v,
            vec![
                Transaction::from("Knuth    6/14/1999  288.34"),
                Transaction::from("Turing   6/17/1990  644.08"),
                Transaction::from("Dijkstra 8/22/2007 2678.40"),
                Transaction::from("Tarjan   3/26/2002 4121.85"),
            ]
        );
    }

    #[test]
    fn sort_by_when() {
        let mut v = vec![
            Transaction::from("Turing   6/17/1990  644.08"),
            Transaction::from("Tarjan   3/26/2002 4121.85"),
            Transaction::from("Knuth    6/14/1999  288.34"),
            Transaction::from("Dijkstra 8/22/2007 2678.40"),
        ];

        v.sort_by_key(|t| t.when);
        assert_eq!(
            v,
            vec![
                Transaction::from("Turing   6/17/1990  644.08"),
                Transaction::from("Knuth    6/14/1999  288.34"),
                Transaction::from("Tarjan   3/26/2002 4121.85"),
                Transaction::from("Dijkstra 8/22/2007 2678.40"),
            ]
        );
    }
}
