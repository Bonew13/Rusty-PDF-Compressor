use std::env; // Import env for getting the current directory
use std::fs; // Import fs for working with files and directories
use std::io::stdin; // Import input/output traits for reading user input
use std::path::{Path}; // Import Path and PathBuf for handling file paths
use std::process::Command; // Import Command for executing Ghostscript

fn main() {
    // Get the current working directory as the root directory
    let root_directory = env::current_dir().expect("Failed to get the current directory");

    // Check if the root directory exists
    if !root_directory.exists() || !root_directory.is_dir() {
        eprintln!("Error: Root directory '{}' does not exist or is not a directory.", root_directory.display());
        return;
    }

    // List all files in the root directory
    println!("Listing all files in the root directory '{}':", root_directory.display());
    let entries = match fs::read_dir(&root_directory) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading root directory: {}", err);
            return;
        }
    };

    let mut pdf_files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "pdf" {
                        println!("Found PDF: {}", path.display());
                        pdf_files.push(path);
                    } else {
                        println!("Skipping non-PDF file: {}", path.display());
                    }
                }
            }
        }
    }

    // If no PDFs are found, exit
    if pdf_files.is_empty() {
        eprintln!("No PDF files found in the root directory.");
        return;
    }

    // Ask the user to select a PDF file by index
    println!("Enter the number of the PDF file you want to compress:");
    for (index, pdf) in pdf_files.iter().enumerate() {
        println!("{}: {}", index + 1, pdf.display());
    }

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().parse::<usize>();

    let selected_pdf = match input {
        Ok(index) if index > 0 && index <= pdf_files.len() => &pdf_files[index - 1],
        _ => {
            eprintln!("Invalid selection.");
            return;
        }
    };

    // Define the output path based on the input file name
    let output_path = selected_pdf
        .parent()
        .unwrap()
        .join(format!("{}_compressed.pdf", selected_pdf.file_stem().unwrap().to_string_lossy()));

    println!("Input file: {}", selected_pdf.display());
    println!("Output file: {}", output_path.display());

    // Use Ghostscript to compress the selected PDF
    println!("Compressing PDF using Ghostscript...");
    let status = Command::new("gs")
        .arg("-sDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dPDFSETTINGS=/ebook") // Change to `/screen`, `/printer`, or `/prepress` for other compression levels
        .arg("-dNOPAUSE")
        .arg("-dQUIET")
        .arg("-dBATCH")
        .arg(format!("-sOutputFile={}", output_path.display()))
        .arg(selected_pdf.to_str().unwrap())
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("PDF compressed successfully and saved to {}", output_path.display());
        }
        Ok(status) => {
            eprintln!("Ghostscript exited with status code {}", status.code().unwrap_or(-1));
        }
        Err(err) => {
            eprintln!("Failed to execute Ghostscript: {}", err);
        }
    }
}
