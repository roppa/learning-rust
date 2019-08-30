use std::{thread, time};

mod game;

fn main() {
    let one_second = time::Duration::from_millis(1000);
    let mut cells = vec![false; 1600];

    cells[1] = true;
    cells[2] = true;
    cells[6] = true;
    cells[30] = true;
    cells[31] = true;
    cells[26] = true;
    cells[51] = true;
    cells[52] = true;
    cells[53] = true;
    cells[54] = true;
    cells[62] = true;
    cells[63] = true;
    cells[64] = true;
    cells[125] = true;
    cells[126] = true;
    cells[145] = true;
    cells[146] = true;
    cells[167] = true;
    cells[168] = true;
    cells[187] = true;
    cells[188] = true;
    cells[221] = true;
    cells[222] = true;
    cells[223] = true;
    cells[382] = true;
    cells[403] = true;
    cells[421] = true;
    cells[422] = true;
    cells[423] = true;
    cells[1200] = true;
    cells[1201] = true;
    cells[1202] = true;
    cells[1203] = true;

    println!("{:?}", cells);
    cells = game::life::regenerate(&cells);
    println!("{:?}", cells);
    thread::sleep(one_second);
    cells = game::life::regenerate(&cells);
    println!("{:?}", cells);
    thread::sleep(one_second);
    cells = game::life::regenerate(&cells);
    println!("{:?}", cells);
    thread::sleep(one_second);
    cells = game::life::regenerate(&cells);
    println!("{:?}", cells);
}
