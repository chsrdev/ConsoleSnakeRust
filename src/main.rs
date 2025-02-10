use std::thread;
use std::io::{Write, stdout, Stdout};
use crossterm::{cursor, queue};
use crossterm::terminal::{Clear, ClearType, size};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::{SetForegroundColor, ResetColor, Color};
use std::time::Duration;

// ^ < ˅ >
struct Direction {
    x: i8,
    y: i8,
    c: char
}

const DIRECTION_UP: Direction = Direction{x: 0, y: -1, c: '∧'};
const DIRECTION_DOWN: Direction = Direction{x: 0, y: 1, c: '˅'};
const DIRECTION_LEFT: Direction = Direction{x: -1, y: 0, c: '<'};
const DIRECTION_RIGHT: Direction = Direction{x: 1, y: 0, c: '>'};


fn main() {
    let mut stdout = stdout();
    queue!(stdout, Clear(ClearType::All)).unwrap();
    let (w, h) = size().unwrap();
    queue!(stdout, cursor::MoveTo(w/2-12, h/2)).unwrap();
    stdout.flush().unwrap();

    let start_x = w / 3;
    let start_y = h / 2;
    let mut snake = vec![(start_x, start_y)];
    for dx in 1..=5 {
        snake.push((start_x - dx, start_y));
    }
    let mut direction: Direction = DIRECTION_RIGHT;

    draw_snake(&snake, &direction,&mut stdout);

    'game_loop:
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(_, _) => break 'game_loop,
                Event::Key(event) => {
                    if let KeyCode::Char(c) = event.code {
                        match c {
                            'w' => direction = DIRECTION_UP,
                            'a' => direction = DIRECTION_LEFT,
                            's' => direction = DIRECTION_DOWN,
                            'd' => direction = DIRECTION_RIGHT,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        queue!(stdout, Clear(ClearType::All)).unwrap();

        let snake_move_result = move_snake(&mut snake, &direction, w, h);
        if snake_move_result.1 {
            draw_snake(&snake, &direction, &mut stdout);
        } else {
            queue!(stdout, SetForegroundColor(Color::Red)).unwrap();
            draw_snake(&snake_move_result.0, &direction, &mut stdout);
            queue!(stdout, ResetColor).unwrap();
            break 'game_loop;
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn is_snake_not_in_self(snake: &mut Vec<(u16, u16)>, head_x: u16, head_y: u16) -> bool {
    for i in 1..snake.len() {
        let (x, y) = snake.get(i).unwrap();
        if head_x == *x && head_y == *y {
            return false;
        }
    }
    return true;
}

fn move_snake(snake: &mut  Vec<(u16, u16)>, direction: &Direction, width: u16, height: u16) -> (Vec<(u16, u16)>, bool) {
    let snake_prev = snake.clone();
    for idx in (1..snake.len()).rev() {
        snake[idx] = *snake.get(idx-1).unwrap();
    }
    let (head_x, head_y) = snake.get(0).unwrap();
    let new_x = (*head_x as i32 + direction.x as i32) as u16;
    let new_y = (*head_y as i32 + direction.y as i32) as u16;
    let is_snake_in_bounds =
        new_x < width && new_y < height;

    if is_snake_in_bounds && is_snake_not_in_self(snake, new_x, new_y) {
        snake[0] = (new_x, new_y);
        return (snake.to_vec(), true);
    }
    return (snake_prev.to_vec(), false);
}

fn draw_snake(snake: &Vec<(u16, u16)>, direction: &Direction, stdout: &mut Stdout) {
    let (head_x, head_y) = snake.get(0).unwrap();
    queue!(*stdout, cursor::MoveTo(*head_x, *head_y)).unwrap();

    stdout.write(direction.c.to_string().as_bytes()).unwrap();

    for i in 1..snake.len() {
        let (x, y) = snake.get(i).unwrap();
        queue!(*stdout, cursor::MoveTo(*x, *y)).unwrap();
        stdout.write("#".as_bytes()).unwrap();
    }
}