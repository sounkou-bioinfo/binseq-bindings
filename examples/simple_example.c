#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the binseq API
#include "../binseq.h"

int main(int argc, char **argv) {
  if (argc < 2) {
    printf("Usage: %s <binseq_file>\n", argv[0]);
    return 1;
  }

  // Open the binseq file
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

  // Create a decoding context
  struct BinseqContext *ctx = binseq_context_new();
  if (!ctx) {
    printf("Failed to create context\n");
    binseq_reader_close(reader);
    return 1;
  }

  // Create a reusable record container
  struct BinseqRecord *record = binseq_record_new();
  if (!record) {
    printf("Failed to create record container\n");
    binseq_context_free(ctx);
    binseq_reader_close(reader);
    return 1;
  }

  // Print the first few records
  size_t records_to_show = (num_records < 5) ? num_records : 5;
  printf("\nFirst %zu records:\n", records_to_show);
  printf("------------------\n");

  // Allocate buffers for displaying sequences (with null terminator)
  char *sbuf = (char *)malloc(slen + 1);
  char *xbuf = NULL;
  if (xlen > 0) {
    xbuf = (char *)malloc(xlen + 1);
  }

  if (!sbuf || (xlen > 0 && !xbuf)) {
    printf("Memory allocation failed\n");
    free(sbuf);
    free(xbuf);
    binseq_record_free(record);
    binseq_context_free(ctx);
    binseq_reader_close(reader);
    return 1;
  }

  for (size_t i = 0; i < records_to_show; i++) {
    // Load the record into our reusable container
    if (!binseq_reader_get_record(reader, i, record)) {
      printf("Error reading record %zu: %s\n", i, binseq_last_error());
      continue;
    }

    uint64_t flag = binseq_record_flag(record);

    // Decode the primary sequence
    size_t s_len = binseq_record_decode_primary(record, ctx);

    // Copy the sequence to our buffer and null-terminate
    binseq_context_copy_primary(ctx, sbuf, slen);
    sbuf[s_len] = '\0';

    printf("Record %zu (flag=%llu):\n", i, (unsigned long long)flag);
    printf("  Sequence (%zu bp): %s\n", s_len, sbuf);

    // If paired, decode the extended sequence too
    if (binseq_record_is_paired(record) && xbuf) {
      size_t x_len = binseq_record_decode_extended(record, ctx);

      // Copy the sequence to our buffer and null-terminate
      binseq_context_copy_extended(ctx, xbuf, xlen);
      xbuf[x_len] = '\0';

      printf("  Paired (%zu bp): %s\n", x_len, xbuf);
    }

    // No need to free the record after each use - it will be reused!
  }

  // Performance test
  if (num_records > 10) {
    printf("\nPerformance test: processing all %zu records...\n", num_records);
    clock_t start_time = clock();

    size_t total_bases = 0;
    for (size_t i = 0; i < num_records; i++) {
      // Reuse the same record container for each record
      if (!binseq_reader_get_record(reader, i, record)) {
        continue;
      }

      // Decode sequences and count bases
      size_t s_len = binseq_record_decode_primary(record, ctx);
      total_bases += s_len;

      if (binseq_record_is_paired(record)) {
        size_t x_len = binseq_record_decode_extended(record, ctx);
        total_bases += x_len;
      }

      // No need to free the record after each use
    }

    clock_t end_time = clock();
    double elapsed_time = (double)(end_time - start_time) / CLOCKS_PER_SEC;
    printf("Processed %zu bases in %.4f seconds\n", total_bases, elapsed_time);
    printf("%.2f million bases per second\n",
           total_bases / elapsed_time / 1000000.0);
  }

  // Clean up
  free(sbuf);
  free(xbuf);
  binseq_record_free(record);
  binseq_context_free(ctx);
  binseq_reader_close(reader);

  return 0;
}
