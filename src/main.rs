use pyo3::prelude::*;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::HashMap;
use chrono::Local;

/// Primer设计参数结构体
#[derive(Debug, Clone)]
struct PrimerDesignParams {
    primer_len_opt: i32,
    primer_len_min: i32,
    primer_len_max: i32,
    primer_tm_opt: i32,
    primer_tm_min: i32,
    primer_tm_max: i32,
    probe_len_opt: i32,
    probe_len_min: i32,
    probe_len_max: i32,
    probe_tm_opt: i32,
    probe_tm_min: i32,
    probe_tm_max: i32,
    primer_gc_min: i32,
    primer_gc_max: i32,
    target_len_min: i32,
    target_len_max: i32,
    primer_num: i32,
    dg_hairpin: f64,
    dg_homodimer: f64,
    dg_heterodimer: f64,
}

impl Default for PrimerDesignParams {
    fn default() -> Self {
        Self {
            primer_len_opt: 21,
            primer_len_min: 18,
            primer_len_max: 25,
            primer_tm_opt: 62,
            primer_tm_min: 58,
            primer_tm_max: 70,
            probe_len_opt: 21,
            probe_len_min: 18,
            probe_len_max: 25,
            probe_tm_opt: 65,
            probe_tm_min: 56,
            probe_tm_max: 70,
            primer_gc_min: 40,
            primer_gc_max: 65,
            target_len_min: 1050,
            target_len_max: 1990,
            primer_num: 5,
            dg_hairpin: 0.0,
            dg_homodimer: -4.0,
            dg_heterodimer: -4.0,
        }
    }
}

/// 命令行参数
struct Args {
    input: String,
    output: String,
    species: String,
    params: PrimerDesignParams,
}

/// 打印帮助信息
fn print_help() {
    println!("\
PRIMER DESIGN v1.0 - Design primers using primer3

Usage: primer3_design -i <input_fasta> -o <output_file> -s <species> [options]

Required options:
  -i, --input <file>     FASTA input file
  -o, --output <file>    PRIMER result output file
  -s, --species <name>   SPECIES name

Primer options:
  --primerLen_opt <int>  PRIMER_OPT_SIZE (default: 21)
  --primerLen_min <int>  PRIMER_MIN_SIZE (default: 18)
  --primerLen_max <int>  PRIMER_MAX_SIZE (default: 25)
  --primerTm_opt <int>   PRIMER_OPT_TM (default: 62)
  --primerTm_min <int>   PRIMER_MIN_TM (default: 58)
  --primerTm_max <int>   PRIMER_MAX_TM (default: 70)

Probe options:
  --probeLen_opt <int>   Probe_OPT_SIZE (default: 21)
  --probeLen_min <int>   Probe_MIN_SIZE (default: 18)
  --probeLen_max <int>   Probe_MAX_SIZE (default: 25)
  --probeTm_opt <int>    Probe_OPT_TM (default: 65)
  --probeTm_min <int>    Probe_MIN_TM (default: 56)
  --probeTm_max <int>    Probe_MAX_TM (default: 70)

Other options:
  --primerGC_min <int>   PRIMER_MIN_GC (default: 40)
  --primerGC_max <int>   PRIMER_MAX_GC (default: 65)
  --targetLen_min <int>  PRIMER_PRODUCT_SIZE_RANGE min (default: 1050)
  --targetLen_max <int>  PRIMER_PRODUCT_SIZE_RANGE max (default: 1990)
  --primerNum <int>      PRIMER return num (default: 5)
  --dg_Hairpin <float>   PRIMER_Hairpin formation thermodynamics (default: 0)
  --dg_Homodimer <float> PRIMER_Homodimer formation thermodynamics (default: -4)
  --dg_Heterodimer <float> PRIMER_Heterodimer formation thermodynamics (default: -4)
  -v, --version          Show version information
  -h, --help             Show this help message
");
}

/// 解析命令行参数
fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut species: Option<String> = None;
    let mut params = PrimerDesignParams::default();

        let mut i = 1;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "-v" | "--version" => {
                println!("PRIMER DESIGN v1.0");
                process::exit(0);
            }
            "-i" | "--input" => {
                if i + 1 < args.len() {
                    input = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: -i/--input requires an argument");
                    process::exit(1);
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: -o/--output requires an argument");
                    process::exit(1);
                }
            }
            "-s" | "--species" => {
                if i + 1 < args.len() {
                    species = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: -s/--species requires an argument");
                    process::exit(1);
                }
            }
            "--primerLen_opt" => {
                if i + 1 < args.len() {
                    params.primer_len_opt = args[i + 1].parse().unwrap_or(21);
                    i += 1;
                }
            }
            "--primerLen_min" => {
                if i + 1 < args.len() {
                    params.primer_len_min = args[i + 1].parse().unwrap_or(18);
                    i += 1;
                }
            }
            "--primerLen_max" => {
                if i + 1 < args.len() {
                    params.primer_len_max = args[i + 1].parse().unwrap_or(25);
                    i += 1;
                }
            }
            "--primerTm_opt" => {
                if i + 1 < args.len() {
                    params.primer_tm_opt = args[i + 1].parse().unwrap_or(62);
                    i += 1;
                }
            }
            "--primerTm_min" => {
                if i + 1 < args.len() {
                    params.primer_tm_min = args[i + 1].parse().unwrap_or(58);
                    i += 1;
                }
            }
            "--primerTm_max" => {
                if i + 1 < args.len() {
                    params.primer_tm_max = args[i + 1].parse().unwrap_or(70);
                    i += 1;
                }
            }
            "--probeLen_opt" => {
                if i + 1 < args.len() {
                    params.probe_len_opt = args[i + 1].parse().unwrap_or(21);
                    i += 1;
                }
            }
            "--probeLen_min" => {
                if i + 1 < args.len() {
                    params.probe_len_min = args[i + 1].parse().unwrap_or(18);
                    i += 1;
                }
            }
            "--probeLen_max" => {
                if i + 1 < args.len() {
                    params.probe_len_max = args[i + 1].parse().unwrap_or(25);
                    i += 1;
                }
            }
            "--probeTm_opt" => {
                if i + 1 < args.len() {
                    params.probe_tm_opt = args[i + 1].parse().unwrap_or(65);
                    i += 1;
                }
            }
            "--probeTm_min" => {
                if i + 1 < args.len() {
                    params.probe_tm_min = args[i + 1].parse().unwrap_or(56);
                    i += 1;
                }
            }
            "--probeTm_max" => {
                if i + 1 < args.len() {
                    params.probe_tm_max = args[i + 1].parse().unwrap_or(70);
                    i += 1;
                }
            }
            "--primerGC_min" => {
                if i + 1 < args.len() {
                    params.primer_gc_min = args[i + 1].parse().unwrap_or(40);
                    i += 1;
                }
            }
            "--primerGC_max" => {
                if i + 1 < args.len() {
                    params.primer_gc_max = args[i + 1].parse().unwrap_or(65);
                    i += 1;
                }
            }
            "--targetLen_min" => {
                if i + 1 < args.len() {
                    params.target_len_min = args[i + 1].parse().unwrap_or(1050);
                    i += 1;
                }
            }
            "--targetLen_max" => {
                if i + 1 < args.len() {
                    params.target_len_max = args[i + 1].parse().unwrap_or(1990);
                    i += 1;
                }
            }
            "--primerNum" => {
                if i + 1 < args.len() {
                    params.primer_num = args[i + 1].parse().unwrap_or(5);
                    i += 1;
                }
            }
            s if s.starts_with("--dg_Hairpin") => {
                // 支持 --dg_Hairpin=0.1 和 --dg_Hairpin 0.1 两种格式
                if arg.contains('=') {
                    if let Some(value) = arg.split('=').nth(1) {
                        params.dg_hairpin = value.parse().unwrap_or(0.0);
                    }
                } else if i + 1 < args.len() {
                    params.dg_hairpin = args[i + 1].parse().unwrap_or(0.0);
                    i += 1;
                }
            }
            s if s.starts_with("--dg_Homodimer") => {
                if arg.contains('=') {
                    if let Some(value) = arg.split('=').nth(1) {
                        params.dg_homodimer = value.parse().unwrap_or(-4.0);
                    }
                } else if i + 1 < args.len() {
                    params.dg_homodimer = args[i + 1].parse().unwrap_or(-4.0);
                    i += 1;
                }
            }
            s if s.starts_with("--dg_Heterodimer") => {
                if arg.contains('=') {
                    if let Some(value) = arg.split('=').nth(1) {
                        params.dg_heterodimer = value.parse().unwrap_or(-4.0);
                    }
                } else if i + 1 < args.len() {
                    params.dg_heterodimer = args[i + 1].parse().unwrap_or(-4.0);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    Args {
        input: input.unwrap_or_else(|| {
            eprintln!("Error: -i/--input is required");
            process::exit(1);
        }),
        output: output.unwrap_or_else(|| {
            eprintln!("Error: -o/--output is required");
            process::exit(1);
        }),
        species: species.unwrap_or_else(|| {
            eprintln!("Error: -s/--species is required");
            process::exit(1);
        }),
        params,
    }
}

/// DNA互补序列
fn dna_complement(seq: &str) -> String {
    seq.chars()
        .map(|c| match c {
            'A' | 'a' => 'T',
            'T' | 't' => 'A',
            'C' | 'c' => 'G',
            'G' | 'g' => 'C',
            _ => c,
        })
        .collect()
}

/// DNA反向序列
fn dna_reverse(seq: &str) -> String {
    seq.chars().rev().collect()
}

/// 检查序列是否只包含ATGC
fn is_valid_sequence(seq: &str) -> bool {
    let seq_upper = seq.to_uppercase();
    seq_upper.chars().all(|c| c == 'A' || c == 'T' || c == 'G' || c == 'C')
}

/// 读取FASTA文件
fn read_fasta(filename: &str) -> Vec<(String, String)> {
    let file = File::open(filename).expect("Unable to open FASTA file");
    let reader = BufReader::new(file);
    let mut records: Vec<(String, String)> = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_seq = String::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let line = line.trim();
        if line.starts_with('>') {
            if let Some(id) = current_id.take() {
                if !current_seq.is_empty() {
                    records.push((id, current_seq));
                }
                current_seq = String::new();
            }
            current_id = Some(line[1..].to_string());
        } else {
            current_seq.push_str(line);
        }
    }

    if let Some(id) = current_id {
        if !current_seq.is_empty() {
            records.push((id, current_seq));
        }
    }

    records
}

/// 构建全局参数字典
fn build_global_params(params: &PrimerDesignParams) -> HashMap<&'static str, Py<PyAny>> {
    Python::with_gil(|py| {
        let mut global_args: HashMap<&'static str, Py<PyAny>> = HashMap::new();

        // Primer参数
        global_args.insert("PRIMER_OPT_SIZE", params.primer_len_opt.to_object(py));
        global_args.insert("PRIMER_PICK_INTERNAL_OLIGO", 1_i32.to_object(py));
        global_args.insert("PRIMER_THERMODYNAMIC_OLIGO_ALIGNMENT", 1_i32.to_object(py));
        global_args.insert("PRIMER_PICK_ANYWAY", 1_i32.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MAX_SELF_END", 8_i32.to_object(py));
        global_args.insert("PRIMER_INTERNAL_OPT_SIZE", params.probe_len_opt.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MAX_SIZE", params.probe_len_max.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MIN_SIZE", params.probe_len_min.to_object(py));
        global_args.insert("PRIMER_INTERNAL_OPT_TM", params.probe_tm_opt.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MIN_TM", params.probe_tm_min.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MAX_TM", params.probe_tm_max.to_object(py));
        global_args.insert("PRIMER_MIN_SIZE", params.primer_len_min.to_object(py));
        global_args.insert("PRIMER_MAX_SIZE", params.primer_len_max.to_object(py));
        global_args.insert("PRIMER_OPT_TM", params.primer_tm_opt.to_object(py));
        global_args.insert("PRIMER_MIN_TM", params.primer_tm_min.to_object(py));
        global_args.insert("PRIMER_PAIR_MAX_DIFF_TM", 2.0_f64.to_object(py));
        global_args.insert("PRIMER_WT_TM_LT", 1.0_f64.to_object(py));
        global_args.insert("PRIMER_WT_TM_GT", 1.0_f64.to_object(py));
        global_args.insert("PRIMER_PAIR_WT_DIFF_TM", 0.0_f64.to_object(py));
        global_args.insert("PRIMER_MAX_TM", params.primer_tm_max.to_object(py));
        global_args.insert("PRIMER_MIN_GC", params.primer_gc_min.to_object(py));
        global_args.insert("PRIMER_MAX_GC", params.primer_gc_max.to_object(py));
        global_args.insert("PRIMER_WT_SIZE_LT", 1.0_f64.to_object(py));
        global_args.insert("PRIMER_WT_SIZE_GT", 1.0_f64.to_object(py));
        global_args.insert("PRIMER_MAX_POLY_X", 3_i32.to_object(py));
        global_args.insert("PRIMER_INTERNAL_MAX_POLY_X", 3_i32.to_object(py));
        global_args.insert("PRIMER_SALT_MONOVALENT", 50.0_f64.to_object(py));
        global_args.insert("PRIMER_DNA_CONC", 50.0_f64.to_object(py));
        global_args.insert("PRIMER_MAX_NS_ACCEPTED", 0_i32.to_object(py));
        global_args.insert("PRIMER_MAX_SELF_ANY", 8_i32.to_object(py));
        global_args.insert("PRIMER_MAX_SELF_ANY_TH", 35.0_f64.to_object(py));
        global_args.insert("PRIMER_MAX_SELF_END", 3_i32.to_object(py));
        global_args.insert("PRIMER_MAX_SELF_END_TH", 35.0_f64.to_object(py));
        global_args.insert("PRIMER_PAIR_MAX_COMPL_ANY", 6_i32.to_object(py));
        global_args.insert("PRIMER_PAIR_MAX_COMPL_END", 3_i32.to_object(py));
        global_args.insert("PRIMER_PAIR_MAX_COMPL_ANY_TH", 35.0_f64.to_object(py));
        global_args.insert("PRIMER_PAIR_MAX_COMPL_END_TH", 35.0_f64.to_object(py));
        global_args.insert("PRIMER_WT_HAIRPIN_TH", 0.0_f64.to_object(py));
        global_args.insert("PRIMER_MAX_END_STABILITY", 9.0_f64.to_object(py));
        global_args.insert("PRIMER_WT_END_STABILITY", 0.0_f64.to_object(py));

        // Product size range
        let size_range: Vec<i32> = vec![params.target_len_min, params.target_len_max];
        global_args.insert("PRIMER_PRODUCT_SIZE_RANGE", size_range.to_object(py));
        global_args.insert("PRIMER_NUM_RETURN", params.primer_num.to_object(py));

        global_args
    })
}

/// 计算发夹结构ΔG
fn calc_hairpin(seq: &str) -> PyResult<f64> {
    Python::with_gil(|py| {
        let primer3 = PyModule::import_bound(py, "primer3.bindings")?;
        let hairpin = primer3.getattr("calc_hairpin")?.call1((seq,))?;
        let dg: f64 = hairpin.getattr("dg")?.extract()?;
        Ok(dg / 1000.0)
    })
}

/// 计算同源二聚体ΔG
fn calc_homodimer(seq: &str) -> PyResult<f64> {
    Python::with_gil(|py| {
        let primer3 = PyModule::import_bound(py, "primer3.bindings")?;
        let homodimer = primer3.getattr("calc_homodimer")?.call1((seq,))?;
        let dg: f64 = homodimer.getattr("dg")?.extract()?;
        Ok(dg / 1000.0)
    })
}

/// 计算异源二聚体ΔG
fn calc_heterodimer(seq1: &str, seq2: &str) -> PyResult<f64> {
    Python::with_gil(|py| {
        let primer3 = PyModule::import_bound(py, "primer3.bindings")?;
        let heterodimer = primer3.getattr("calc_heterodimer")?.call1((seq1, seq2))?;
        let dg: f64 = heterodimer.getattr("dg")?.extract()?;
        Ok(dg / 1000.0)
    })
}

/// 计算末端稳定性ΔG
fn calc_end_stability(primer: &str, template: &str) -> PyResult<f64> {
    Python::with_gil(|py| {
        let primer3 = PyModule::import_bound(py, "primer3.bindings")?;
        let end_stability = primer3.getattr("calc_end_stability")?.call1((primer, template))?;
        let dg: f64 = end_stability.getattr("dg")?.extract()?;
        Ok(dg / 1000.0)
    })
}

/// 计算Tm值
fn calc_tm(seq: &str) -> PyResult<f64> {
    Python::with_gil(|py| {
        let primer3 = PyModule::import_bound(py, "primer3")?;
        let tm = primer3.getattr("calcTm")?.call1((seq,))?;
        tm.extract()
    })
}

/// 设计引物
fn design_primers(args: &Args) -> PyResult<String> {
    let fasta_records = read_fasta(&args.input);

    let mut results: Vec<String> = Vec::new();
    results.push(String::from("Species\tID\tF_primer\tR_primer\tProbe\tprimer_TM\tprimer_GC\tdg_Homodimer_Heterodimer\tdg_EndStability\tPCR_product\tPCR_product_len\tPCR_product_TM"));

    for (seq_id, seq) in fasta_records {
        // 过滤包含非ATGC的序列
        if !is_valid_sequence(&seq) {
            continue;
        }

        Python::with_gil(|py| {
            // 构建primer3输入
            let p3_seq: HashMap<&str, String> = HashMap::from([
                ("SEQUENCE_ID", seq_id.clone()),
                ("SEQUENCE_TEMPLATE", seq.clone()),
            ]);

            // 构建全局参数（每次在GIL内重新构建）
            let global_params = build_global_params(&args.params);

            // 转换为Python字典
            let py_p3_seq = p3_seq.to_object(py);
            let py_global_params = global_params.to_object(py);

            // 调用primer3设计引物
            let primer3 = PyModule::import_bound(py, "primer3.bindings")?;
            let design_primers = primer3.getattr("design_primers")?;
            let result = design_primers.call1((py_p3_seq, py_global_params))?;

            // 获取返回的引物对数量
            let pair_num_returned: i32 = result
                .get_item("PRIMER_PAIR_NUM_RETURNED")?
                .extract()?;

            for index in 0..pair_num_returned {
                let idx_str = index.to_string();

                // 获取引物序列
                let primer_left: String = match result.get_item(format!("PRIMER_LEFT_{}_SEQUENCE", idx_str).as_str()) {
                    Ok(seq) => seq.extract().unwrap_or_default(),
                    Err(_) => continue,
                };
                let primer_right: String = match result.get_item(format!("PRIMER_RIGHT_{}_SEQUENCE", idx_str).as_str()) {
                    Ok(seq) => seq.extract().unwrap_or_default(),
                    Err(_) => continue,
                };
                let primer_probe: String = match result.get_item(format!("PRIMER_INTERNAL_{}_SEQUENCE", idx_str).as_str()) {
                    Ok(seq) => seq.extract().unwrap_or_default(),
                    Err(_) => continue,
                };

                // 获取Tm值
                let tm_left: f64 = match result.get_item(format!("PRIMER_LEFT_{}_TM", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };
                let tm_right: f64 = match result.get_item(format!("PRIMER_RIGHT_{}_TM", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };
                let tm_probe: f64 = match result.get_item(format!("PRIMER_INTERNAL_{}_TM", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };

                // 获取GC含量
                let gc_left: f64 = match result.get_item(format!("PRIMER_LEFT_{}_GC_PERCENT", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };
                let gc_right: f64 = match result.get_item(format!("PRIMER_RIGHT_{}_GC_PERCENT", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };
                let gc_probe: f64 = match result.get_item(format!("PRIMER_INTERNAL_{}_GC_PERCENT", idx_str).as_str()) {
                    Ok(val) => val.extract().unwrap_or(0.0),
                    Err(_) => continue,
                };

                // 获取引物位置
                let left_pos: Vec<i32> = match result.get_item(format!("PRIMER_LEFT_{}", idx_str).as_str()) {
                    Ok(pos) => pos.extract().unwrap_or_default(),
                    Err(_) => continue,
                };
                let right_pos: Vec<i32> = match result.get_item(format!("PRIMER_RIGHT_{}", idx_str).as_str()) {
                    Ok(pos) => pos.extract().unwrap_or_default(),
                    Err(_) => continue,
                };

                if left_pos.is_empty() || right_pos.is_empty() {
                    continue;
                }

                // 计算PCR产物
                let start = left_pos[0] as usize;
                let end = right_pos[0] as usize + 1;
                let pcr_product = if end <= seq.len() {
                    &seq[start..end]
                } else {
                    ""
                };

                let pcr_product_len = pcr_product.len();
                let pcr_product_tm = match calc_tm(pcr_product) {
                    Ok(tm) => (tm * 100.0).round() / 100.0,
                    Err(_) => 0.0,
                };

                // 计算热力学参数
                let dg_hairpin_left = calc_hairpin(&primer_left).unwrap_or(0.0);
                let dg_hairpin_right = calc_hairpin(&primer_right).unwrap_or(0.0);
                let dg_homodimer_left = calc_homodimer(&primer_left).unwrap_or(0.0);
                let dg_homodimer_right = calc_homodimer(&primer_right).unwrap_or(0.0);
                let dg_heterodimer = calc_heterodimer(&primer_left, &primer_right).unwrap_or(0.0);

                // 计算末端稳定性
                let primer_left_revcomp = dna_reverse(&dna_complement(&primer_left));
                let dg_end_stability_left = calc_end_stability(&primer_left_revcomp, &seq).unwrap_or(0.0);
                let dg_end_stability_right = calc_end_stability(&primer_right, &seq).unwrap_or(0.0);

                // 过滤条件
                let tm_ok = (tm_left - tm_right).abs() <= 2.0;
                let homo_f_ok = dg_homodimer_left >= args.params.dg_homodimer;
                let homo_r_ok = dg_homodimer_right >= args.params.dg_homodimer;
                let hetero_ok = dg_heterodimer >= args.params.dg_heterodimer;
                let probe_ok = !primer_probe.starts_with('G');

                let passed = tm_ok && homo_f_ok && homo_r_ok && hetero_ok && probe_ok;

                if passed {
                    // 格式化输出 - 统一转换为大写
                    let primer_left_upper = primer_left.to_uppercase();
                    let primer_right_upper = primer_right.to_uppercase();
                    let primer_probe_upper = primer_probe.to_uppercase();

                    let tm_str = format!("{:.2}|{:.2}|{:.2}", tm_left, tm_right, tm_probe);
                    let gc_str = format!("{:.2}|{:.2}|{:.2}", gc_left, gc_right, gc_probe);
                    let dg_homo_hetero = format!(
                        "{:.2}|{:.2}|{:.2}",
                        dg_homodimer_left, dg_homodimer_right, dg_heterodimer
                    );
                    let dg_end_stab = format!("{:.2}|{:.2}", dg_end_stability_left, dg_end_stability_right);

                    results.push(format!(
                        "{}\t{}-{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        args.species,
                        seq_id,
                        idx_str,
                        format!("{}\t{}", primer_left_upper, primer_right_upper),
                        primer_probe_upper,
                        tm_str,
                        gc_str,
                        dg_homo_hetero,
                        dg_end_stab,
                        pcr_product,
                        pcr_product_len,
                        pcr_product_tm
                    ));
                }
            }

            Ok::<(), PyErr>(())
        })?;
    }

    Ok(results.join("\n"))
}

fn main() -> PyResult<()> {
    let args = parse_args();

    // 输出关键参数
    eprintln!("🧬 Primer Design Parameters:");
    eprintln!("  dg_Homodimer: {}", args.params.dg_homodimer);
    eprintln!("  dg_Heterodimer: {}", args.params.dg_heterodimer);
    eprintln!();

    let start_time = std::time::Instant::now();

    match design_primers(&args) {
        Ok(results) => {
            let elapsed = start_time.elapsed();
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            if let Err(e) = fs::write(&args.output, results) {
                eprintln!("Error writing output file: {}", e);
                process::exit(1);
            }

            println!("✅ Primer design completed!");
            println!("📄 Results written to: {}", args.output);
            println!("⏱️  Time: {} ({}s)", timestamp, elapsed.as_secs_f64());
        }
        Err(e) => {
            eprintln!("Error during primer design: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}
