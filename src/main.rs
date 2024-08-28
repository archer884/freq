use clap::Parser;
use either::Either;
use frequency::Frequency;
use wavelength::Meters;

mod error;
mod frequency;
mod wavelength;

/// ...as in E = MC^2 -- the speed of light
const C: f64 = 299_792_458.0;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_parser = parse_arg)]
    value: Either<Meters, Frequency>,
}

fn main() {
    let Args { value } = Args::parse();

    match value {
        Either::Left(wavelength) => convert_wavelength(wavelength),
        Either::Right(frequency) => convert_frequency(frequency),
    }
}

fn convert_frequency(f: Frequency) {
    println!("{}", f.to_wavelength());
}

fn convert_wavelength(w: Meters) {
    println!("{}", w.to_frequency());
}

fn parse_arg(s: &str) -> Result<Either<Meters, Frequency>, &'static str> {
    let (left, right): (Result<Meters, _>, Result<Frequency, _>) = (s.parse(), s.parse());
    match (left, right) {
        (Ok(meters), _) => Ok(Either::Left(meters)),
        (_, Ok(frequency)) => Ok(Either::Right(frequency)),
        _ => Err("enter a value in meters or Hertz, with suffix m, cm, MHz, KHz, or Hz"),
    }
}
