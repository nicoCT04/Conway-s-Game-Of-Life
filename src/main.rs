use raylib::prelude::*;
use std::thread;
use std::time::Duration;

// Tamaño de la "pantalla"
const WIDTH: usize = 100;
const HEIGHT: usize = 100;
//Cada celula se pinta como un cuadrado de 5x5
const PIXEL_SIZE: i32 = 5;

//Define la grilla como una matriz 2d de booleanos donde true es viva y false es muerta
type Grid = [[bool; WIDTH]; HEIGHT];

//Patrones
fn glider_fn(grid: &mut Grid, x:usize, y:usize){
    grid[y][x+1] = true;
    grid[y+1][x+2] = true;
    grid[y+2][x] = true;
    grid[y+2][x+1] = true;
    grid[y+2][x+2] = true;
}

fn blinker_fn(grid: &mut Grid, x: usize, y:usize){
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
}

fn toad_fn(grid: &mut Grid, x: usize , y:usize){
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
    grid[y][x + 3] =  true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;
    grid[y + 1][x + 2] = true;
}
fn beacon_fn(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;
    grid[y + 2][x + 2] = true;
    grid[y + 2][x + 3] = true;
    grid[y + 3][x + 2] = true;
    grid[y + 3][x + 3] = true;
}

fn pulsar_fn(grid: &mut Grid, x: usize, y: usize) {
    let offsets = [
        (0, 2), (0, 3), (0, 4), (0, 8), (0, 9), (0, 10),
        (5, 2), (5, 3), (5, 4), (5, 8), (5, 9), (5, 10),
        (7, 2), (7, 3), (7, 4), (7, 8), (7, 9), (7, 10),
        (12, 2), (12, 3), (12, 4), (12, 8), (12, 9), (12, 10),
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    for (dx, dy) in offsets.iter() {
        grid[y + dy][x + dx] = true;
    }
}

fn lwss_fn(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y][x + 4] = true;
    grid[y + 1][x] = true;
    grid[y + 2][x] = true;
    grid[y + 3][x] = true;
    grid[y + 3][x + 4] = true;
    grid[y + 4][x + 1] = true;
    grid[y + 4][x + 2] = true;
    grid[y + 4][x + 3] = true;
    grid[y + 4][x + 4] = true;
}

fn boat_fn(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 2][x + 1] = true;
}

fn tub_fn(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 2][x + 1] = true;
}

fn loaf_fn(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 3] = true;
    grid[y + 2][x + 1] = true;
    grid[y + 2][x + 3] = true;
    grid[y + 3][x + 2] = true;
}

//Patron inicial 
//el &mut permite mutar a una grilla, permite modificaciones, si no se pone no se puede modificar
fn set_initial_pattern(grid: &mut Grid) {
    glider_fn(grid, 2, 2);
    blinker_fn(grid, 10, 10);
    toad_fn(grid, 20, 20);
    beacon_fn(grid, 30, 30);
    pulsar_fn(grid, 40, 40);
    lwss_fn(grid, 60, 60);
    boat_fn(grid, 70, 70);
    tub_fn(grid, 80, 80);
    loaf_fn(grid, 90, 90);
}

//main
fn main(){
    let (mut rl, thread) =raylib::init()
    .size((WIDTH as i32) * PIXEL_SIZE, (HEIGHT as i32) * PIXEL_SIZE)
    .title("DIseño de Game of life")
    .build();

    let mut grid = [[false; WIDTH]; HEIGHT]; //el mut permite modificar patrones o estados
    set_initial_pattern(&mut grid);

    while !rl.window_should_close() { //se ejecuta mientras no se cierre la ventana
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    
        //Dibujo de celdas vivas
        for y in 0.. HEIGHT {
            for x in 0..WIDTH {
                if grid[y][x]{
                    d.draw_rectangle(
                        (x as i32) * PIXEL_SIZE,
                        (y as i32) * PIXEL_SIZE,
                        PIXEL_SIZE,  //ancho del rectángulo
                        PIXEL_SIZE,  //alto del rectángulo
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
