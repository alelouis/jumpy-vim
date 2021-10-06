use crate::Position;

// left
pub fn vim_move_h(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let x = i32::max(p.x - 1, 0);
    let y = p.y;
    let mut r = Position { x: p.x, y: p.y };
    if w[y as usize][x as usize] != 0 {
        r.x = x;
        r.y = y;
    }
    r
}

// top
pub fn vim_move_j(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let x = p.x;
    let y = i32::max(p.y - 1, 0);
    let mut r = Position { x: p.x, y: p.y };
    if w[y as usize][x as usize] != 0 {
        r.x = x;
        r.y = y;
    }
    r
}

// bottom
pub fn vim_move_k(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let x = p.x;
    let y = i32::min(p.y + 1, w.len() as i32 - 1);
    let mut r = Position { x: p.x, y: p.y };
    if w[y as usize][x as usize] != 0 {
        r.x = x;
        r.y = y;
    }
    r
}

// right
pub fn vim_move_l(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let x = i32::min(p.x + 1, w[0].len() as i32 - 1);
    let y = p.y;
    let mut r = Position { x: p.x, y: p.y };
    if w[y as usize][x as usize] != 0 {
        r.x = x;
        r.y = y;
    }
    r
}

// begin of previous word
pub fn vim_move_w(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let mut r = Position { x: p.x, y: p.y };
    for i in (0..p.x as usize).rev() {
        if w[p.y as usize][i] == 2 {
            r.x = i as i32;
            break;
        }
    }
    r
}

// begin of next word
pub fn vim_move_b(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let mut r = Position { x: p.x, y: p.y };
    for i in (p.x + 1) as usize..w[0].len() {
        if w[p.y as usize][i] == 2 {
            r.x = i as i32;
            break;
        }
    }
    r
}

// end of next word
pub fn vim_move_e(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let mut r = Position { x: p.x, y: p.y };
    for i in (p.x + 1) as usize..w[0].len() {
        if w[p.y as usize][i] == 3 {
            r.x = i as i32;
            break;
        }
    }
    r
}
