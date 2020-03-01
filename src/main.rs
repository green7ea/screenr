mod display;

use std::process::Command;

use display::Display;

fn get_outputs() -> Vec<Display>
{
    let cmd = Command::new("swaymsg")
        .arg("-t")
        .arg("get_outputs")
        .output()
        .expect("Couldn't run 'swaymsg'");

    let output = std::str::from_utf8(&cmd.stdout)
        .expect("Couldn't make sense of swaymsg's anwer");
    serde_json::from_str(output).expect(":-(")
}

fn turn_off(display: &Display)
{
    println!("Turning off {}", &display.name,);
    Command::new("swaymsg")
        .arg("output")
        .arg(&display.name)
        .arg("disable")
        .output()
        .expect("Couldn't run 'swaymsg'");
}

fn turn_on(display: &Display)
{
    if display.active
    {
        return;
    }

    let best_mode = display
        .modes
        .iter()
        .max_by(|a, b| a.width.cmp(&b.width))
        .expect(&format!(
            "Couldn't determine best resolution for display {}",
            &display.name,
        ));
    println!("Turning on {} - {}", &display.name, best_mode.to_string());

    Command::new("swaymsg")
        .arg("output")
        .arg(&display.name)
        .arg("enable")
        .arg("mode")
        .arg(best_mode.to_string())
        .output()
        .expect("Couldn't run 'swaymsg'");
}

fn toggle(display: &Display)
{
    if display.active
    {
        turn_off(display);
    }
    else
    {
        turn_on(display);
    }
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mut outputs = get_outputs();
    outputs.sort();

    match args.get(1).map(|x| x.parse())
    {
        Some(Ok(num)) => outputs
            .iter()
            .skip(num)
            .take(1)
            .for_each(|output| toggle(&output)),
        _ => outputs.iter().for_each(|output| turn_on(&output)),
    };
}
