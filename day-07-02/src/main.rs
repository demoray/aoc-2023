use std::{
    cmp::Ordering,
    io::{stdin, BufRead, BufReader, Read},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Card(u32);
impl From<char> for Card {
    fn from(value: char) -> Self {
        Self(match value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0,
            'T' => 10,
            _ => value.to_digit(10).unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
    High(Card),
    Pair(Card),
    TwoPair(Card, Card),
    Three(Card),
    Full(Card, Card),
    Four(Card),
    Five(Card),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum BaseKind {
    High = 1,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl BaseKind {
    fn jokers(&self, count: usize) -> Self {
        if count == 0 {
            return *self;
        }
        match (self, count) {
            (BaseKind::High, 1) => BaseKind::Pair,
            (BaseKind::High, 0) => BaseKind::High,

            (BaseKind::Pair, 2 | 1) => BaseKind::Three,

            (BaseKind::TwoPair, 2) => BaseKind::Four,
            (BaseKind::TwoPair, 1) => BaseKind::Full,
            (BaseKind::Three, 3 | 1) => BaseKind::Four,
            (BaseKind::Three, 2) => BaseKind::Five,
            (BaseKind::Full, 3 | 2) => BaseKind::Five,
            (BaseKind::Full, 1) => BaseKind::Four,
            (BaseKind::Four, 1 | 4) => BaseKind::Five,
            (BaseKind::Five, _) => BaseKind::Five,
            _ => unimplemented!("unsupported {self:?} {count}"),
        }
    }
}

impl From<Kind> for BaseKind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::High(_) => BaseKind::High,
            Kind::Pair(_) => BaseKind::Pair,
            Kind::TwoPair(_, _) => BaseKind::TwoPair,
            Kind::Three(_) => BaseKind::Three,
            Kind::Full(_, _) => BaseKind::Full,
            Kind::Four(_) => BaseKind::Four,
            Kind::Five(_) => BaseKind::Five,
        }
    }
}

impl Kind {
    fn from_hand(hand: &[char]) -> BaseKind {
        let mut cards = hand.iter().map(|x| Card::from(*x)).collect::<Vec<_>>();
        cards.sort();
        let mut counts = vec![];
        let mut current = None;
        for card in cards.clone() {
            if let Some((_, count)) = current {
                if card == current.unwrap().0 {
                    current = Some((card, count + 1));
                } else {
                    counts.push(current.unwrap());
                    current = Some((card, 1));
                }
            } else {
                current = Some((card, 1));
            }
        }
        counts.push(current.unwrap());
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let kind = match counts.as_slice() {
            [(a, 5)] => Kind::Five(*a),
            [(a, 4), (_, 1)] => Kind::Four(*a),
            [(a, 3), (b, 2)] => Kind::Full(a.clone(), b.clone()),
            [(a, 3), (_, 1), (_, 1)] => Kind::Three(*a),
            [(a, 2), (b, 2), (_, 1)] => Kind::TwoPair(*a, *b),
            [(a, 2), (_, 1), (_, 1), (_, 1)] => Kind::Pair(*a),
            _ => Kind::High(cards[4]),
        };
        let base: BaseKind = kind.into();
        let jokers = cards
            .into_iter()
            .filter(|x| x == &Card(0))
            .collect::<Vec<_>>()
            .len();
        base.jokers(jokers)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    hand: BaseKind,
    bid: usize,
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a: BaseKind = self.hand.into();
        let b: BaseKind = other.hand.into();
        println!("AA a: {a:?} - b: {b:?} {:?}", a.cmp(&b));

        match a.cmp(&b) {
            Ordering::Equal => Some(self.cards.cmp(&other.cards)),
            x => Some(x),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a: BaseKind = self.hand.into();
        let b: BaseKind = other.hand.into();

        println!(
            "BB a: {a:?} - b: {b:?} {:?} {:?} {:?} {:?}",
            a.cmp(&b),
            self.cards.cmp(&other.cards),
            self.cards,
            other.cards,
        );
        match a.cmp(&b) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl Hand {
    fn new(hand: Vec<char>, bid: usize) -> Self {
        let cards = hand.iter().map(|x| Card::from(*x)).collect::<Vec<_>>();
        Self {
            hand: Kind::from_hand(&hand),
            cards,
            bid,
        }
    }
}

fn main() {
    let result = run(stdin().lock());
    println!("{result:?}");
}

fn run<R>(handle: R) -> Option<usize>
where
    R: Read,
{
    let lines: Vec<String> = BufReader::new(handle)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let mut hands = vec![];
    for line in lines {
        let Some((hand_text, bid_text)) = line.split_once(' ') else {
            continue;
        };
        let bid = bid_text.parse().ok()?;
        hands.push(Hand::new(hand_text.chars().collect(), bid));
    }
    hands.sort();
    //println!("hands: {:#?}", hands);
    let result = hands
        .into_iter()
        .enumerate()
        .fold(0_usize, |acc, (i, hand)| {
            println!("++ {acc} {} {hand:?}", i + 1);
            acc + ((i + 1) * hand.bid)
        });

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_kind() {
        assert_eq!(Kind::from_hand(&['2', '2', '2', '2', '2']), BaseKind::Five);
        assert_eq!(Kind::from_hand(&['2', '2', '2', '2', '3']), BaseKind::Four);
        assert_eq!(Kind::from_hand(&['2', '2', '2', '3', '3']), BaseKind::Full);
        assert_eq!(Kind::from_hand(&['2', '2', '2', '3', '4']), BaseKind::Three);
        assert_eq!(
            Kind::from_hand(&['2', '2', '3', '3', '4']),
            BaseKind::TwoPair
        );
        assert_eq!(Kind::from_hand(&['2', '2', '3', '4', '5']), BaseKind::Pair);
        assert_eq!(Kind::from_hand(&['2', '3', '4', '5', '6']), BaseKind::High);
        assert_eq!(Kind::from_hand(&['A', '3', '4', '5', '6']), BaseKind::High);
    }

    #[test]
    fn test_cmp1() -> Result<(), Box<dyn std::error::Error>> {
        let a = Hand::new("77888".chars().collect(), 1);
        let b = Hand::new("77788".chars().collect(), 2);
        let c = Hand::new("67777".chars().collect(), 3);

        println!("a: {a:?}");
        println!("b: {b:?}");
        println!("c: {c:?}");
        assert!(c > a);
        assert!(c > b);
        assert!(a > b);

        Ok(())
    }

    #[test]
    fn test_cmp2() -> Result<(), Box<dyn std::error::Error>> {
        let a = Hand::new("KKTKT".chars().collect(), 1);
        let b = Hand::new("KKKTT".chars().collect(), 2);
        println!("{:?}", a.cmp(&b));
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        assert!(a < b);

        Ok(())
    }

    #[test]
    fn test_card() {
        let a = Card::from('A');
        let k = Card::from('K');
        let a_32: u32 = a.0;
        let k_32: u32 = k.0;
        println!("a: {a:?} k: {k:?}");
        println!("{:?}", a.cmp(&k));
        println!("a: {a_32} k: {k_32}");
        println!("{:?}", a_32.cmp(&k_32));
        assert!(Card::from('K') > Card::from('Q'));
        assert!(Card::from('Q') > Card::from('T'));
        assert!(Card::from('T') > Card::from('9'));
        assert!(Card::from('9') > Card::from('8'));
        assert!(Card::from('8') > Card::from('7'));
        assert!(Card::from('7') > Card::from('6'));
        assert!(Card::from('6') > Card::from('5'));
        assert!(Card::from('5') > Card::from('4'));
        assert!(Card::from('4') > Card::from('3'));
        assert!(Card::from('3') > Card::from('2'));
        assert!(Card::from('2') > Card::from('J'));
    }

    #[test]
    fn test_08() -> Result<(), Box<dyn std::error::Error>> {
        let result = run(File::open("data/example.txt")?);
        assert_eq!(result, Some(5905));
        Ok(())
    }
}
