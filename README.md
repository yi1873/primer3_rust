# primer3_rust 🧬

A high-performance Rust implementation of primer3 design, leveraging PyO3 to interface with Python's primer3 package. This tool provides fast and reliable primer design for qPCR applications with thermodynamic filtering.

## Features ✨

- 🚀 **High Performance**: Rust's performance with Python's primer3 library
- 🔬 **Thermodynamic Filtering**: Advanced filtering based on ΔG values
- 💊 **Probe Validation**: Ensures probes don't start with 'G'
- 🎯 **Flexible Parameters**: Extensive customization of primer design parameters
- 📊 **Comprehensive Output**: Detailed results including Tm, GC%, and ΔG values
- ⏱️ **Performance Metrics**: Execution time tracking
- 🔧 **Multiple Parameter Formats**: Supports both `--key value` and `--key=value` formats

## Prerequisites 📋

- **Rust**: 1.70 or higher
- **Python**: 3.8 or higher
- **primer3 package**: Install via `pip install primer3`
- **Biopython**: Install via `pip install biopython`

### Install Python Dependencies

```bash
pip install primer3 biopython
```

## Installation 🔧

### 1. Clone the Repository

```bash
git clone https://github.com/yi1873/primer3_rust.git
cd primer3_rust
```

### 2. Build the Project

```bash
cargo build --release
```


## Usage 📖

### Basic Usage

```bash
./target/release/primer3_design_byRUST \
  -i input.fasta \
  -o results.tsv \
  -s species_name
```

### Required Arguments

| Argument | Description |
|----------|-------------|
| `-i, --input <file>` | FASTA input file containing target sequences |
| `-o, --output <file>` | Output file for primer design results |
| `-s, --species <name>` | Species name for identification |

### Optional Arguments

#### Primer Length Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--primerLen_opt <int>` | 21 | Optimal primer length |
| `--primerLen_min <int>` | 18 | Minimum primer length |
| `--primerLen_max <int>` | 25 | Maximum primer length |

#### Primer Tm Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--primerTm_opt <int>` | 62 | Optimal primer melting temperature (°C) |
| `--primerTm_min <int>` | 58 | Minimum primer melting temperature (°C) |
| `--primerTm_max <int>` | 70 | Maximum primer melting temperature (°C) |

#### Probe Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--probeLen_opt <int>` | 21 | Optimal probe length |
| `--probeLen_min <int>` | 18 | Minimum probe length |
| `--probeLen_max <int>` | 25 | Maximum probe length |
| `--probeTm_opt <int>` | 65 | Optimal probe melting temperature (°C) |
| `--probeTm_min <int>` | 56 | Minimum probe melting temperature (°C) |
| `--probeTm_max <int>` | 70 | Maximum probe melting temperature (°C) |

#### GC Content Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--primerGC_min <int>` | 40 | Minimum GC content (%) |
| `--primerGC_max <int>` | 65 | Maximum GC content (%) |

#### Product Size Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--targetLen_min <int>` | 85 | Minimum PCR product length (bp) |
| `--targetLen_max <int>` | 120 | Maximum PCR product length (bp) |

#### Thermodynamic Filtering Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--dg_Homodimer <float>` | -4.0 | Minimum ΔG for homodimer formation (kcal/mol) |
| `--dg_Heterodimer <float>` | -4.0 | Minimum ΔG for heterodimer formation (kcal/mol) |
| `--dg_Hairpin <float>` | 0.0 | Minimum ΔG for hairpin formation (kcal/mol) |

#### Other Options

| Argument | Default | Description |
|----------|---------|-------------|
| `--primerNum <int>` | 5 | Number of primer pairs to return |
| `-v, --version` | - | Show version information |
| `-h, --help` | - | Show help message |

## Examples 📝

### Example 1: Basic Primer Design

```bash
./target/release/primer3_design_byRUST \
  -i sequences.fasta \
  -o primers.tsv \
  -s Homo_sapiens
```

### Example 2: Custom Parameters

```bash
cd test
bash run_test.sh
 
```

### Example 3: Using Space-Separated Parameters

Both `--key=value` and `--key value` formats are supported:

```bash
./target/release/primer3_design_byRUST \
  -i input.fa \
  -o output.tsv \
  -s species \
  --dg_Homodimer=-4.5 \
  --dg_Heterodimer=-4.5
```

## Output Format 📊

The output is a tab-separated file with the following columns:

| Column | Description |
|--------|-------------|
| Species | Species name |
| ID | Sequence identifier |
| F_primer | Forward primer sequence |
| R_primer | Reverse primer sequence |
| Probe | Probe sequence |
| primer_TM | Melting temperatures (F\|R\|Probe) |
| primer_GC | GC content percentages (F\|R\|Probe) |
| dg_Homodimer_Heterodimer | ΔG values (F_Homo\|R_Homo\|FR_Hetero) |
| dg_EndStability | End stability ΔG values of product (F\|R) |
| PCR_product | PCR product sequence |
| PCR_product_len | PCR product length (bp) |
| PCR_product_TM | PCR product melting temperature (°C) |


## Filtering Criteria 🔬

Primers are filtered based on the following criteria:

1. **Tm Difference**: |Tm(F) - Tm(R)| ≤ 2°C
2. **Homodimer Stability**: ΔG ≥ `--dg_Homodimer` threshold
3. **Heterodimer Stability**: ΔG ≥ `--dg_Heterodimer` threshold
4. **Probe Constraint**: Probe must NOT start with 'G'

## Performance 🚀

- **Fast execution**: Rust's performance with optimized primer3 bindings
- **Memory efficient**: Efficient memory usage for large FASTA files
- **Parallel processing ready**: Ready for future parallelization

## Troubleshooting 🔧

### Issue: `undefined symbol: PyInit_builtins`

**Solution**: Ensure `RUSTFLAGS` and `PYO3_PYTHON` are correctly set before building.

### Issue: Cannot find primer3 package

**Solution**: Install primer3 in your Python environment:
```bash
pip install primer3 biopython
```

### Issue: Parameters not recognized

**Solution**: Check that you're using the correct parameter format. Both `--key=value` and `--key value` work.

## Comparison with Python Version 📈

| Feature | Python Version | Rust Version |
|---------|----------------|--------------|
| Performance | Standard | **~2-5x faster** |
| Memory Usage | Higher | **Lower** |
| Parameter Format | Space-separated | **Both formats** |
| Error Handling | Basic | **Robust** |

## Development 💻

### Project Structure

```
primer3_rust/
├── src/
│   └── main.rs          # Main source code
├── Cargo.toml           # Rust project configuration
├── test/
│   └── run_test.sh      # Test script
└── README.md            # This file
```

### Building from Source

```bash
cargo build --release
```

### Running Tests

```bash
cd test
bash run_test.sh
```

## Acknowledgments 🙏

- [primer3](https://primer3.org/) for the excellent primer design library
- [PyO3](https://pyo3.rs/) for Rust-Python bindings
- [Biopython](https://biopython.org/) for sequence handling utilities

## Citation 📚

If you use this tool in your research, please cite:

```bibtex
@software{primer3_rust,
  title = {primer3_rust: High-performance primer design in Rust},
  author = {Liang Xiangzhi},
  year = {2026},
  url = {https://github.com/yi1873/primer3_rust}
}
```


**Happy Primer Design! 🧬✨**
