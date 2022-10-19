//16 10 2022


/*Exercise taken from:
https://exercism.org/tracks/rust/exercises/armstrong-numbers

Instructions

An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.

For example:

    9 is an Armstrong number, because 9 = 9^1 = 9
    10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
    153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
    154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

Write some code to determine whether a number is an Armstrong number.
*/

//Step1: How to decompose a number into digits

struct Digits {
    n: usize,
    divisor: usize,
}


impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}



//Step 2: We implement the rules of the Amstrong numbers

pub fn is_armstrong_number(count: usize) -> bool {

        let digits: Vec<_> = Digits::new(count).collect();

        let filter: usize = digits.len();

        if filter == 1 {

            if count == digits[0].pow(1) {
                    true
            } else {
                    false
            }
        }

        else if filter == 2 {

            if count == digits[0].pow(2) + digits[1].pow(2) {
                     true
            } 
            else {
                    false
        }
        }

        else if filter == 3 {

            if count == digits[0].pow(3) + digits[1].pow(3) + digits[2].pow(3) {
                true
            }
            else {
                false
            }
        }

        else if filter == 4 {

            if count == digits[0].pow(4) + digits[1].pow(4) + digits[2].pow(4) + digits[3].pow(4) {
                true
            }
            else {
                false
            }
        }

        else if filter == 7 {

            if count == digits[0].pow(7) + digits[1].pow(7) + digits[2].pow(7) + digits[3].pow(7) + digits[4].pow(7) + digits[5].pow(7) + digits[6].pow(7) {
                true
            }
            else {
                false
            }
        }

        else {
            true
        }
    }

//Step 3: We have the function main and can test (here for 1000)

    fn main() {
 
        is_armstrong_number(1000);
         
    }
    
