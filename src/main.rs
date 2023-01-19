use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

use csv;
use rand::prelude::*;
use tests::{Test, TestType};

use gcd;

/// Fills a given tuple vector with random unsigned integers
fn get_rand_list(y: &mut Vec<(u32, u32)>) {
    let mut rng = rand::thread_rng();
    let len = y.len();
    y.clear();
    for _i in 0..len {
        y.push((rng.gen_range(1..10000), rng.gen_range(1..10000)));
    }
}

/// Adds the header to the records CSV
fn prepare_record_csv(wtr: &mut csv::Writer<File>) -> io::Result<()> {
    let header = &[
        "Number One",
        "Number Two",
        "Their GCD",
        "Time Spent(nanoseconds)",
    ];
    wtr.write_record(header)?;
    Ok(())
}

/// Divides the sum of the list by the length of the given list, returning its mean (average)
fn mean(list: Vec<u128>) -> f64 {
    let mut sum = 0;
    for x in list.clone() {
        sum += x
    }
    sum as f64 / (list.len() as f64)
}


/// Creates csv file(s) and runs the bruteforce v1 algorithm on the random set of data
fn run_test(t: TestType, pairs: Vec<(u32, u32)>) -> Test {
    let mut times = Vec::<u128>::new();

    let mut wtr = match t {
        TestType::BF1 => csv::Writer::from_path("./GCD_RESULTS/BF_v1_Results.csv").unwrap(),
        TestType::BF2 => csv::Writer::from_path("./GCD_RESULTS/BF_v2_Results.csv").unwrap(),
        TestType::OE => csv::Writer::from_path("./GCD_RESULTS/OE_Results.csv").unwrap(),
        TestType::SE => csv::Writer::from_path("./GCD_RESULTS/SE_Results.csv").unwrap(),
    };
    let _err = prepare_record_csv(&mut wtr);
    // Go through each pair and run it through the algorithm while timing the execution
    for x in pairs {
        let now = Instant::now();
        let gcd = match t.clone() {
            TestType::BF1 => gcd::bruteforce_v1_gcd(x.0, x.1).to_string(),
            TestType::BF2 => gcd::bruteforce_v2_gcd(x.0, x.1).to_string(),
            TestType::OE => gcd::euclid_gcd(x.0, x.1).to_string(),
            TestType::SE => gcd::euclid_v2_gcd(x.0, x.1).to_string(),
        };
        let time = now.elapsed().as_nanos();
        times.push(time);
        wtr.write_record(&[(x.0).to_string(), (x.1).to_string(), gcd, time.to_string()])
            .expect("Failed to write to result record");
    }
    let data = wtr.into_inner().unwrap();
    println!("\t{:?}", data);
    // Now we create the statistics csv
    let mut wtr2 = match t {
        TestType::BF1 => csv::Writer::from_path("./GCD_RESULTS/BF_v1_Statistics.csv").unwrap(),
        TestType::BF2 => csv::Writer::from_path("./GCD_RESULTS/BF_v2_Statistics.csv").unwrap(),
        TestType::OE => csv::Writer::from_path("./GCD_RESULTS/OE_Statistics.csv").unwrap(),
        TestType::SE => csv::Writer::from_path("./GCD_RESULTS/SE_Statistics.csv").unwrap(),
    };

    wtr2.write_record(&["Statistic", "Nanoseconds"]).expect("Could not write Header to Statistic Record");
    times.sort();
    let max = times[times.len() - 1];
    wtr2.write_record(&["Maximum Time", max.to_string().as_str()]).expect("Could not write maximum time");
    let min = times[0];
    wtr2.write_record(&["Minimum Time", min.to_string().as_str()]).expect("Could not write maximum time");
    let avg = mean(times.clone());
    wtr2.write_record(&["Average Time", avg.to_string().as_str()]).expect("Could not write mean time");
    let med = times.get(times.len() / 2).unwrap();
    wtr2.write_record(&["Median Time", med.to_string().as_str()]).expect("Could not write median time");

    let data = wtr2.into_inner().unwrap();
    println!("\t{:?}\n", data);
    Test {
        times: times.clone(),
        maximum: max,
        minimum: min,
        mean: avg,
        median: *med,
    }
}

fn main() {
    let mut pairs: Vec<(u32, u32)> = vec![(0, 0); 1000];
    get_rand_list(&mut pairs); // fills the list

    // Checks to make sure the folder GCD_RESULTS is not already created, will panic if I dont
    let x = Path::exists("./GCD_RESULTS".as_ref());

    if !x {
        println!("Creating Results Directory");
        // Creates folder for results
        std::fs::create_dir("./GCD_RESULTS").expect("Could not create directory");
    } else {
        println!("Results Directory already exists, continuing...");
    }

    // Test calls, you can see I have created a custom type to abstract my algorithm block to clean the code up a bit.
    println!("Running BruteForce algorithm V1 Test");
    let bf1 = run_test(TestType::BF1, pairs.clone());
    println!("Running BruteForce algorithm V2 Test");
    let bf2 = run_test(TestType::BF2, pairs.clone());
    println!("Running Original Euclid's algorithm Test");
    let oe = run_test(TestType::OE, pairs.clone());
    println!("Running Second Euclid's algorithm Test");
    let se = run_test(TestType::SE, pairs.clone());

    println!("Running Comparisons...");
    let mut compare = bf2.compare_to(bf1.clone());

    // Now we use the times returned from each test and we make the conclusions text file analyzing our results
    let mut file = File::create("./GCD_RESULTS/Conclusion.txt").expect("Could not create conclusion file");
    let mut string = format!("(1) Out of 1,000 pairs of integers, brute-force (v2) outperformed brute-force (v1) in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");

    compare = oe.compare_to(bf1.clone());
    string = format!("(2) Out of 1,000 pairs of integers, the original version of Euclid outperformed brute-force (v1) in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");

    compare = oe.compare_to(bf2.clone());
    string = format!("(3) Out of 1,000 pairs of integers, the original version of Euclid outperformed brute-force (v2) in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");

    compare = oe.compare_to(se.clone());
    string = format!("(4) Out of 1,000 pairs of integers, the second version of Euclid outperformed the original version of Euclid in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");

    compare = se.compare_to(bf1.clone());
    string = format!("(5) Out of 1,000 pairs of integers, the second version of Euclid outperformed brute-force (v1) in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");

    compare = se.compare_to(bf2.clone());
    string = format!("(6) Out of 1,000 pairs of integers, the second version of Euclid outperformed brute-force (v2) in {0} pairs; and the average saved time for these {0} pairs of integers was {1} nanoseconds.\n", compare.0, compare.1);
    file.write_all(string.as_bytes()).expect("Could not write to file");
    println!("\nFinished. \nPress enter to close.");

    // to keep the console window open as proof of it working
    io::stdin().read_line(&mut String::new()).unwrap();
}
