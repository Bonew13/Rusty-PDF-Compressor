# PDF Compressor

## Overview

This simple, offline PDF compressor built in Rust uses Ghostscript to compress PDF files. The idea behind this tool is to provide a secure and free way to compress PDFs without relying on potentially untrustworthy online services or expensive tools like Adobe Acrobat.

### Key Features
- Compress PDF files directly on your local machine.
- Avoid the risks of uploading sensitive documents to online services.
- Generate compressed PDFs with the suffix `_compressed` added to the file name.
- Easy-to-use file selection interface.

---

## Requirements

1. **Rust**: Ensure you have Rust installed on your system. You can install Rust via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Ghostscript**: This tool relies on Ghostscript for the actual PDF compression. Install it using the following instructions:

   ### Linux (Ubuntu/Debian):
   ```bash
   sudo apt update
   sudo apt install ghostscript
   ```

   ### macOS:
   Install Ghostscript via Homebrew:
   ```bash
   brew install ghostscript
   ```

   ### Windows (WSL or Native):
   - Install Ghostscript for Windows from the [official Ghostscript site](https://www.ghostscript.com/).
   - Add the Ghostscript binary (`gswin64c.exe` or `gs`) to your system's PATH.

   Verify installation:
   ```bash
   gs --version
   ```
   This should output the installed Ghostscript version.

---

## Installation and Usage

### 1. Clone the Repository
Clone the repository containing this project to your local machine:
```bash
git clone <repository-url>
cd <repository-directory>
```

### 2. Build the Program
Compile the Rust code:
```bash
cargo build --release
```
This will generate an executable in the `target/release` directory.

### 3. Run the Program
Run the program from the directory where your PDFs are located:
```bash
./target/release/pdf_compressor
```
The program will:
1. List all PDF files in the current directory.
2. Allow you to select a file by its index.
3. Compress the selected file and save it with `_compressed` appended to the original file name.

---

## Example Usage

1. Place your PDFs in the directory where the program will run.
2. Execute the program:
   ```bash
   ./target/release/pdf_compressor
   ```
3. Example output:
   ```
   Listing all files in the root directory '/path/to/your/directory':
   Found PDF: /path/to/your/directory/document.pdf
   Found PDF: /path/to/your/directory/another_file.pdf

   Enter the number of the PDF file you want to compress:
   1: /path/to/your/directory/document.pdf
   2: /path/to/your/directory/another_file.pdf

   1
   Input file: /path/to/your/directory/document.pdf
   Output file: /path/to/your/directory/document_compressed.pdf
   Compressing PDF using Ghostscript...
   PDF compressed successfully and saved to /path/to/your/directory/document_compressed.pdf
   ```

---

## Notes

- The compression level can be adjusted by modifying the Ghostscript `-dPDFSETTINGS` parameter in the code:
  - `/screen`: Lower quality and smallest file size.
  - `/ebook`: Medium quality (default).
  - `/printer`: High quality for printing.
  - `/prepress`: Maximum quality for publishing.

- Ensure you have sufficient permissions to read/write files in the directory where the program runs.

---

## Why This Tool Exists

This project was created out of skepticism towards online PDF compressors and frustration with expensive tools like Adobe Acrobat. With this offline, open-source tool, you can compress your PDFs securely and without extra costs.

---

## License
This project is open-source and available under the MIT License. Feel free to modify and distribute it as you like.

---

## Contributions
Contributions are welcome! If you have suggestions or improvements, feel free to open an issue or submit a pull request.

