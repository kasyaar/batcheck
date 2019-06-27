extern crate battery;
use notify_rust::Notification;
use notify_rust::NotificationUrgency::*;
use battery::State::*;
fn message(msg:&str) {

    Notification::new().summary(msg)
        .action("default", "default")
        .action("clicked", "Okay!")
        .urgency(Critical)
        .show()
        .unwrap();

}

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    for bat in manager.batteries()?  {
        match bat {
            Ok(bat) => {
                match 
                    (bat.state(),
                     (bat.state_of_charge().value < 0.81),
                     (bat.state_of_charge().value > 0.85))
                     
                     {
                         (Charging, _, true)=> 

                         {
                             message("Please switch off charger!");
                             println!("Please switch off charger!")
                         },
                         (Discharging, true, _) => {
                             message("Please switch on charger!");
                             println!("Please switch on charger!")
                         },
                         _ => 
                             println!("Everything is OK!")
                     }
            },
            _ => println!("Baterry not found!")
        }
    }
    Ok(())
}
