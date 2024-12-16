use crate::{
    get_intersect_lines, init_game,
    line::{check_intersect, draw_lines, title_lines},
    point::{draw_points, title_points, title_points_speed, DrawPointsFlag::*, Point, PointsSpeed},
    resource::*,
    unwrap,
    wasm4::{
        blit, diskr, diskw, hline, line, rect, text, BLIT_1BPP, BLIT_2BPP, BLIT_FLIP_X,
        BLIT_FLIP_Y, BLIT_ROTATE, DRAW_COLORS, MOUSE_X, MOUSE_Y, SCREEN_SIZE,
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
        Some(i) => {
            let adjacent = GAME
                .lines
                .iter()
                .filter_map(|(a, b)| {
                    if *a == i {
                        Some(*b)
                    } else if *b == i {
                        Some(*a)
                    } else {
                        None
                    }
                })
                .collect();
            Hoding(i, adjacent)
        }
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
        Level(speed) => update_level_select(mouse_state, speed),
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
    let speed = title_animation(speed);
    title();

    if matches!(mouse_state, Released) {
        GAME.state = Title(Level(speed));
        return;
    };

    if speed.len() != 0 {
        GAME.state = Title(Screen(speed));
    }
}

unsafe fn update_level_select(mouse_state: MouseButtonState, speed: &PointsSpeed) {
    if GAME.points.len() == 0 {
        GAME.points = title_points();
        GAME.lines = title_lines();
        GAME.state = Title(Level(title_points_speed()));
    }
    let speed = title_animation(speed);
    if speed.len() != 0 {
        GAME.state = Title(Level(speed));
    }

    if GAME.unlock_level.is_none() {
        let mut buffer = [0u8; 1];
        diskr(buffer.as_mut_ptr(), 1);
        GAME.unlock_level = Some(u8::from_le_bytes(buffer).min(LEVEL_POINTS.len() as u8 - 1));
    }
    let unlock = unwrap(GAME.unlock_level);

    *DRAW_COLORS = 0x33;
    rect(0, 20, 160, 28);
    *DRAW_COLORS = 0x1;
    text("Select Level", 32, 30);

    button(7, 65, Some(0), &mouse_state);
    button(45, 65, (1 <= unlock).then_some(1), &mouse_state);
    button(83, 65, (2 <= unlock).then_some(2), &mouse_state);
    button(121, 65, (3 <= unlock).then_some(3), &mouse_state);
    button(26, 87, (4 <= unlock).then_some(4), &mouse_state);
    button(64, 87, (5 <= unlock).then_some(5), &mouse_state);
    button(102, 87, (6 <= unlock).then_some(6), &mouse_state);
    button(45, 109, (7 <= unlock).then_some(7), &mouse_state);
    button(83, 109, (8 <= unlock).then_some(8), &mouse_state);
    button(64, 131, (9 <= unlock).then_some(9), &mouse_state);

    if matches!(mouse_state, Released) {
        GAME.holding_point = None;
    }
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
    if GAME.current_level > unwrap(GAME.unlock_level) {
        diskw(GAME.current_level.to_le_bytes().as_ptr(), 1);
        GAME.unlock_level = Some(GAME.current_level);
    }
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
    if GAME.current_level == 10 {
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
            GAME.current_level += 1;
        }
        _ => (),
    }
    let level = "LEVEL ".to_string() + &GAME.current_level.to_string();
    let points = level_points_num().to_string() + " Points";
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

unsafe fn title_animation(speed: &PointsSpeed) -> PointsSpeed {
    draw_lines(&GAME.lines);
    draw_points(&GAME.points, AllLight);
    speed
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
        .collect::<Vec<_>>()
}

pub unsafe fn level_points_num() -> u8 {
    let level = (GAME.current_level as usize).min(LEVEL_POINTS.len() - 1);
    *unwrap(LEVEL_POINTS.get(level))
}

unsafe fn button(x: i32, y: i32, level: Option<usize>, mouse_state: &MouseButtonState) {
    let contains_x = *MOUSE_X as i32 >= x && (*MOUSE_X as i32) < x + BUTTON_W;
    let contains_y = *MOUSE_Y as i32 >= y && (*MOUSE_Y as i32) < y + BUTTON_H;
    let (stroke, fill, txt) = match (level, mouse_state) {
        (None, _) => (0x3, 0x3, 0x4),
        (_, Pressed) if contains_x && contains_y => {
            GAME.holding_point = level;
            (0x3, 0x3, 0x1)
        }
        (_, Released) if GAME.holding_point == level && contains_x && contains_y => {
            GAME.current_level = unwrap(level) as u8;
            GAME.state = Title(Start);
            GAME.metronome = 90;
            (0x3, 0x3, 0x1)
        }
        (_, Held) if GAME.holding_point == level => (0x3, 0x3, 0x1),
        (_, Idle) if contains_x && contains_y => (0x3, 0x2, 0x3),
        _ => (0x3, 0x1, 0x3),
    };

    *DRAW_COLORS = 0x1;
    rect(x - 1, y - 1, BUTTON_W as u32 + 2, BUTTON_H as u32 + 2);
    *DRAW_COLORS = stroke << 4 | fill;
    rect(x, y, BUTTON_W as u32, BUTTON_H as u32);
    *DRAW_COLORS = 0x1;
    rect(x, y, 2, 2);
    rect(x - 2 + BUTTON_W, y, 2, 2);
    rect(x, y - 2 + BUTTON_H, 2, 2);
    rect(x - 2 + BUTTON_W, y - 2 + BUTTON_H, 2, 2);
    *DRAW_COLORS = stroke;
    hline(x + 1, y + 1, 1);
    hline(x - 2 + BUTTON_W, y + 1, 1);
    hline(x + 1, y - 2 + BUTTON_H, 1);
    hline(x - 2 + BUTTON_W, y - 2 + BUTTON_H, 1);

    let cx = x + unwrap((BUTTON_W - 8).checked_div(2));
    let cy = y + unwrap((BUTTON_H - 8).checked_div(2));
    *DRAW_COLORS = txt;
    match level {
        Some(l) => text(l.to_string(), cx, cy),
        None => blit(&LOCK, cx, cy, LOCK_W, LOCK_H, 0),
    };
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
