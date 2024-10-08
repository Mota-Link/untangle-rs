#[cfg(feature = "buddy-alloc")]
mod alloc;
mod line;
mod point;
mod resource;
mod scene;
mod wasm4;

use line::{check_intersect, init_lines, Line, Lines};
use point::{init_points, Point, Points, PointsSpeed};
use scene::{update_finish, update_playing, update_title, update_end};
use wasm4::{MOUSE_BUTTONS, MOUSE_LEFT};

use GameState::*;
use MouseButtonState::*;
use TitleState::*;

#[no_mangle]
unsafe fn update() {
    let mouse = mouse_state();
    GAME.previous_mouse = *MOUSE_BUTTONS;
    GAME.seed += 1;
    match &GAME.state {
        Title(speed) => update_title(mouse, speed),
        Playing => update_playing(mouse),
        Finish(state) => update_finish(mouse, state),
        End => update_end(),
    }
}

static mut GAME: Game = Game {
    points: Points::new(),
    lines: Lines::new(),
    previous_mouse: 0,
    holding_point: None,
    intersect_lines: None,
    state: Title(Screen(PointsSpeed::new())),
    seed: 0,
    difficulty: 6,
    metronome: 0,
};

struct Game {
    points: Points,
    lines: Lines,
    previous_mouse: u8,
    holding_point: Option<usize>,
    intersect_lines: Option<(Line, Line)>,
    state: GameState,
    seed: u64,
    difficulty: u8,
    metronome: u32,
}

unsafe fn init_game() {
    GAME.holding_point = None;
    GAME.previous_mouse = 0;
    loop {
        GAME.points = init_points(GAME.difficulty as usize, true);
        GAME.lines = init_lines(GAME.difficulty as usize);
        GAME.intersect_lines = get_intersect_lines();
        if GAME.intersect_lines.is_some() {
            break;
        }
        GAME.seed += 1;
    }
}

fn mouse_state() -> MouseButtonState {
    let previous = unsafe { GAME.previous_mouse } & MOUSE_LEFT;
    let current = unsafe { *MOUSE_BUTTONS } & MOUSE_LEFT;
    match (previous == 1, current == 1) {
        (false, false) => Idle,
        (false, true) => Pressed,
        (true, false) => Released,
        (true, true) => Held,
    }
}

#[inline]
fn get_point(idx: usize) -> Point {
    unsafe { *unwrap(GAME.points.get(idx)) }
}

fn get_intersect_lines() -> Option<(Line, Line)> {
    for l1 in unsafe { &GAME.lines } {
        for l2 in unsafe { &GAME.lines } {
            if l1.0 == l2.0 || l1.0 == l2.1 || l1.1 == l2.0 || l1.1 == l2.1 {
                continue;
            }
            if check_intersect(*l1, *l2) {
                return Some((*l1, *l2));
            }
        }
    }
    None
}

enum MouseButtonState {
    Pressed,
    Held,
    Released,
    Idle,
}

enum GameState {
    Title(TitleState),
    Playing,
    Finish(FinishState),
    End,
}

enum TitleState {
    Screen(PointsSpeed),
    Start,
}

enum FinishState {
    Twinkle(String),
    Win(String),
    Arrow(ArrowState),
    Curtain,
}

enum ArrowState {
    Appear,
    WaitClick,
}

#[inline]
fn unwrap<T>(o: Option<T>) -> T {
    match o {
        Some(t) => t,
        None => std::process::abort(),
    }
}
