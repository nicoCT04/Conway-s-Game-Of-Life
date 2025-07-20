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

fn beehive_fn(grid: &mut Grid, x: usize, y: usize) {
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
    //Primera fila
    glider_fn(grid, 2, 2);
    blinker_fn(grid, 10, 2);
    toad_fn(grid, 20, 2);
    beehive_fn(grid, 30, 2);
    toad_fn(grid, 40, 2);
    glider_fn(grid, 50, 2);
    toad_fn(grid, 60, 2);
    glider_fn(grid, 70, 2);
    beehive_fn(grid, 80, 2);
    toad_fn(grid, 90, 2);

    //Segunda fila
    beacon_fn(grid, 2, 20);
    boat_fn(grid, 12, 20);
    beacon_fn(grid, 20, 20);
    loaf_fn(grid, 30, 20);
    beacon_fn(grid, 40, 20);
    boat_fn(grid, 50, 20);
    beacon_fn(grid, 60, 20);
    loaf_fn(grid, 70, 20);
    tub_fn(grid, 80, 20);
    tub_fn(grid, 90, 20);


    //tercera fila
    pulsar_fn(grid, 2, 40);
    pulsar_fn(grid, 20, 40);
    pulsar_fn(grid, 40, 40);
    pulsar_fn(grid, 60, 40);
    pulsar_fn(grid, 80, 40);

    //cuarta fila
    beacon_fn(grid, 2, 60);
    boat_fn(grid, 12, 60);
    beacon_fn(grid, 20, 60);
    loaf_fn(grid, 30, 60);
    beacon_fn(grid, 40, 60);
    boat_fn(grid, 50, 60);
    beacon_fn(grid, 60, 60);
    loaf_fn(grid, 70, 60);
    tub_fn(grid, 80, 60);
    tub_fn(grid, 90, 60);

    //Quinta fila
    glider_fn(grid, 2, 70);
    blinker_fn(grid, 10, 70);
    toad_fn(grid, 20, 70);
    beehive_fn(grid, 30, 70);
    toad_fn(grid, 40, 70);
    glider_fn(grid, 50, 70);
    toad_fn(grid, 60, 70);
    glider_fn(grid, 70, 70);
    beehive_fn(grid, 80, 70);
    toad_fn(grid, 90, 70);

    //sexta fila
    pulsar_fn(grid, 2, 80);
    pulsar_fn(grid, 20, 80);
    pulsar_fn(grid, 40, 80);
    pulsar_fn(grid, 60, 80);
    pulsar_fn(grid, 80, 80);

}

//Ver cuantos vecinos vivos tiene una "Celula" hay 8 posibles por eso el u8
fn count_alive (grid: &Grid, x:usize, y: usize) -> u8 {
    let mut count = 0;
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = ((x as isize + dx + WIDTH as isize) % WIDTH as isize) as usize;
            let ny = ((y as isize + dy + HEIGHT as isize) % HEIGHT as isize) as usize;

            if grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

//Funcion que aplica la regla de conway para generarel nuevo estado
fn update_grid(current: &Grid) -> Grid {
    let mut next = [[false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = current[y][x];
            let neighbors = count_alive(current, x, y);

            next[y][x] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,    // sobrevive
                (false, 3) => true,               // nace
                _ => false,                       // muere
            };
        }
    }

    next
}

//no usamos mut en las funciones porque no estamos modificando la grilla origianl solo leyendola


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
        grid = update_grid(&grid); //actualiza el universo, admeas el & solo pasa la direccion en memeoria como un puntero seguro
        thread::sleep(Duration::from_millis(100));
    }
}
