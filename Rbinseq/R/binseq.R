#' @useDynLib Rbinseq, .registration = TRUE
NULL

#' Read a BinSeq file
#'
#' This function reads and displays information about a BinSeq file, including
#' file statistics and a preview of the first few records.
#'
#' @param file Path to the BinSeq file
#' @return Invisibly returns NULL (the function is used for its side effects of printing information)
#' @export
#' @examples
#' \dontrun{
#' # If you have a sample.binseq file:
#' read_binseq("path/to/sample.binseq")
#' }
read_binseq <- function(file) {
    # Validate input
    if (!is.character(file) || length(file) != 1) {
        stop("file argument must be a single string")
    }

    if (!file.exists(file)) {
        stop("File does not exist: ", file)
    }

    # Call the C function without assigning the result
    # since it returns NULL (the function is used for its side effects)
    .Call("C_read_binseq", file)

    # Return invisibly (function is primarily used for its output)
    invisible(NULL)
}

#' @rdname read_binseq
#' @export
readBinseq <- read_binseq # Alias for convenience

#' Test reading the sample BinSeq file
#'
#' This function demonstrates how to use the readBinseq function with the
#' included sample file in the package.
#'
#' @return Invisibly returns NULL
#' @export
#' @examples
#' test_readBinseq()
test_readBinseq <- function() {
    # Get the path to the example file
    sample_file <- system.file("extdata", "subset.bq", package = "Rbinseq")

    if (file.exists(sample_file)) {
        message("Reading sample BinSeq file: ", sample_file)
        readBinseq(sample_file)
    } else {
        stop("Sample BinSeq file not found. The package may not be installed correctly.")
    }

    # Return invisibly
    invisible(NULL)
}
