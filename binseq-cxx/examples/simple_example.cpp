#include <chrono>
#include <iostream>
#include <string>

#include "binseq-cxx/src/lib.rs.h"
#include "rust/cxx.h"

// Include the generated binseq C++ header

int main(int argc, char **argv) {
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <binseq_file>" << std::endl;
    return 1;
  }

  const std::string filename = argv[1];

  try {
    // Open the binseq file
    auto reader = open_mmap_reader(filename);

    // Get basic file info
    size_t num_records = reader->num_records();
    uint32_t slen = reader->get_slen();
    uint32_t xlen = reader->get_xlen();

    std::cout << "File: " << filename << std::endl;
    std::cout << "Records: " << num_records << std::endl;
    std::cout << "Primary Sequence length: " << slen << std::endl;

    if (xlen > 0) {
      std::cout << "Extended Sequence length: " << xlen << std::endl;
    } else {
      std::cout << "Single-end reads" << std::endl;
    }

    // Create reusable buffers for sequence decoding
    rust::Vec<uint8_t> sbuf;
    rust::Vec<uint8_t> xbuf;

    // Print the first few records
    size_t records_to_show = std::min(num_records, size_t(5));
    std::cout << "\nFirst " << records_to_show << " records:" << std::endl;
    std::cout << "------------------" << std::endl;

    for (size_t i = 0; i < records_to_show; i++) {
      // Get the record
      auto record = reader->get_record(i);
      uint64_t flag = record->get_flag();

      // Clear the buffers
      sbuf.clear();
      xbuf.clear();

      // Decode the primary sequence
      record->decode_s(sbuf);
      std::string s_str(sbuf.begin(), sbuf.end());

      std::cout << "Record " << i << " (flag=" << flag << "):" << std::endl;
      std::cout << "  Sequence (" << s_str.length() << " bp): " << s_str
                << std::endl;

      // If paired, decode the extended sequence too
      if (record->is_paired()) {
        record->decode_x(xbuf);
        std::string x_str(xbuf.begin(), xbuf.end());
        std::cout << "  Paired (" << x_str.length() << " bp): " << x_str
                  << std::endl;
      }
    }

    // Performance test
    if (num_records > 10) {
      std::cout << "\nPerformance test: processing all " << num_records
                << " records..." << std::endl;
      auto start_time = std::chrono::high_resolution_clock::now();

      size_t total_bases = 0;
      for (size_t i = 0; i < num_records; i++) {

        // Clear the buffers
        sbuf.clear();
        xbuf.clear();

        auto record = reader->get_record(i);

        // Decode and count bases
        record->decode_s(sbuf);
        total_bases += sbuf.size();

        if (record->is_paired()) {
          record->decode_x(xbuf);
          total_bases += xbuf.size();
        }
      }

      auto end_time = std::chrono::high_resolution_clock::now();
      auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(
          end_time - start_time);
      double seconds = duration.count() / 1000.0;

      std::cout << "Processed " << total_bases << " bases in " << seconds
                << " seconds" << std::endl;
      std::cout << (total_bases / seconds / 1000000.0)
                << " million bases per second" << std::endl;
    }

  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return 1;
  }

  return 0;
}
