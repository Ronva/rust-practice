enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Up"),
    Movement::Down => println!("Down"),
    Movement::Left => println!("Left"),
    Movement::Right => println!("right"),
  }
}

pub fn run() {
  let a1 = Movement::Left;
  let a2 = Movement::Up;
  let a3 = Movement::Down;
  let a4 = Movement::Right;

  move_avatar(a1);
  move_avatar(a2);
  move_avatar(a3);
  move_avatar(a4);
}
