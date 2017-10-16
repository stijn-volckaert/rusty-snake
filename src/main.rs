extern crate sdl;
extern crate rand;

use rand::Rng;
use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

static GRID_SIZE: i16 = 40;
static TILE_SIZE: i16 = 15;
static BLACK: sdl::video::Color = sdl::video::Color::RGB { 0: 0, 1: 0, 2: 0};
static WHITE: sdl::video::Color = sdl::video::Color::RGB { 0: 255, 1: 255, 2: 255};
static RED: sdl::video::Color = sdl::video::Color::RGB { 0: 255, 1: 0, 2: 0};

#[derive(Copy,Clone)]
struct Coords
{
    x: i16,
    y: i16
}

impl rand::Rand for Coords 
{
    fn rand<R: rand::Rng>(rng: &mut R) -> Coords
    {
        Coords { x: rng.gen_range(0, GRID_SIZE), y: rng.gen_range(0, GRID_SIZE) }    
    }
}

impl std::ops::Add for Coords
{
    type Output = Coords;

    fn add(self, other: Coords) -> Coords
    {
        let mut out = Coords { x: (self.x + other.x), y: (self.y + other.y) };
        Coords::clamp(&mut out);
        out
    }
}

impl std::cmp::PartialEq for Coords
{
    fn eq(&self, other: &Coords) -> bool
    {
        self.x == other.x && self.y == other.y        
    }
}

impl Coords
{
    pub fn clamp(c: &mut Coords) -> &Coords
    {
        if c.x < 0 {
            c.x = GRID_SIZE + c.x;
        }
        if c.x >= GRID_SIZE {
            c.x = c.x - GRID_SIZE;
        }
        if c.y < 0 {
            c.y = GRID_SIZE + c.y;
        }
        if c.y >= GRID_SIZE {
            c.y = c.y - GRID_SIZE;
        }
        c
    }

    pub fn as_sdl_rect(&self) -> sdl::Rect
    {
        sdl::Rect { x: self.x * TILE_SIZE, y: self.y * TILE_SIZE, h: TILE_SIZE as u16, w: TILE_SIZE as u16}
    }
}

fn main() 
{
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("Rusty snake", "snake");
      
    let window_size = GRID_SIZE * TILE_SIZE;
    let mut apple: Coords;
    let mut speed = Coords { x: 1, y: 0};
    let mut snake: VecDeque<Coords> = VecDeque::new();

    // get rng to generate apple coords
    let mut rng = rand::thread_rng();
    apple = rng.gen::<Coords>();
    let mut tail = apple;
    while apple.x == tail.x
    {
        tail = rng.gen::<Coords>();
    }
    snake.push_front(tail);
    snake.push_front(tail + speed);
    snake.push_front(tail + speed + speed);

    let screen = match sdl::video::set_video_mode(
        window_size as isize, window_size as isize, 32,
        &[SurfaceFlag::HWSurface],
        &[VideoFlag::DoubleBuf]) 
    {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let mut last_render = SystemTime::now();

    'main : loop {        

        // render
        if last_render + Duration::from_millis(100) < SystemTime::now()
        {
            last_render = SystemTime::now();

            // start with black screen
            screen.fill(BLACK);
            // draw snake
            for segment in &snake
            {
                screen.fill_rect(Some(segment.as_sdl_rect()), WHITE);
            }
            // draw apple
            screen.fill_rect(Some(apple.as_sdl_rect()), RED);
            screen.flip();

            // update game state
            let next_head = *snake.front().unwrap() + speed;

            if next_head == apple
            {
                if snake.len() == GRID_SIZE as usize * GRID_SIZE as usize - 1
                {
                    println!("You Win!");
                    break 'main
                }

                // grow snake
                snake.push_front(next_head);

                // generate new apple
                'gen_apple : loop
                {
                    apple = rng.gen::<Coords>();
                    if !snake.contains(&apple)
                    {
                        break 'gen_apple
                    }
                }
            }
            else 
            {
                if snake.contains(&next_head)
                {
                    println!("You Lose!");
                    break 'main
                }

                snake.pop_back();
                snake.push_front(next_head);
            }
        }

        // read input
        match sdl::event::poll_event() {
            Event::Quit => break 'main,
            Event::Key(k, _, _, _) =>
                match k {
                    Key::Escape => break 'main,
                    Key::Up => { speed = Coords {x: 0, y: -1} },
                    Key::Left => { speed = Coords {x: -1, y: 0} },
                    Key::Right => { speed = Coords {x: 1, y: 0} },
                    Key::Down => { speed = Coords {x: 0, y: 1} },
                    Key::Q => break 'main,
                    _ => {}
                }
            _ => {}
        }       
    }

    sdl::quit();
}
