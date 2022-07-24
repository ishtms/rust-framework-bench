use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write,
    fs,
    process::{Child, Command, Output},
    time::Duration,
};

macro_rules! infer_len_slice {
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident $name:ident: [$ty:ty; _] = $value:expr
    ) => {
        $( #[$attr] )*
        $v $id $name: [$ty; $value.len()] = $value;
    }
}

#[cfg(not(debug_assertions))]
static FRAMEWORK_SETTINGS: &str = include_str!("./utils/config.json");
#[cfg(debug_assertions)]
static FRAMEWORK_SETTINGS: &str = include_str!("./utils/config.dev.json");

#[cfg(not(debug_assertions))]
static OUTPUT_MD_FILE: &str = "./readme.md";
#[cfg(debug_assertions)]
static OUTPUT_MD_FILE: &str = "./readme.dev.md";

#[cfg(not(debug_assertions))]
const DEFAULT_DURATION: u32 = 40_u32;
#[cfg(debug_assertions)]
const DEFAULT_DURATION: u32 = 5_u32;

static TABLE_SEPARATOR: &str = "|:-------------------:|:---------------------:|:--------------:|\n";

static READ_ME_STRING: &str = include_str!("./utils/readme_block.txt");

infer_len_slice !(static BENCHMARK_SETTINGS: [Settings; _] = [
    #[cfg(not(debug_assertions))]
    Settings {
        concurrency: 16,
        threads: 2,
        duration: DEFAULT_DURATION,
    },
    #[cfg(not(debug_assertions))]
    Settings {
        concurrency: 64,
        threads: 2,
        duration: DEFAULT_DURATION,
    },
    Settings {
        concurrency: 128,
        threads: 2,
        duration: DEFAULT_DURATION,
    },
    Settings {
        concurrency: 256,
        threads: 2,
        duration: DEFAULT_DURATION,
    },
]);

struct Settings {
    concurrency: u32,
    threads: u8,
    duration: u32, // seconds
}

#[derive(Debug, Clone)]
struct Stats {
    requests_per_second: f64,
    name: String,
    average_latency: String,
    max_latency: String,
    total_requests: u64,
    concurrency: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Framework {
    name: &'static str,
    port: u32,
    binary: &'static str,
    url: &'static str,
    description: &'static str,
}

impl Framework {
    fn print_log(&self, settings: &Settings, framework_index: usize, bench_index: usize) {
        print_current_info(
            framework_index,
            bench_index,
            settings,
            &self.name.to_string(),
        );
        show_progress_bar(settings);
    }

    #[must_use = "Require handle to kill it once the benchmark finishes"]
    fn start_server(&self) -> Child {
        match Command::new(format!("target/release/{}", self.binary))
            .arg(format!("{}", self.port))
            .spawn()
        {
            Ok(handle) => handle,
            Err(err_message) => panic!("{}", err_message.to_string()),
        }
    }

    async fn run_benchmark(&self, framework_index: usize) {
        // start the framework's server
        let mut server_handle = self.start_server();

        // kill server if there's an error while creating a directory
        if let Err(err_message) = fs::create_dir_all(format!("perf/{}", self.binary)) {
            server_handle.kill().unwrap();
            println!(
                "\n[ERROR] Couldn't create directory (perf/{}) : {}",
                self.name, err_message
            );
            std::process::exit(-1);
        };
        BENCHMARK_SETTINGS
            .iter()
            .enumerate()
            .for_each(|(bench_index, setting)| {
                self.print_log(setting, framework_index, bench_index);
                // wait 2 secs till the server starts running (some servers take more time to start - for example tide)
                std::thread::sleep(Duration::from_secs(1));
                let rewrk_output = start_bench(setting, self.port);

                if !rewrk_output.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&rewrk_output.stderr));
                }

                // kill server if there's an error while writing `rewrk` output to the file
                let file_name = format!("perf/{}/{}.txt", self.binary, setting.concurrency);
                if let Err(err_message) = fs::write(file_name, rewrk_output.stdout) {
                    server_handle.kill().unwrap();
                    println!("\n[ERROR] Couldn't write to file: {}", err_message);
                    std::process::exit(-1);
                }

                // wait a bit to free system resources
                std::thread::sleep(Duration::from_secs(1));
            });

        if let Err(err_message) = server_handle.kill() {
            println!("\nFailed to kill {} server.\n{}", self.name, err_message);
            std::process::exit(-1);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut frameworks = parse_frameworks();
    print_benchmark_message();
    print_expected_time(frameworks.len());

    for (index, current_framework) in frameworks.iter().enumerate() {
        current_framework.run_benchmark(index).await;
    }

    let sorted_frameworks = sort_framework(&mut frameworks);
    write_markdown(&sorted_frameworks);
    write_readme(&frameworks);

    // optional - disable it if you've not forked yet
    // disable auto commit while running in debug mode
    if !cfg!(debug_assertions) {
        commit_and_push();
    }
}

fn write_readme(frameworks: &Vec<Framework>) {
    let split_string: Vec<&str> = READ_ME_STRING.split("==SPLIT==").collect();
    let mut markdown_content = String::new();

    writeln!(&mut markdown_content, "## Frameworks included").unwrap();
    for framework in frameworks {
        writeln!(
            &mut markdown_content,
            "**[{}]({})** - {}<br>",
            framework.name, framework.url, framework.description
        )
        .unwrap();
    }
    writeln!(&mut markdown_content, "# Results").unwrap();
    BENCHMARK_SETTINGS.iter().for_each(|curr| {
        let file_path = format!("results/concurrency-{}.md", curr.concurrency);
        let current_result = fs::read_to_string(file_path).unwrap();

        writeln!(
            &mut markdown_content,
            "|   Concurrency: {}   |   Duration: {} secs   |   Threads: {}   |",
            curr.concurrency, curr.duration, curr.threads
        )
        .unwrap();
        writeln!(&mut markdown_content, "{}", TABLE_SEPARATOR).unwrap();
        writeln!(&mut markdown_content, "{}\n", current_result).unwrap();
    });

    let new_md = format!(
        "{}\n{}\n{}",
        split_string[0], markdown_content, split_string[1]
    );
    fs::write(OUTPUT_MD_FILE, new_md).unwrap();
}

fn commit_and_push() {
    let add_output = Command::new("git").arg("add").arg(".").output().unwrap();
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-am 'add new results [MD]'")
        .output()
        .unwrap();
    let push_output = Command::new("git").arg("push").output().unwrap();
    println!(
        "{}\n\n{}\n{}",
        String::from_utf8_lossy(&add_output.stdout),
        String::from_utf8_lossy(&commit_output.stdout),
        String::from_utf8_lossy(&push_output.stdout),
    )
}

fn write_markdown(sorted_frameworks: &[Vec<Stats>]) {
    for batches in sorted_frameworks.iter() {
        let concurrency = batches[0].concurrency.to_string();
        let mut markdown_string = String::new();
        writeln!(&mut markdown_string, "{}", MARKDOWN_HEADER).unwrap();

        for framework_stat in batches {
            writeln!(
                &mut markdown_string,
                "|**{}** {}|{}|{}|{}|{}|",
                framework_stat.name,
                if framework_stat.average_latency == "99999" {
                    "(FAIL)"
                } else {
                    ""
                },
                (framework_stat.requests_per_second as u64).to_formatted_string(&Locale::en),
                if framework_stat.average_latency == "99999" {
                    "FAIL".to_string()
                } else {
                    framework_stat.average_latency.to_string()
                },
                if framework_stat.max_latency == "99999" {
                    "FAIL".to_string()
                } else {
                    framework_stat.max_latency.to_string()
                },
                (framework_stat.total_requests as u64).to_formatted_string(&Locale::en)
            )
            .unwrap();
        }
        fs::write(
            format!("results/concurrency-{}.md", concurrency),
            markdown_string,
        )
        .unwrap();
    }
}

fn parse_frameworks() -> Vec<Framework> {
    serde_json::from_str(FRAMEWORK_SETTINGS).unwrap()
}

fn calculate_results(frameworks: &[Framework]) -> Vec<Stats> {
    let mut statistics: Vec<Stats> =
        Vec::with_capacity(BENCHMARK_SETTINGS.len() * frameworks.len());
    // for every setting type, fetch all frameworks stats
    for setting in &BENCHMARK_SETTINGS {
        for framework in frameworks {
            let benchmark_result = read_perf_data(framework.binary, setting.concurrency);

            lazy_static! {
                static ref LATENCY_RGX: Regex = Regex::new(r"([0-9]+.[0-9]+[a-z]+)").unwrap();
                static ref TOTAL_REQUESTS_RGX: Regex = Regex::new(r"Total:\s[0-9]+\s").unwrap();
                static ref REQUESTS_PER_SECOND_RGX: Regex =
                    Regex::new(r"Sec:\s[0-9]+.[0-9]+\s").unwrap();
                static ref LATENCY_PERCENTILE_90: Regex =
                    Regex::new(r"90%\s*[0-9]+.[0-9]*[a-z]").unwrap();
                static ref LATENCY_PERCENTILE_99: Regex =
                    Regex::new(r"99%\s*[0-9]+.[0-9]*[a-z]").unwrap();
            }
            // ^ TODO: Add 95 and 99 %ile
            // let latency_perc_90 = LATENCY_PERCENTILE_90
            //     .find_iter(&)my_string
            //     .map(|mat| mat.as_str())
            //     .collect::<String>();

            let framework_stats: Stats = calculate_stats(
                framework.name,
                &LATENCY_RGX,
                &TOTAL_REQUESTS_RGX,
                &REQUESTS_PER_SECOND_RGX,
                &benchmark_result,
                setting.concurrency,
            );

            statistics.push(framework_stats)
        }
    }
    statistics
}

fn sort_framework(frameworks: &mut [Framework]) -> Vec<Vec<Stats>> {
    // calculate results and render into markdown table
    let mut vec = calculate_results(frameworks);

    let mut sorted_frameworks = Vec::new();

    let chunks = vec.chunks_mut(frameworks.len());
    chunks.for_each(|curr| {
        curr.sort_by(|curr, next| {
            next.requests_per_second
                .partial_cmp(&curr.requests_per_second)
                .unwrap()
        });
        sorted_frameworks.push(curr.to_vec());
    });

    sorted_frameworks
}

fn start_bench(setting: &Settings, port: u32) -> Output {
    Command::new("rewrk")
        .arg(format!("-d{}s", setting.duration))
        .arg(format!("--threads={}", setting.threads))
        .arg(format!("-c{}", setting.concurrency))
        .arg(format!("-h=http://localhost:{}", port))
        .output()
        .unwrap()
}

fn show_progress_bar(settings: &Settings) {
    let goal = settings.duration;
    let pb = ProgressBar::new(goal.into());
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} ❖⎨{bar:40.white}⎬❖ ⏰ [{elapsed_precise}]")
            .progress_chars(r"▋░"),
    );
    let duration = settings.duration + 1;

    std::thread::spawn(move || {
        for _ in 0..duration {
            pb.inc((1_u32).into());
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}

fn calculate_stats(
    framework_name: &str,
    latency_regex: &Regex,
    requests_regex: &Regex,
    rps_regex: &Regex,
    benchmark_result: &str,
    concurrency: u32,
) -> Stats {
    let latency_string: String = latency_regex
        .find_iter(&benchmark_result)
        .map(|mat| format!("{} ", mat.as_str()))
        .collect();
    let mut latencies = latency_string.split_whitespace();
    let avg_latency = latencies.next().unwrap_or("99999");
    let max_latency = latencies.nth(2).unwrap_or("99999");
    let total_requests_string: String = requests_regex
        .find_iter(&benchmark_result)
        .map(|mat| mat.as_str())
        .collect();
    let requests_per_sec_string: String = rps_regex
        .find_iter(&benchmark_result)
        .map(|mat| mat.as_str())
        .collect();

    let total_requests = total_requests_string
        .split_whitespace()
        .nth(1)
        .unwrap_or("0");
    let req_per_sec = requests_per_sec_string
        .split_whitespace()
        .nth(1)
        .unwrap_or("0");
    Stats {
        requests_per_second: req_per_sec.parse::<f64>().unwrap(),
        name: framework_name.to_string(),
        average_latency: avg_latency.to_string(),
        max_latency: max_latency.to_string(),
        total_requests: total_requests.parse().unwrap(),
        concurrency,
    }
}

fn read_perf_data(binary_name: &str, concurrency: u32) -> String {
    fs::read_to_string(format!(
        "perf/{}/{}.txt",
        binary_name, concurrency
    ))
    .unwrap_or_else(|_| {
        println!(
            "\
            Couldn't find the file: perf/{}/{}.txt. {} failed to finish the tests successfully, it will not be printed out in the results.
            ",
            binary_name, concurrency, binary_name
        );
        "".to_string()
    })
}

fn print_current_info(
    framework_index: usize,
    bench_index: usize,
    settings: &Settings,
    framework_name: &str,
) {
    println!(
        " {} {} {} {} {}\n",
        format!(
            " ⁅{}/{}⁆ ",
            framework_index * BENCHMARK_SETTINGS.len() + bench_index + 1,
            BENCHMARK_SETTINGS.len() * parse_frameworks().len()
        )
        .black()
        .on_bright_cyan(),
        format!(" 🚀 RUNNING: {} ", framework_name)
            .bright_white()
            .on_bright_red(),
        format!(" 💪 CONCURRENCY: {} ", settings.concurrency)
            .black()
            .on_bright_white(),
        format!(" 🪡 REWRK THREADS: {} ", settings.threads)
            .bright_white()
            .on_bright_black(),
        format!(" ⏰ DURATION: {}s ", settings.duration)
            .black()
            .on_bright_yellow(),
    );
}

fn print_expected_time(total_frameworks: usize) {
    let total_time_per_framework = BENCHMARK_SETTINGS
        .iter()
        .fold(0, |accumulated, current| accumulated + current.duration + 4);
    println!(
        "\t\t 💤 Benchmark will take around {} to finish.\n\n",
        format!(
            " {} minutes {} seconds ",
            (total_time_per_framework * total_frameworks as u32 / 60),
            (total_time_per_framework * total_frameworks as u32 % 60)
        )
        .black()
        .on_white()
    );
}

fn print_benchmark_message() {
    print!("{esc}c", esc = 27 as char);
    println!(
        "\n\n\t █▀▀ ▀▀█▀▀ █▀▀█ █▀▀█ ▀▀█▀▀   █▀▀▄ █▀▀ █▀▀▄ █▀▀ █░░█ █▀▄▀█ █▀▀█ █▀▀█ █░█
\t ▀▀█ ░░█░░ █▄▄█ █▄▄▀ ░░█░░   █▀▀▄ █▀▀ █░░█ █░░ █▀▀█ █░▀░█ █▄▄█ █▄▄▀ █▀▄
\t ▀▀▀ ░░▀░░ ▀░░▀ ▀░▀▀ ░░▀░░   ▀▀▀░ ▀▀▀ ▀░░▀ ▀▀▀ ▀░░▀ ▀░░░▀ ▀░░▀ ▀░▀▀ ▀░▀\n\n"
    );
}

static MARKDOWN_HEADER: &str =
    "|   **Name**   |   Req/sec   | Avg Latency | Max Latency |  # Requests |\n|:------------:|:-----------:|:-----------:|:-----------:|:-----------:|";
