#![feature(test)]
extern crate test;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use rayon::prelude::*;

#[derive(Debug)]

struct TemperatureAgg(f32, f32, i64, f32);
static FILE_PATH_TEST: &str = "/Users/walid/Lab/onebrc-rust/data/weather_stations_test.csv";
static FILE_PATH: &str = "/Users/walid/Lab/onebrc-rust/data/weather_stations.csv";
static FILE_PATH_FULL: &str = "/Users/walid/Lab/onebrc-rust/data/weather_stations_full.csv";
fn main() {


    let file_path = std::env::args().nth(1).expect("no path given");
    let acc = compute_agg(file_path.as_str());
    print!("Tokyo : {:?}", acc["Tokyo"])
}

fn compute_agg(file_path: &str) -> HashMap<String, TemperatureAgg> {
    
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);


    let acc: HashMap<String, TemperatureAgg> = reader
        .lines()
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<String, TemperatureAgg> , value| {
            if value.is_ok() {
                let line = value.unwrap();
                let parsed_line = line.split(";").collect::<Vec<&str>>();
                let (city, temperature) = (parsed_line[0].to_owned(), parsed_line[1].parse::<f32>().unwrap().to_owned());
                acc.entry(city)
                .and_modify(|agg: &mut TemperatureAgg| {
                    (*agg).0  =agg.0.min(temperature);
                    (*agg).1  =agg.1 + temperature;
                    (*agg).2  += 1;
                    (*agg).3  =agg.3.max(temperature)
                })
                .or_insert(TemperatureAgg(temperature, temperature, 1, temperature));
            }
            acc
        });
        return acc
}

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn basic_implemntation_test(b: &mut Bencher) {
        b.iter(|| compute_agg(FILE_PATH_TEST));
    }
    #[bench]
    fn basic_implemntation_light(b: &mut Bencher) {
        b.iter(|| compute_agg(FILE_PATH));
    }
    #[bench]
    fn basic_implemntation_full(b: &mut Bencher) {
        b.iter(|| compute_agg(FILE_PATH));
    }
}