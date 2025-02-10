use std::thread;
use std::io::{Write, stdout, Stdout};
use std::time::Duration;
use crossterm::{cursor, queue};
use crossterm::terminal::{Clear, ClearType, size};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::{SetForegroundColor, ResetColor, Color};
use rand::Rng;

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
    let (mut width, mut height) = size().unwrap();

    let start_x = width / 3;
    let start_y = height / 2;
    let mut snake = vec![(start_x, start_y)];

    for dx in 1..=5 { 
        snake.push((start_x - dx, start_y));
    }

    let mut direction: Direction = DIRECTION_RIGHT;
    let mut fruit = generate_fruit(&snake, width, height);
    let mut points = 0;

    queue!(stdout, Clear(ClearType::All)).unwrap();
    draw_snake(&snake, &direction, &mut stdout);
    draw_fruit(fruit, &mut stdout);

    'game_loop:
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                },
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
        write_points(width, points, &mut stdout);

        let snake_move_result = move_snake(&mut snake, &direction, width, height);
        draw_fruit(fruit, &mut stdout);
        if snake_move_result.1 {
            draw_snake(&snake, &direction, &mut stdout);
            if is_fruit_was_eaten(&snake, fruit) {
                fruit = generate_fruit(&snake, width, height);
                snake.push(*snake.get(snake.len()-1).unwrap());
                points += 1;
            }
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

fn write_points(width: u16, points: u32, stdout: &mut Stdout) {
    let label = format!("Points: {points}");
    let label_x = width as usize / 2 - label.len() / 2;
    queue!(*stdout, cursor::MoveTo(label_x as u16, 0));
    stdout.write(label.as_bytes()).unwrap();
    queue!(*stdout, cursor::MoveTo(0, 1)).unwrap();
    stdout.write("=".repeat(width as usize).as_bytes()).unwrap();
}

fn is_fruit_was_eaten(snake: &Vec<(u16, u16)>, fruit: (u16, u16)) -> bool {
    let (x, y) = snake.get(0).unwrap();
    if fruit.0 == *x && fruit.1 == *y {
        return true;
    } return false;
}

fn generate_fruit(snake: &Vec<(u16, u16)>, width: u16, height: u16) -> (u16, u16) {
    let mut rng = rand::rng();
    let mut fruit: (u16, u16) = (rng.random_range(2..width), rng.random_range(2..height));
    
    while snake.contains(&fruit) {
        fruit = (rng.random_range(3..width-1), rng.random_range(3..height));
    }
    return fruit;
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
        new_y > 1 && new_x < width && new_y < height;

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

fn draw_fruit(fruit: (u16, u16), stdout: &mut Stdout) {
    queue!(*stdout, SetForegroundColor(Color::Green), cursor::MoveTo(fruit.0, fruit.1)).unwrap();
    stdout.write("o".as_bytes()).unwrap();
    queue!(*stdout, ResetColor).unwrap();
}