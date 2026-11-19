use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card(pub u8);

impl Card {
    pub fn suit(self) -> usize {
        usize::from(self.0 / 13)
    }
    pub fn rank(self) -> u8 {
        self.0 % 13 + 1
    }
    pub fn red(self) -> bool {
        matches!(self.suit(), 1 | 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Waste,
    Foundation(usize),
    Column(usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Destination {
    Foundation(usize),
    Column(usize),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Tableau {
    pub card: Card,
    pub open: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: [Vec<Card>; 4],
    pub columns: [Vec<Tableau>; 7],
    pub draw_three: bool,
}

impl Board {
    pub fn new(mut seed: u64, draw_three: bool) -> Self {
        let mut stock = (0..52).map(Card).collect::<Vec<_>>();
        for i in (1..stock.len()).rev() {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            stock.swap(i, ((seed >> 32) as usize) % (i + 1));
        }
        let columns = std::array::from_fn(|column| {
            (0..=column)
                .map(|row| Tableau {
                    card: stock.pop().unwrap(),
                    open: row == column,
                })
                .collect()
        });
        Self {
            stock,
            waste: Vec::new(),
            foundations: Default::default(),
            columns,
            draw_three,
        }
    }

    pub fn encoded(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("board contains serializable cards")
    }

    pub fn restore(data: &[u8], seed: u64) -> Self {
        let decoded = serde_json::from_slice::<Self>(data)
            .ok()
            .filter(Self::valid);
        decoded.unwrap_or_else(|| Self::new(seed, false))
    }

    fn valid(&self) -> bool {
        let mut seen = [false; 52];
        let cards = self
            .stock
            .iter()
            .chain(&self.waste)
            .chain(self.foundations.iter().flatten())
            .copied()
            .chain(self.columns.iter().flatten().map(|entry| entry.card));
        for card in cards {
            let Some(entry) = seen.get_mut(usize::from(card.0)) else {
                return false;
            };
            if std::mem::replace(entry, true) {
                return false;
            }
        }
        seen.into_iter().all(|card| card)
            && self.foundations.iter().enumerate().all(|(suit, pile)| {
                pile.iter()
                    .enumerate()
                    .all(|(rank, card)| card.suit() == suit && usize::from(card.rank()) == rank + 1)
            })
            && self
                .columns
                .iter()
                .all(|column| column.last().is_none_or(|entry| entry.open))
    }

    pub fn draw(&mut self) -> bool {
        if self.stock.is_empty() {
            if self.waste.is_empty() {
                return false;
            }
            self.stock.extend(self.waste.drain(..).rev());
        } else {
            for _ in 0..if self.draw_three { 3 } else { 1 } {
                if let Some(card) = self.stock.pop() {
                    self.waste.push(card);
                }
            }
        }
        true
    }

    pub fn move_cards(&mut self, from: Source, to: Destination) -> bool {
        let mut moving = match from {
            Source::Waste => self.waste.last().copied().into_iter().collect::<Vec<_>>(),
            Source::Foundation(suit) => self
                .foundations
                .get(suit)
                .and_then(|pile| pile.last())
                .copied()
                .into_iter()
                .collect(),
            Source::Column(column, row) => {
                let Some(cards) = self
                    .columns
                    .get(column)
                    .and_then(|column| column.get(row..))
                else {
                    return false;
                };
                if cards.iter().any(|card| !card.open)
                    || cards.windows(2).any(|p| {
                        p[0].card.rank() != p[1].card.rank() + 1
                            || p[0].card.red() == p[1].card.red()
                    })
                {
                    return false;
                }
                cards.iter().map(|card| card.card).collect()
            }
        };
        let Some(&first) = moving.first() else {
            return false;
        };
        match to {
            Destination::Foundation(suit) => {
                let Some(pile) = self.foundations.get(suit) else {
                    return false;
                };
                if moving.len() != 1
                    || first.suit() != suit
                    || usize::from(first.rank()) != pile.len() + 1
                {
                    return false;
                }
                self.foundations[suit].append(&mut moving);
            }
            Destination::Column(column) => {
                if matches!(from, Source::Column(old, _) if old == column) {
                    return false;
                }
                let Some(pile) = self.columns.get_mut(column) else {
                    return false;
                };
                if !pile.last().map_or(first.rank() == 13, |last| {
                    last.card.rank() == first.rank() + 1 && last.card.red() != first.red()
                }) {
                    return false;
                }
                pile.extend(moving.into_iter().map(|card| Tableau { card, open: true }));
            }
        }
        match from {
            Source::Waste => {
                self.waste.pop();
            }
            Source::Foundation(suit) => {
                self.foundations[suit].pop();
            }
            Source::Column(column, row) => {
                self.columns[column].truncate(row);
                if let Some(top) = self.columns[column].last_mut() {
                    top.open = true;
                }
            }
        }
        true
    }

    pub fn complete(&self) -> bool {
        self.foundations.iter().all(|pile| pile.len() == 13)
    }

    pub fn auto_finish(&mut self) -> bool {
        if self.columns.iter().flatten().any(|card| !card.open) {
            return false;
        }
        let mut candidate = self.clone();
        let mut stalled_draws = 0;
        while !candidate.complete() {
            let sources = std::iter::once(Source::Waste)
                .chain((0..7).filter_map(|index| {
                    candidate.columns[index]
                        .len()
                        .checked_sub(1)
                        .map(|row| Source::Column(index, row))
                }))
                .collect::<Vec<_>>();
            let moved = sources.into_iter().any(|source| {
                (0..4).any(|suit| candidate.move_cards(source, Destination::Foundation(suit)))
            });
            if moved {
                stalled_draws = 0;
            } else {
                stalled_draws += 1;
                if stalled_draws > 53 || !candidate.draw() {
                    return false;
                }
            }
        }
        *self = candidate;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deals_a_full_deck_and_draws_three() {
        let mut game = Board::new(51, true);
        assert!(game.valid());
        assert_eq!(game.stock.len(), 24);
        for (i, cards) in game.columns.iter().enumerate() {
            assert_eq!(cards.len(), i + 1);
            assert_eq!(cards.iter().filter(|entry| entry.open).count(), 1);
        }
        assert!(game.draw());
        assert_eq!(game.waste.len(), 3);
        assert_eq!(Board::restore(&game.encoded(), 0).stock, game.stock);
    }

    #[test]
    fn moves_aces_opens_the_underlying_card_and_can_auto_finish() {
        let mut game = Board {
            stock: vec![],
            waste: vec![],
            foundations: Default::default(),
            columns: Default::default(),
            draw_three: false,
        };
        game.columns[0] = vec![
            Tableau {
                card: Card(1),
                open: false,
            },
            Tableau {
                card: Card(0),
                open: true,
            },
        ];
        assert!(!game.move_cards(Source::Column(0, 0), Destination::Foundation(0)));
        assert!(game.move_cards(Source::Column(0, 1), Destination::Foundation(0)));
        assert!(game.columns[0].last().unwrap().open);
        assert!(!game.auto_finish());
        for suit in 0..4 {
            game.foundations[suit] = (0..12).map(|rank| Card(suit as u8 * 13 + rank)).collect();
            game.columns[suit] = vec![Tableau {
                card: Card(suit as u8 * 13 + 12),
                open: true,
            }];
        }
        assert!(game.auto_finish());
        assert!(game.complete());
    }
}
