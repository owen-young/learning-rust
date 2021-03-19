use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct Vowel(char);

impl TryFrom<char> for Vowel {
    // creating an error type to return, this has to be called "Error" for defining TryFrom
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'e' | 'i' | 'o' | 'u' => Ok(Vowel(value)),
            _ => Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(Vowel::try_from('a'), Ok(Vowel('a')));
    assert_eq!(Vowel::try_from('b'), Err(()));

    // TryInto

    let result: Result<Vowel, ()> = 'a'.try_into();
    assert_eq!(result, Ok(Vowel('a')));
    let result: Result<Vowel, ()> = 'c'.try_into();
    assert_eq!(result, Err(()));
}