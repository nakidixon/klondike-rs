extern crate ncurses;

use crate::display::*;
use crate::display::card::*;
use crate::display::coords::*;
use crate::display::stack::*;
use crate::game::*;

static STOCK_COORDS: Coords = Coords { x: 1, y: 0 };
static TALON_COORDS: Coords = Coords { x: 12, y: 0 };
static FOUNDATION_COORDS: Coords = Coords { x: 34, y: 0 };
static TABLEAUX_COORDS: Coords = Coords { x: 1, y: 5 };

static COLUMN_OFFSET: Coords = Coords::x(3);

pub fn draw_game(
    display: &mut KlondikeDisplay,
    game: &KlondikeGame,
) {
    draw_horizontal_card_stack(display, STOCK_COORDS, &game.stock());
    draw_horizontal_card_stack(display, TALON_COORDS, &game.talon());

    for (i, (_, stack)) in game.foundation().enumerate() {
        let coords =
            FOUNDATION_COORDS
                + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
        draw_horizontal_card_stack(display, coords, &stack);
    }

    for (i, stack) in game.tableaux().enumerate() {
        let coords =
            TABLEAUX_COORDS
                + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
        draw_vertical_card_stack(display, coords, &stack);
    }
}
