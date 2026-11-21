use std::{
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use primitive_app::{App, Change};
use primitive_ui::{back_button, header, Canvas, Input, Rect, TextStyle, CAPTION};
use solitaire_core::{Board, Card, Destination, Source, UNDO_LIMIT};

pub struct View {
    pub board: Board,
    pub data: Option<PathBuf>,
    undo: Vec<Board>,
    selected: Option<Source>,
    notice: String,
    confirm_deal: bool,
}

pub fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

impl View {
    pub fn new(board: Board, data: Option<PathBuf>) -> Self {
        Self {
            board,
            data,
            undo: Vec::new(),
            selected: None,
            notice: String::new(),
            confirm_deal: false,
        }
    }

    pub fn load(data: PathBuf) -> Self {
        let (board, undo) = Board::restore_with_undo(&fs::read(&data).unwrap_or_default(), seed());
        let mut view = Self::new(board, Some(data));
        view.undo = undo;
        view
    }

    const STOCK: Rect = Rect::new(40, 290, 154, 190);
    const WASTE: Rect = Rect::new(212, 290, 154, 190);
    const UNDO: Rect = Rect::new(42, 1460, 310, 100);
    const FINISH: Rect = Rect::new(366, 1460, 350, 100);
    const NEW: Rect = Rect::new(730, 1460, 490, 100);
    const DRAW_MODE: Rect = Rect::new(830, 160, 390, 100);
    const SUITS: [primitive_quick::Icon; 4] = [
        primitive_quick::Icon { stroke: &[], fill: &["M12 2 C4 2 5 9 7 10 C0 8 0 19 8 18 L10 16 8 22 H16 L14 16 16 18 C24 19 24 8 17 10 C19 9 20 2 12 2 Z"] },
        primitive_quick::Icon { stroke: &[], fill: &["M12 1 L22 12 12 23 2 12 Z"] },
        primitive_quick::Icon { stroke: &[], fill: &["M12 22 C6 16 1 12 2 7 C3 1 10 2 12 6 C14 2 21 1 22 7 C23 12 18 16 12 22 Z"] },
        primitive_quick::Icon { stroke: &[], fill: &["M12 1 C8 6 1 10 2 15 C3 20 8 20 10 17 L8 23 H16 L14 17 C16 20 21 20 22 15 C23 10 16 6 12 1 Z"] },
    ];

    fn foundation(suit: usize) -> Rect {
        Rect::new(558 + 170 * suit as i32, 290, 154, 190)
    }

    fn card_rect(column: usize, row: usize, cards: usize) -> Rect {
        let step = if cards <= 1 {
            93
        } else {
            (730 / (cards - 1) as i32).min(93)
        };
        Rect::new(40 + 170 * column as i32, 530 + step * row as i32, 154, 160)
    }

    fn visible_card_rect(column: usize, row: usize, cards: usize) -> Rect {
        let mut rect = Self::card_rect(column, row, cards);
        if row + 1 < cards {
            rect.h = Self::card_rect(column, row + 1, cards).y - rect.y;
        }
        rect
    }

    fn draw_card(
        canvas: &mut Canvas,
        rect: Rect,
        card: Option<Card>,
        selected: bool,
        control: Option<Rect>,
        label: &str,
    ) -> bool {
        let pressed = control.is_some_and(|visible| canvas.control_pressed(visible, false));
        // A tap reverses the card: a selected card turns white while it is held, an unselected one turns black.
        let (paper, ink) = match (selected, pressed) {
            (true, true) | (false, false) => (255, 0),
            (true, false) | (false, true) => (0, 255),
        };
        canvas.panel(rect, 14, if selected || pressed { 5 } else { 2 }, paper);
        let text = card.map_or_else(
            || label.to_owned(),
            |card| match card.rank() {
                1 => "A".into(),
                11 => "J".into(),
                12 => "Q".into(),
                13 => "K".into(),
                rank => rank.to_string(),
            },
        );
        canvas.text(
            Rect::new(rect.x + 9, rect.y + 10, rect.w - 18, 76),
            &text,
            TextStyle::new(37).bold().ink(ink),
        );
        if let Some(card) = card {
            canvas.icon(
                Rect::new(rect.x + 82, rect.y + 16, 36, 36),
                Self::SUITS[card.suit()],
                ink,
            );
        }
        pressed
    }

    fn save_move(&mut self, previous: Board) -> Change {
        self.undo.push(previous);
        if self.undo.len() > UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.selected = None;
        self.persist();
        Change::Partial
    }

    fn persist(&mut self) {
        self.notice = self
            .save()
            .err()
            .map_or_else(String::new, |error| format!("Could not save. {error}"));
    }

    fn try_move(&mut self, target: Destination) -> bool {
        if let Some(from) = self.selected {
            let previous = self.board.clone();
            if self.board.move_cards(from, target) {
                self.save_move(previous);
                return true;
            }
            self.notice = "That move is unavailable.".into();
        }
        false
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        header(canvas, "Solitaire", "Exit");
        canvas.text(
            Rect::new(40, 185, 700, 64),
            "Klondike",
            TextStyle::new(47).bold(),
        );
        canvas.button(
            Self::DRAW_MODE,
            if self.board.draw_three {
                "Draw three"
            } else {
                "Draw one"
            },
            false,
            false,
        );
        let accepts_taps = !self.confirm_deal;
        let moving = self.selected.is_some();
        let can_draw = !self.board.stock.is_empty() || !self.board.waste.is_empty();
        Self::draw_card(
            canvas,
            Self::STOCK,
            None,
            false,
            (accepts_taps && can_draw).then_some(Self::STOCK),
            if self.board.stock.is_empty() {
                "Again"
            } else {
                "Draw"
            },
        );
        Self::draw_card(
            canvas,
            Self::WASTE,
            self.board.waste.last().copied(),
            self.selected == Some(Source::Waste),
            (accepts_taps && !self.board.waste.is_empty()).then_some(Self::WASTE),
            "",
        );
        for (suit, pile) in self.board.foundations.iter().enumerate() {
            let rect = Self::foundation(suit);
            let pressed = Self::draw_card(
                canvas,
                rect,
                pile.last().copied(),
                self.selected == Some(Source::Foundation(suit)),
                (accepts_taps && (moving || !pile.is_empty())).then_some(rect),
                "",
            );
            if pile.is_empty() {
                canvas.icon(
                    Rect::new(rect.x + 53, rect.y + 66, 48, 48),
                    Self::SUITS[suit],
                    if pressed { 255 } else { 85 },
                );
            }
        }
        for (column, cards) in self.board.columns.iter().enumerate() {
            if cards.is_empty() {
                let rect = Self::card_rect(column, 0, 0);
                Self::draw_card(
                    canvas,
                    rect,
                    None,
                    false,
                    (accepts_taps && moving).then_some(rect),
                    "",
                );
            }
            for (row, card) in cards.iter().enumerate() {
                let selected = matches!(self.selected, Some(Source::Column(c, start)) if c == column && row >= start);
                let rect = Self::card_rect(column, row, cards.len());
                let control = (accepts_taps && (moving || card.open))
                    .then(|| Self::visible_card_rect(column, row, cards.len()));
                let pressed = Self::draw_card(
                    canvas,
                    rect,
                    card.open.then_some(card.card),
                    selected,
                    control,
                    "",
                );
                if !card.open {
                    for line in 0..3 {
                        canvas.fill(
                            Rect::new(rect.x + 30, rect.y + 13 + line * 7, 94, 2),
                            if pressed { 255 } else { 85 },
                        );
                    }
                }
            }
        }
        let message = if self.board.complete() {
            "You completed the game."
        } else {
            &self.notice
        };
        canvas.text(Rect::new(40, 1390, 1180, 63), message, CAPTION);
        canvas.button(Self::UNDO, "Undo", false, false);
        canvas.button(Self::FINISH, "Auto-finish", false, false);
        canvas.button(Self::NEW, "New deal", true, false);
        if self.confirm_deal {
            canvas.popup(Rect::new(200, 570, 864, 410));
            canvas.text(
                Rect::new(245, 625, 775, 90),
                "Start a new deal?",
                TextStyle::new(50).bold(),
            );
            canvas.text(
                Rect::new(245, 735, 775, 75),
                "The current deal will be replaced.",
                CAPTION,
            );
            canvas.button(Rect::new(245, 832, 365, 100), "Cancel", false, false);
            canvas.button(Rect::new(654, 832, 365, 100), "New deal", true, false);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        let Input::Tap(x, y) = input else {
            return Change::None;
        };
        if self.confirm_deal {
            self.confirm_deal = false;
            if Rect::new(654, 832, 365, 100).contains(x, y) {
                let draw_three = self.board.draw_three;
                let previous = std::mem::replace(&mut self.board, Board::new(seed(), draw_three));
                self.save_move(previous);
            }
            return Change::Full;
        }
        if back_button(1264).contains(x, y) {
            return Change::Exit;
        }
        if Self::NEW.contains(x, y) {
            self.confirm_deal = true;
            return Change::Full;
        }
        if Self::DRAW_MODE.contains(x, y) {
            let previous = self.board.clone();
            self.board.draw_three = !self.board.draw_three;
            return self.save_move(previous);
        }
        if Self::UNDO.contains(x, y) {
            if let Some(previous) = self.undo.pop() {
                self.board = previous;
                self.selected = None;
                self.persist();
                return Change::Partial;
            }
            return Change::None;
        }
        if Self::FINISH.contains(x, y) {
            let previous = self.board.clone();
            if self.board.auto_finish() {
                return self.save_move(previous);
            }
            self.notice =
                "Auto-finish is available once the remaining cards can go to their suits.".into();
            return Change::Partial;
        }
        if Self::STOCK.contains(x, y) {
            let previous = self.board.clone();
            return if self.board.draw() {
                self.save_move(previous)
            } else {
                Change::None
            };
        }
        if Self::WASTE.contains(x, y) && !self.board.waste.is_empty() {
            self.selected = Some(Source::Waste).filter(|source| self.selected != Some(*source));
            return Change::Partial;
        }
        for suit in 0..4 {
            if Self::foundation(suit).contains(x, y) {
                if !self.try_move(Destination::Foundation(suit))
                    && !self.board.foundations[suit].is_empty()
                {
                    self.selected = Some(Source::Foundation(suit))
                        .filter(|source| self.selected != Some(*source));
                    self.notice.clear();
                }
                return Change::Partial;
            }
        }
        for column in 0..7 {
            let len = self.board.columns[column].len();
            let rect = Self::card_rect(column, 0, len);
            if x < rect.x as f32
                || x >= rect.right() as f32
                || y < rect.y as f32
                || y >= Self::card_rect(column, len.saturating_sub(1), len).bottom() as f32
            {
                continue;
            }
            if self.try_move(Destination::Column(column)) {
                return Change::Partial;
            }
            let row =
                (0..len).find(|&row| Self::visible_card_rect(column, row, len).contains(x, y));
            self.selected = row
                .filter(|row| self.board.columns[column][*row].open)
                .map(|row| Source::Column(column, row));
            self.notice.clear();
            return Change::Partial;
        }
        Change::None
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.data {
            primitive_app::save(path, &self.board.encoded_with_undo(&self.undo))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_visible_card_owns_each_touch_point_in_an_overlapping_column() {
        let covered = View::card_rect(0, 0, 2);
        let drawn_on_top = View::card_rect(0, 1, 2);
        let x = (covered.x + covered.w / 2) as f32;
        let overlap = (drawn_on_top.y + 1) as f32;
        assert!(covered.contains(x, overlap));
        assert!(!View::visible_card_rect(0, 0, 2).contains(x, overlap));
        assert!(View::visible_card_rect(0, 1, 2).contains(x, overlap));
    }

    #[test]
    fn relaunch_preserves_undo_and_saves_the_undone_deal() {
        let data = std::env::temp_dir().join(format!(
            "primitive-solitaire-deal-{}-{}.json",
            std::process::id(),
            seed()
        ));
        let board = Board::new(39, false);
        let original = board.encoded();
        let mut view = View::new(board, Some(data.clone()));
        assert_eq!(view.input(Input::Tap(90.0, 315.0)), Change::Partial);

        let mut restored = View::load(data.clone());
        assert_eq!(restored.board.waste.len(), 1);
        assert_eq!(restored.input(Input::Tap(100.0, 1490.0)), Change::Partial);
        assert_eq!(restored.board.encoded(), original);

        let mut reopened = View::load(data.clone());
        assert_eq!(reopened.board.encoded(), original);
        assert_eq!(reopened.input(Input::Tap(100.0, 1490.0)), Change::None);
        fs::remove_file(data).unwrap();
    }
}
