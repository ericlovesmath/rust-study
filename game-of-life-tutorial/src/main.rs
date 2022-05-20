mod game;

fn main() {
    // let mut game = game::Universe::new(5, 5);
    // game.set_cells(&[(2, 1), (2, 2), (2, 3)]);

    let mut game = game::Universe::new(10, 10);
    game.set_cells(&[(3, 1), (4, 2), (4, 3), (5, 1), (5, 2)]);
    print!("{}", game);

    loop {
        game.tick();
        print!("\n{}", game);
    }
}

