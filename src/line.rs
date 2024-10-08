use crate::{get_point, unwrap};
use crate::{
    point::Point,
    wasm4::{line, DRAW_COLORS},
};

pub type Line = (usize, usize);
pub type Lines = Vec<Line>;

pub fn init_lines(num_points: usize) -> Vec<Line> {
    let mut pz = Puzzle::new();
    for new in 3..num_points {
        let choice = pz.choice();
        let ValidPos { pos, links } = remove(&mut pz.valid_pos, choice);
        'a: for (p, cond) in pz.gen_valid_pos(&pos) {
            let link = Link { related: new, cond };
            for ValidPos { pos, links } in pz.valid_pos.iter_mut() {
                if p == *pos {
                    links.push(link);
                    continue 'a;
                }
            }
            pz.valid_pos.push(valid_pos(p, &[link]))
        }

        pz.placed_pos.push(pos);
        pz.degrees.push(0);
        let link_count = fastrand::usize(2..=3);
        for Link { related, cond } in fastrand::choose_multiple(links.into_iter(), link_count) {
            pz.lines.push((related, new));
            *unwrap(pz.degrees.get_mut(related)) += 1;
            *unwrap(pz.degrees.get_mut(new)) += 1;
            if let Some(cond) = cond {
                pz.unmed_cond.push(cond);
            }
        }
    }
    pz.lines
}

pub fn draw_lines(lines: &Lines) {
    unsafe { *DRAW_COLORS = 0x2 }
    for (i, j) in lines {
        let Point { x: x1, y: y1 } = get_point(*i);
        let Point { x: x2, y: y2 } = get_point(*j);
        line(x1 as i32, y1 as i32, x2 as i32, y2 as i32)
    }
}

pub fn check_intersect(l1: Line, l2: Line) -> bool {
    let l1 = get_endpoints(l1);
    let l2 = get_endpoints(l2);
    if !bounding_boxes_intersect(&l1, &l2) {
        return false;
    }

    let d1 = cross_product(&l1.p1, &l1.p2, &l2.p1);
    let d2 = cross_product(&l1.p1, &l1.p2, &l2.p2);
    let d3 = cross_product(&l2.p1, &l2.p2, &l1.p1);
    let d4 = cross_product(&l2.p1, &l2.p2, &l1.p2);

    if d1 * d2 < 0 && d3 * d4 < 0 {
        return true;
    }
    if d1 == 0 && is_point_on_segment(&l2.p1, &l1) {
        return true;
    }
    if d2 == 0 && is_point_on_segment(&l2.p2, &l1) {
        return true;
    }
    if d3 == 0 && is_point_on_segment(&l1.p1, &l2) {
        return true;
    }
    if d4 == 0 && is_point_on_segment(&l1.p2, &l2) {
        return true;
    }

    false
}

struct LineEndpoints {
    p1: Point,
    p2: Point,
}

fn get_endpoints(line: Line) -> LineEndpoints {
    LineEndpoints {
        p1: get_point(line.0),
        p2: get_point(line.1),
    }
}

fn cross_product(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

fn is_point_on_segment(p: &Point, s: &LineEndpoints) -> bool {
    let min_x = s.p1.x.min(s.p2.x);
    let max_x = s.p1.x.max(s.p2.x);
    let min_y = s.p1.y.min(s.p2.y);
    let max_y = s.p1.y.max(s.p2.y);
    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

fn bounding_boxes_intersect(l1: &LineEndpoints, l2: &LineEndpoints) -> bool {
    let (min_x1, max_x1) = (l1.p1.x.min(l1.p2.x), l1.p1.x.max(l1.p2.x));
    let (min_y1, max_y1) = (l1.p1.y.min(l1.p2.y), l1.p1.y.max(l1.p2.y));
    let (min_x2, max_x2) = (l2.p1.x.min(l2.p2.x), l2.p1.x.max(l2.p2.x));
    let (min_y2, max_y2) = (l2.p1.y.min(l2.p2.y), l2.p1.y.max(l2.p2.y));

    !(min_x1 > max_x2 || max_x1 < min_x2 || min_y1 > max_y2 || max_y1 < min_y2)
}

struct Puzzle {
    valid_pos: Vec<ValidPos>,
    placed_pos: Vec<P>,
    unmed_cond: Vec<P>,
    lines: Vec<Line>,
    degrees: Vec<u8>,
}

impl Puzzle {
    fn new() -> Self {
        let lines = vec![(0, 1), (0, 2), (1, 2)];
        let degrees = vec![2, 2, 2];
        let placed_pos = vec![(0, 0), (0, 2), (1, 1)];
        let unmed_cond = vec![(0, 1)];
        let valid_pos = vec![
            valid_pos((0, 4), &[link_with(2, (0, 3))]),
            valid_pos((1, 3), &[link(2), link_with(1, (1, 2))]),
            valid_pos((2, 2), &[link(1), link_with(2, (1, 2))]),
            valid_pos((3, 1), &[link_with(1, (2, 1))]),
            valid_pos((0, 2), &[link(1), link_with(0, (0, 1))]),
            valid_pos((1, -1), &[link(0), link_with(1, (0, 1))]),
            valid_pos((0, -2), &[link_with(0, (0, -1))]),
            valid_pos((-1, -1), &[link(0)]),
            valid_pos((-2, 0), &[link_with(0, (-1, 0))]),
            valid_pos((-1, 1), &[link(0), link(2)]),
            valid_pos((-2, 2), &[link_with(2, (-1, 2))]),
            valid_pos((-1, 3), &[link(2)]),
        ];
        Self {
            valid_pos,
            placed_pos,
            unmed_cond,
            lines,
            degrees,
        }
    }

    fn choice(&mut self) -> usize {
        let mut valid = Vec::new();
        for (i, ValidPos { pos: _, links }) in self.valid_pos.iter_mut().enumerate() {
            links.retain(|Link { related, cond }| {
                let mut retain = *unwrap(self.degrees.get(*related)) <= 5;
                if let Some(cond) = cond {
                    retain = retain && !self.unmed_cond.contains(cond);
                }
                retain
            });
            if links.len() >= 2 {
                valid.push(i)
            }
        }
        unwrap(fastrand::choice(valid))
    }

    fn gen_valid_pos(&self, point: &P) -> Vec<(P, Option<P>)> {
        let (x, y) = *point;
        let mut valid_pos = vec![
            ((x - 1, y - 1), None),
            ((x - 1, y + 1), None),
            ((x + 1, y - 1), None),
            ((x + 1, y + 1), None),
        ];
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let block = (x + dx, y + dy);
            if !self.unmed_cond.contains(&block) {
                valid_pos.push(((x + dx * 2, y + dy * 2), Some(block)))
            }
        }
        valid_pos.retain(|(p, _)| !self.placed_pos.contains(p));
        valid_pos
    }
}

type P = (i8, i8);

struct ValidPos {
    pos: P,
    links: Vec<Link>,
}

#[inline]
fn valid_pos(pos: P, links: &[Link]) -> ValidPos {
    ValidPos {
        pos,
        links: links.to_vec(),
    }
}

#[derive(Clone)]
struct Link {
    related: usize,
    cond: Option<P>,
}

#[inline]
fn link(related: usize) -> Link {
    Link {
        related,
        cond: None,
    }
}

#[inline]
fn link_with(related: usize, cond: P) -> Link {
    Link {
        related,
        cond: Some(cond),
    }
}

fn remove<T>(vec: &mut Vec<T>, index: usize) -> T {
    let len = vec.len();
    unsafe {
        let value = std::ptr::read(vec.as_ptr().add(index));
        let base_ptr = vec.as_mut_ptr();
        std::ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
        vec.set_len(len - 1);
        value
    }
}

pub fn title_lines() -> Lines {
    vec![
        (0, 5),
        (5, 1),
        (1, 6),
        (6, 2),
        (2, 7),
        (7, 3),
        (3, 8),
        (8, 4),
        (4, 0),
    ]
}
