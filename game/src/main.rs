use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    println!("Let the game begin...");

    start();
}

fn start()
{
    // here we will define the game environment
    // and how the map will be updated as the 
    // bird falls
}

fn input()
{
    let device_state = DeviceState::new();

    loop
    {
        let keys: Vec<Keycode> = device_state.get_keys();

        for key in keys.iter()
        {
            println!("Pressed key: {:?} {:?}", key);
        }
    }
}
