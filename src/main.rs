use clap::Parser;

/// Program to conver from fahrenheit to celcius or vice versa.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Temperature unit:
    /// This can be 'fahr' to convert from fahrenheit to celcius or 'cel' to convert from celcius to fahrenheit 
    #[clap(short, long)]
    temp: String,

    /// Temperature value: This can be any floating number.
    #[clap(short, long, default_value_t = 0.0)]
    val: f32
}

fn main() {
    let args = Args::parse();

    match args.temp.as_str().trim(){
        "fahr" => { fahr_to_cel(args.val) }
        "cel" =>  { cel_to_fahr(args.val) }
        _ => { println!("Unknown argument value!") }
    }    
}

fn fahr_to_cel(fahr: f32){
    let cel = (fahr - 32.0) * 5.0/9.0;
    println!("{} Fahrenheit equals {} Celcius", fahr, cel);
}

fn cel_to_fahr(cel: f32){
    let fahr = (cel * 9.0/5.0) + 32.0;
    println!("{} Celcius equals {} Fahrenheit", cel, fahr);
}