use crate::{
    get_intersect_lines, init_game,
    line::{check_intersect, draw_lines, title_lines},
    point::{draw_points, title_points, title_points_speed, DrawPointsFlag::*, Point, PointsSpeed},
    resource::*,
    unwrap,
    wasm4::{
        blit, line, rect, text, BLIT_1BPP, BLIT_2BPP, BLIT_FLIP_X, BLIT_FLIP_Y, BLIT_ROTATE,
        DRAW_COLORS, MOUSE_X, MOUSE_Y, SCREEN_SIZE,
    },
    ArrowState::{self, *},
    FinishState::{self, *},
    GameState::*,
    MouseButtonState::{self, *},
    TitleState::{self, *},
    GAME,
};

pub unsafe fn update_playing(mouse_state: MouseButtonState) {
    GAME.metronome += 1;
    let mouse = Point::new(*MOUSE_X as i32, *MOUSE_Y as i32);
    match mouse_state {
        Pressed => {
            for (i, point) in GAME.points.iter().enumerate() {
                if point.contain(mouse) {
                    GAME.holding_point = Some(i);
                    break;
                }
            }
        }
        Held => {
            if let Some(i) = GAME.holding_point {
                unwrap(GAME.points.get_mut(i)).move_to(mouse)
            }
        }
        Released => {
            if let Some(i) = GAME.holding_point.take() {
                let (l1, l2) = unwrap(GAME.intersect_lines);
                if (i == l1.0 || i == l1.1 || i == l2.0 || i == l2.1) && !check_intersect(l1, l2) {
                    GAME.intersect_lines = get_intersect_lines();
                    if GAME.intersect_lines.is_none() {
                        let min = unwrap(GAME.metronome.checked_div(3600)).to_string() + "m";
                        let sec = unwrap((GAME.metronome % 3600).checked_div(60)).to_string() + "s";
                        let time = "TIME ".to_string() + &min + " " + &sec;
                        GAME.state = Finish(Twinkle(time));
                        GAME.metronome = 0;
                    }
                }
            }
        }
        Idle => (),
    }

    let draw_points_flag = match GAME.holding_point {
        Some(i) => Hoding(i),
        None => {
            let mut flag = Normal;
            for (i, point) in GAME.points.iter().enumerate() {
                if point.contain(mouse) {
                    flag = Hover(i);
                    break;
                }
            }
            flag
        }
    };

    draw_lines(&GAME.lines);
    draw_points(&GAME.points, draw_points_flag);
}

pub unsafe fn update_title(mouse_state: MouseButtonState, title_state: &TitleState) {
    match title_state {
        Screen(speed) => update_title_screen(mouse_state, speed),
        Start => update_title_start(),
    }
}

pub unsafe fn update_finish(mouse_state: MouseButtonState, finish_state: &FinishState) {
    draw_lines(&GAME.lines);
    draw_points(&GAME.points, AllLight);
    match finish_state {
        Twinkle(time) => update_finish_twinkle(time),
        Win(time) => update_finish_curtain(time),
        Arrow(arrow_state) => update_finish_arrow(mouse_state, arrow_state),
        Curtain => update_next_level(),
    }
}

pub unsafe fn update_end() {
    thanks();
    let y = GAME.metronome as i32 - GRADIENT_H * 2 - CURTAIN_H;
    if y < 170 {
        curtain(&["The End."], y, false);
    }

    GAME.metronome += 3;
}

unsafe fn update_title_screen(mouse_state: MouseButtonState, speed: &PointsSpeed) {
    if GAME.points.len() == 0 {
        GAME.points = title_points();
        GAME.lines = title_lines();
        GAME.state = Title(Screen(title_points_speed()));
    }
    draw_lines(&GAME.lines);
    draw_points(&GAME.points, AllLight);

    let speed = speed
        .iter()
        .enumerate()
        .map(|(i, (move_x, move_y))| {
            let Point { x, y } = unwrap(GAME.points.get_mut(i));
            *x = (*x + *move_x as i32).max(1).min(158);
            let new_move_x = match x {
                1 | 158 => move_x * -1,
                _ => *move_x,
            };
            *y = (*y + *move_y as i32).max(1).min(158);
            let new_move_y = match y {
                1 | 158 => move_y * -1,
                _ => *move_y,
            };
            (new_move_x, new_move_y)
        })
        .collect::<Vec<_>>();

    if speed.len() != 0 {
        GAME.state = Title(Screen(speed));
    }

    if matches!(mouse_state, Released) {
        GAME.state = Title(Start);
        GAME.metronome = 90;
    };

    title();
}

unsafe fn update_title_start() {
    draw_lines(&GAME.lines);
    draw_points(&GAME.points, Normal);
    if GAME.metronome < 168 {
        *DRAW_COLORS = 0x33;
        rect(0, 0, 160, 160);
    }
    update_next_level()
}

unsafe fn update_finish_twinkle(time: &str) {
    match GAME.metronome {
        ..5 | 15..20 => draw_points(&GAME.points, AllBigLight),
        5..10 | 20..25 => draw_points(&GAME.points, AllBig),
        10..15 => (),
        25.. => {
            GAME.state = Finish(Win(time.to_string()));
            GAME.metronome = 0;
        }
    }
    GAME.metronome += 1;
}

unsafe fn update_finish_curtain(time: &str) {
    let y = GAME.metronome as i32 - GRADIENT_H * 2 - CURTAIN_H;
    curtain(&["Win!", &time], y, true);
    GAME.metronome += 3;
    if y + 3 > SCREEN_SIZE as i32 {
        GAME.state = Finish(Arrow(Appear));
        GAME.metronome = 0;
    }
}

unsafe fn update_finish_arrow(mouse_state: MouseButtonState, arrow_state: &ArrowState) {
    let x = 160 - GAME.metronome as i32;
    match arrow_state {
        Appear => {
            if x > 113 {
                GAME.metronome = std::cmp::min(160 - 113, GAME.metronome + 3)
            } else {
                GAME.state = Finish(Arrow(WaitClick))
            }
        }
        WaitClick => {
            if matches!(*MOUSE_X, 96..=160) && matches!(*MOUSE_Y, 126..150) {
                if x > 97 {
                    GAME.metronome += 2;
                }
                match mouse_state {
                    Pressed => GAME.holding_point = Some(0),
                    Released if GAME.holding_point.is_some() => {
                        GAME.state = Finish(Curtain);
                        GAME.metronome = 0;
                        return;
                    }
                    _ => (),
                }
            } else if x < 113 {
                GAME.metronome -= 2;
            }
        }
    }
    arrow(x);
}

unsafe fn update_next_level() {
    let y = GAME.metronome as i32 - GRADIENT_H * 2 - CURTAIN_H;
    let level = unwrap((GAME.difficulty - 6).checked_div(3));
    if level == 7 {
        curtain(&["The End."], y, false);
        if y >= -130 {
            GAME.state = End;
            return;
        }
        GAME.metronome += 3;
        return;
    }

    match y {
        -83..-80 => init_game(),
        0..160 => draw_points(&GAME.points, Normal),
        160.. => {
            draw_points(&GAME.points, Normal);
            GAME.state = Playing;
            GAME.metronome = 0;
            GAME.difficulty += 3;
        }
        _ => (),
    }
    let level = "LEVEL ".to_string() + &level.to_string();
    let points = GAME.difficulty.to_string() + " Points";
    curtain(&[&level, &points], y, false);
    GAME.metronome += 3;
}

unsafe fn title() {
    GAME.metronome += 1;
    *DRAW_COLORS = 0x33;
    rect(0, 50, 160, 28);
    *DRAW_COLORS = 0x1;
    text("Untangle", 48, 60);

    if unwrap(GAME.metronome.checked_div(40)) % 2 == 0 {
        *DRAW_COLORS = 0x11;
        rect(30, 118, 100, 12);
        *DRAW_COLORS = 0x3;
        text("Tap to Start", 32, 120);
    }
}

pub unsafe fn gradient(colors: u16, y: i32) {
    *DRAW_COLORS = colors;
    for offset in 0..40 {
        blit(
            &GRADIENT,
            GRADIENT_W * offset,
            y,
            GRADIENT_H as u32,
            GRADIENT_W as u32,
            BLIT_ROTATE,
        )
    }
}

unsafe fn curtain(texts: &[&str], y: i32, is_white: bool) {
    let colors = if is_white { 0x10 } else { 0x30 };
    gradient(colors, y);
    let colors = if is_white { 0x11 } else { 0x33 };
    *DRAW_COLORS = colors;
    rect(0, y + GRADIENT_H, SCREEN_SIZE, CURTAIN_H as u32);
    let colors = if is_white { 0x1 } else { 0x3 };
    gradient(colors, y + GRADIENT_H + CURTAIN_H);

    let texts_height = 20 * texts.len() as i32 - 12;
    let first_text_y = unwrap((SCREEN_SIZE as i32 - texts_height).checked_div(2)) - 4;
    for (i, txt) in texts.iter().enumerate() {
        let txt_offset = first_text_y + i as i32 * 20;
        let colors = if is_white { 0x3 } else { 0x1 };
        curtain_text(colors, txt, y + i as i32 * 24, txt_offset);
    }
}

unsafe fn curtain_text(colors: u16, txt: &str, curtain_y: i32, y_offset: i32) {
    let y = unwrap((curtain_y + 106).pow(3).checked_div(30000)) + y_offset;
    if -8 < y && y < SCREEN_SIZE as i32 {
        let x = unwrap((SCREEN_SIZE as i32 - txt.len() as i32 * 8).checked_div(2));
        *DRAW_COLORS = colors;
        text(txt, x, y);
    }
}

pub unsafe fn arrow(x: i32) {
    let y = 130;

    *DRAW_COLORS = 0x13;
    let ah = ARROW_HEAD;
    let f = BLIT_2BPP;
    blit(&ah, x + 5, y, ARROW_HEAD_W, ARROW_HEAD_H, f);
    blit(&ah, x + 1, y + 4, ARROW_HEAD_W, ARROW_HEAD_H, f);
    let f = BLIT_2BPP | BLIT_FLIP_Y;
    blit(&ah, x + 1, y + 9, ARROW_HEAD_W, ARROW_HEAD_H, f);
    blit(&ah, x + 5, y + 13, ARROW_HEAD_W, ARROW_HEAD_H, f);

    *DRAW_COLORS = 0x1;
    line(x, y + 8, x + 2, y + 8);
    *DRAW_COLORS = 0x3;
    line(x + 2, y + 8, x + 5, y + 8);

    *DRAW_COLORS = 0x11;
    rect(x + 10, y + 3, 54, 11);
    *DRAW_COLORS = 0x33;
    rect(x + 6, y + 4, 42, 9);

    *DRAW_COLORS = 0x3;
    let at = ARROW_TAIL;
    let f = BLIT_1BPP;
    blit(&at, x + 50, y + 4, ARROW_TAIL_W, ARROW_TAIL_H, f);
    line(x + 50, y + 12, x + 54, y + 12);
    let f = BLIT_1BPP | BLIT_FLIP_Y | BLIT_FLIP_X;
    blit(&at, x + 53, y + 5, ARROW_TAIL_W, ARROW_TAIL_H, f);
    line(x + 52, y + 4, x + 56, y + 4);
    let f = BLIT_1BPP | BLIT_FLIP_Y;
    blit(&at, x + 58, y + 5, ARROW_TAIL_W, ARROW_TAIL_H, f);
    line(x + 58, y + 4, x + 62, y + 4);

    *DRAW_COLORS = 0x1;
    blit(&C, x + 8, y + 5, CHAR_W, CHAR_H, 0);
    blit(&O, x + 13, y + 5, CHAR_W, CHAR_H, 0);
    blit(&N, x + 18, y + 5, CHAR_W, CHAR_H, 0);
    blit(&T, x + 23, y + 5, CHAR_W, CHAR_H, 0);
    blit(&I, x + 27, y + 5, CHAR_W, CHAR_H, 0);
    blit(&N, x + 31, y + 5, CHAR_W, CHAR_H, 0);
    blit(&U, x + 36, y + 5, CHAR_W, CHAR_H, 0);
    blit(&E, x + 41, y + 5, CHAR_W, CHAR_H, 0);
}

unsafe fn thanks() {
    *DRAW_COLORS = 0x4321;
    blit(&MOTA, 59, 30, MOTA_W, MOTA_H, BLIT_2BPP | BLIT_ROTATE);
    *DRAW_COLORS = 0x3;
    text("Thank you", 44, 95);
    text("for", 70, 110);
    text("playing my game!", 19, 125);
}
