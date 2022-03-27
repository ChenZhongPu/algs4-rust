const UNICODE_SIZE: usize = 65536;

/// A data type for alphabets, for use with string-processing code.
/// `u16`, whose max is 65535, is big enough.
pub struct Alphabet {
    alphabet: Vec<char>, // the characters in the alphabet
    inverse: Vec<Option<u16>>, // indices: `index` is `char`
    r: u16, // the radix of the alphabet
}

impl Alphabet {
    /// Initializes a new alphabet from the given set of characters.
    pub fn new(alpha: String) -> Self {
        let mut unicode = vec![false; UNICODE_SIZE];
        for c in alpha.chars() {
            if unicode[c as usize] {
                panic!("Illegal alphabet: repeated character = {}", c);
            }
            unicode[c as usize] = true;
        }
        let alphabet: Vec<char> = alpha.chars().collect();
        let r = alphabet.len() as u16;
        let mut inverse: Vec<Option<u16>> = vec![None; UNICODE_SIZE];
        
        for c in 0..r {
            inverse[alphabet[c as usize] as usize] = Some(c);
        }

        Alphabet { alphabet, inverse, r }
    }

    /// Returns the number of characters in this alphabet (the radix).
    pub fn radix(&self) -> u16 {
        self.r
    }

    /// Returns the binary logarithm of the number of characters in this alphabet.
    pub fn lg_r(&self) -> u16 {
        let mut lg_r = 0;
        let mut t = self.r - 1;
        while t >= 1 {
            lg_r += 1;
            t /= 2;
        }
        lg_r
    }

    /// Returns true if the argument is a character in this alphabet.
    pub fn contains(&self, c: char) -> bool {
        self.inverse[c as usize].is_some()
    }

    /// Returns the index corresponding to the argument character.
    pub fn to_index(&self, c: char) -> u16 {
        match self.inverse[c as usize] {
            Some(i) => i,
            None => panic!("Character {} not in alphabet", c),
        }
    }
    
    /// Returns the indices corresponding to the argument characters.
    pub fn to_indices(&self, s: &str) -> Vec<u16> {
        s.chars().map(|c| self.to_index(c)).collect()
    }

    /// Returns the character corresponding to the argument index.
    pub fn to_char(&self, index: u16) -> char {
        if index > self.r {
            panic!("index must be between 0 and {}", self.r);
        }
        self.alphabet[index as usize]
    }

    /// Returns the characters corresponding to the argument indices.
    pub fn to_chars(&self, indices: Vec<u16>) -> Vec<char> {
        indices.into_iter().map(|i| self.to_char(i)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn abar() {
        let alphabet = Alphabet::new(String::from("ABCDR"));

        assert_eq!(alphabet.contains('A'), true);
        assert_eq!(alphabet.contains('a'), false);

        assert_eq!(alphabet.to_char(0), 'A');
        assert_eq!(alphabet.to_index('B'), 1);

        assert_eq!(alphabet.to_indices("AABB"), vec![0, 0, 1, 1]);
        assert_eq!(alphabet.to_chars(vec![2, 3]), vec!['C', 'D']);
    }
}