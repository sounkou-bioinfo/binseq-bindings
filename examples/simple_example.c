#include "../binseq.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
  if (argc < 2) {
    printf("Usage: %s <binseq_file>\n", argv[0]);
    return 1;
  }

  // Open the binseq file (using struct keyword)
  struct BinseqReader *reader = binseq_reader_open(argv[1]);
  if (!reader) {
    printf("Error opening file: %s\n", binseq_last_error());
    return 1;
  }

  // Get basic file info
  size_t num_records = binseq_reader_num_records(reader);
  uint32_t slen = binseq_reader_slen(reader);
  uint32_t xlen = binseq_reader_xlen(reader);

  printf("File: %s\n", argv[1]);
  printf("Records: %zu\n", num_records);
  printf("Primary Sequence length: %u\n", slen);
  if (xlen > 0) {
    printf("Extended Sequence length: %u\n", xlen);
  } else {
    printf("Single-end reads\n");
  }

  // Show first 5 records (or fewer if file has less)
  // size_t records_to_show = (num_records < 5) ? num_records : 5;
  size_t records_to_show = num_records;

  // Allocate buffers for sequence data
  char *sbuf = (char *)malloc(slen + 1);
  char *xbuf = (char *)malloc(xlen + 1);

  if (!sbuf) {
    printf("Memory allocation failed\n");
    binseq_reader_close(reader);
    return 1;
  }

  if (xlen > 0) {
    xbuf = (char *)malloc(xlen + 1);
    if (!xbuf) {
      printf("Memory allocation failed\n");
      free(sbuf);
      binseq_reader_close(reader);
      return 1;
    }
  }

  printf("\nFirst %zu records:\n", records_to_show);
  printf("------------------\n");

  for (size_t i = 0; i < records_to_show; i++) {
    // Get the record (using struct keyword)
    struct BinseqRecord *record = binseq_reader_get_record(reader, i);
    if (!record) {
      printf("Error reading record %zu: %s\n", i, binseq_last_error());
      continue;
    }

    // Get the flag
    uint64_t flag = binseq_record_flag(record);

    // Get the sequence
    size_t actual_len = binseq_record_decode_primary(record, sbuf, slen);
    sbuf[actual_len] = '\0'; // Null-terminate for printing

    // printf("Record %zu (flag=%llu):\n", i, (unsigned long long)flag);
    // printf("  Sequence: %s\n", sbuf);

    // Get paired sequence if available
    if (binseq_record_is_paired(record) && xbuf) {
      size_t paired_actual_len =
          binseq_record_decode_extended(record, xbuf, xlen);
      xbuf[paired_actual_len] = '\0'; // Null-terminate for printing
      // printf("  Paired: %s\n", xbuf);
    }

    // Free the record when done
    binseq_record_free(record);
  }

  // Clean up
  free(sbuf);
  if (xbuf) {
    free(xbuf);
  }
  binseq_reader_close(reader);

  return 0;
}
