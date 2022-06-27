use device_query::{DeviceQuery, DeviceState, Keycode};

const HEIGHT: usize = 25;
const WIDTH: usize = 17;

static SCREEN: [char; WIDTH * HEIGHT] = ['`'; WIDTH * HEIGHT];

fn main() {
    println!("Let the game begin...");

    start();
}

fn start()
{
    // here we will define the game environment
    // and how the map will be updated as the 
    // bird falls

    draw_begining();
}

fn draw_begining()
{
    let mut number: usize = 0;
    for _i in 0..HEIGHT
    {
        for _j in 0..WIDTH
        {
            match number
            {
                24 => print!("# "),

                _ => print!("{:} ", SCREEN[number]),
            }
            number += 1;
        }
        println!();
    }
}

fn input()
{
    let device_state = DeviceState::new();

    loop
    {
        let keys: Vec<Keycode> = device_state.get_keys();

        for key in keys.iter()
        {
            println!("Pressed key: {:?}", key);
        }
    }
}
