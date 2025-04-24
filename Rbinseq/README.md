# Rbinseq

R bindings for the BinSeq format, a highly compressed binary format for storing sequencing data.

> **Note:** This is a proof of concept binding for the BinSeq library. It demonstrates how the BinSeq format can be integrated with R but may not be feature-complete for production use.

## Overview

Rbinseq provides an R interface to the BinSeq library, allowing you to read and analyze BinSeq files directly from R. This package enables efficient processing of large-scale sequencing data by leveraging the performance benefits of the BinSeq format.

## Installation

### Prerequisites

- R 
- A C compiler (gcc, clang, etc.)
- [`cargo`](https://rustup.rs/) - Rust's package manager
- GNU make
- Make sure you have the necessary development tools installed:
  - On Ubuntu/Debian: `sudo apt-get install r-base-dev`
  - On CentOS/RHEL: `sudo yum install R-devel`
  - On macOS with Homebrew: `brew install r` and `brew install --cask rstudio`


### Installing from Local Source

To install the package from a local copy of the source code:

1. Clone the repository:
   ```bash
   git clone https://github.com/sounkou-bioinfo/binseq-bindings.git --branch Rbinseq
   cd binseq-bindings
   ```

2. Build and install the R package:
   ```bash
   R CMD INSTALL Rbinseq
   ```

## Usage  Example

After installation, you can load the package and test it with the included sample file:

```bash
Rscript -e 'Rbinseq::test_readBinseq()'
#Reading sample BinSeq file: /usr/local/lib/R/site-library/Rbinseq/extdata/subset.bq
#File: /usr/local/lib/R/site-library/Rbinseq/extdata/subset.bq
#Records: 24890
#Primary Sequence length: 28
#Extended Sequence length: 90
#
#First 5 records:
#------------------
#Record 0 (flag=0):
#  Sequence (28 bp): CGGTATTGTTAGCGCCGTCATTATCCAA
#  Paired (90 bp): ACGCGGTTAGCACGTACAAGTAGGCTCTTGCTATGCACTCTTGTGCTTAGCTCTGAAACTCGTGTCCTGTGGGCAAAGCCGGTCCTAGCA
#Record 1 (flag=0):
#  Sequence (28 bp): TGAGGTCCAGGGCTATGTGGCCAGCTAA
#  Paired (90 bp): ACGCGGTTAGCACGTAGCAGAGGCAACTTGCTATGCACTCTTGTGCTTAGCTCTGAAACTCATGTCAGCGCTGCCGGCCCGGTCCTAGCA
#Record 2 (flag=0):
#  Sequence (28 bp): TCAAGTTTCGGGCAGTGTCTGAGCTAAG
#  Paired (90 bp): ACGCGGTTAGCACGTAAAACTACTCACTTGCTATGCACTCTTGTGCTTAGCTCTGAAACCGTCACAGCCTCGTCTGCGCCGGTCCTAGCA
#Record 3 (flag=0):
#  Sequence (28 bp): TGGTTAAAGACTTAAGTAACAAATTGGG
#  Paired (90 bp): ACGCGGTTAGCACGTAGGATACGTCAGCTATGCTGTTTCCAGCTTAGCTCTTAAACTAGGTGGTTGTGGCCACTGCCGGTCCTAGCAACC
#Record 4 (flag=0):
#  Sequence (28 bp): CAATTGCGTCAACTACTACATCCAGCCA
#  Paired (90 bp): ACGCGGTTAGCACGTAGTATCCCAACGCTATGCTGTTTCCAGCTTAGCTCTTAAACGAACTCTCCCCGCCGGCGACCGGTCCTAGCAATG
#
#Performance test: processing all 24890 records...
#Processed 2937020 bases in 0.0024 seconds
#1199.27 million bases per second
```


This will display information about the file, including:
- Number of records
- Sequence length
- Whether it contains paired-end reads
- A preview of the first few records
- Performance statistics when processing the entire file

