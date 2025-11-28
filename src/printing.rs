// printing.rs - CUPS integration
// Phase 4: Printing Integration

use crate::layout::{Layout, PaperSize};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Represents a printer available on the system
#[derive(Debug, Clone, PartialEq)]
pub struct PrinterInfo {
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub state: PrinterState,
}

/// Printer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterState {
    Idle,
    Processing,
    Stopped,
    Unknown,
}

/// Print job configuration
#[derive(Debug, Clone)]
pub struct PrintJob {
    pub layout: Layout,
    pub printer_name: String,
    pub copies: u32,
    pub dpi: u32,
}

/// Page orientation (kept for backwards compatibility, but layout.page.orientation is preferred)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Print errors
#[derive(Debug)]
#[allow(dead_code)]
pub enum PrintError {
    NoPrinters,
    PrinterNotFound(String),
    PrinterOffline(String),
    CupsNotAvailable,
    RenderError(String),
    IoError(io::Error),
    CommandFailed(String),
}

impl std::fmt::Display for PrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintError::NoPrinters => write!(f, "No printers found on system"),
            PrintError::PrinterNotFound(name) => write!(f, "Printer '{}' not found", name),
            PrintError::PrinterOffline(name) => write!(f, "Printer '{}' is offline", name),
            PrintError::CupsNotAvailable => {
                write!(f, "CUPS printing system is not available or not running")
            }
            PrintError::RenderError(msg) => write!(f, "Failed to render layout: {}", msg),
            PrintError::IoError(e) => write!(f, "I/O error: {}", e),
            PrintError::CommandFailed(msg) => write!(f, "Print command failed: {}", msg),
        }
    }
}

impl std::error::Error for PrintError {}

impl From<io::Error> for PrintError {
    fn from(err: io::Error) -> Self {
        PrintError::IoError(err)
    }
}

/// Discover available printers using lpstat command
pub fn discover_printers() -> Result<Vec<PrinterInfo>, PrintError> {
    log::info!("Discovering printers via lpstat");

    // Check if CUPS is available
    let test = Command::new("lpstat").arg("-v").output();
    if test.is_err() {
        log::error!("lpstat command not available - CUPS may not be installed");
        return Err(PrintError::CupsNotAvailable);
    }

    // Get list of printers
    let output = Command::new("lpstat")
        .arg("-p")
        .arg("-d")
        .output()
        .map_err(|_| PrintError::CupsNotAvailable)?;

    if !output.status.success() {
        log::warn!("lpstat command failed, returning empty printer list");
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut printers = Vec::new();
    let mut default_printer = None;

    // Parse default printer
    for line in stdout.lines() {
        if line.starts_with("system default destination:") {
            default_printer = line.split(':').nth(1).map(|s| s.trim().to_string());
            break;
        }
    }

    // Parse printer list
    for line in stdout.lines() {
        if line.starts_with("printer ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[1].to_string();
                let is_default = default_printer.as_ref() == Some(&name);

                // Get printer state
                let state = if line.contains("idle") {
                    PrinterState::Idle
                } else if line.contains("processing") {
                    PrinterState::Processing
                } else if line.contains("stopped") {
                    PrinterState::Stopped
                } else {
                    PrinterState::Unknown
                };

                // Get description (everything after "is")
                let description = if let Some(pos) = line.find(" is ") {
                    line[pos + 4..].to_string()
                } else {
                    name.clone()
                };

                printers.push(PrinterInfo {
                    name,
                    description,
                    is_default,
                    state,
                });
            }
        }
    }

    log::info!("Found {} printers", printers.len());
    Ok(printers)
}

/// Get the default printer
#[allow(dead_code)]
pub fn get_default_printer() -> Result<Option<PrinterInfo>, PrintError> {
    let printers = discover_printers()?;
    Ok(printers.into_iter().find(|p| p.is_default))
}

/// Render layout to image buffer at specified DPI
pub fn render_layout_to_image(layout: &Layout, dpi: u32) -> Result<RgbaImage, PrintError> {
    log::info!("Rendering layout at {} DPI", dpi);

    // Calculate page dimensions in pixels
    let page = &layout.page;
    let width_px = ((page.width_mm / 25.4) * dpi as f32) as u32;
    let height_px = ((page.height_mm / 25.4) * dpi as f32) as u32;

    log::debug!(
        "Page dimensions: {}x{} mm -> {}x{} px at {} DPI (Orientation: {:?})",
        page.width_mm,
        page.height_mm,
        width_px,
        height_px,
        dpi,
        page.orientation
    );

    // Create white canvas
    let mut img: RgbaImage = ImageBuffer::from_pixel(width_px, height_px, Rgba([255, 255, 255, 255]));

    // Render each image
    for placed_image in &layout.images {
        // Load the source image - use ImageReader to ensure proper format handling
        let source_img = match load_image_for_print(&placed_image.path) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to load image {:?}: {}", placed_image.path, e);
                continue;
            }
        };

        // Calculate position and size in pixels
        let x_px = ((placed_image.x_mm / 25.4) * dpi as f32) as u32;
        let y_px = ((placed_image.y_mm / 25.4) * dpi as f32) as u32;
        let w_px = ((placed_image.width_mm / 25.4) * dpi as f32) as u32;
        let h_px = ((placed_image.height_mm / 25.4) * dpi as f32) as u32;

        // Resize source image to target dimensions
        let resized = source_img.resize_exact(w_px, h_px, image::imageops::FilterType::Lanczos3);

        // Composite onto canvas
        image::imageops::overlay(&mut img, &resized.to_rgba8(), x_px.into(), y_px.into());

        log::debug!(
            "Rendered image {} at ({}, {}) with size {}x{} px",
            placed_image.id,
            x_px,
            y_px,
            w_px,
            h_px
        );
    }

    // NOTE: We do NOT rotate the image here for landscape mode.
    // The page dimensions (width_mm, height_mm) are already swapped when the user
    // selects landscape orientation, so the canvas is already rendered correctly.
    // CUPS handles the physical paper orientation via the orientation-requested option.

    Ok(img)
}

/// Load an image for printing with proper format handling
/// This handles all supported formats including GIF (first frame only)
fn load_image_for_print(path: &PathBuf) -> Result<image::DynamicImage, PrintError> {
    // Use ImageReader for more robust format detection
    let reader = image::ImageReader::open(path)
        .map_err(|e| PrintError::RenderError(format!("Cannot open image: {}", e)))?
        .with_guessed_format()
        .map_err(|e| PrintError::RenderError(format!("Cannot detect format: {}", e)))?;
    
    log::debug!("Loading image {:?}, detected format: {:?}", path, reader.format());
    
    let img = reader.decode()
        .map_err(|e| PrintError::RenderError(format!("Cannot decode image: {}", e)))?;
    
    Ok(img)
}

/// Send a print job to the specified printer
pub fn send_to_printer(job: &PrintJob, temp_file: &Path) -> Result<String, PrintError> {
    log::info!(
        "Sending print job to printer '{}' with {} copies",
        job.printer_name,
        job.copies
    );

    // Verify printer exists
    let printers = discover_printers()?;
    if !printers.iter().any(|p| p.name == job.printer_name) {
        return Err(PrintError::PrinterNotFound(job.printer_name.clone()));
    }

    // Build lp command
    let mut cmd = Command::new("lp");
    cmd.arg("-d").arg(&job.printer_name);
    cmd.arg("-n").arg(job.copies.to_string());

    // NOTE: We do NOT set orientation-requested or landscape options here.
    // Our rendered image already has the correct dimensions (width/height swapped for landscape).
    // The image is ready to print as-is. Setting CUPS orientation would cause double-rotation.
    // We just need to tell CUPS the correct media size.

    // Add paper size option - use the actual dimensions we rendered
    // For landscape, width > height, so we specify the media accordingly
    let paper_option = match job.layout.page.paper_size {
        PaperSize::A4 => "media=A4",
        PaperSize::A3 => "media=A3",
        PaperSize::A5 => "media=A5",
        PaperSize::Letter => "media=Letter",
        PaperSize::Legal => "media=Legal",
        PaperSize::Tabloid => "media=Tabloid",
        PaperSize::Ledger => "media=Ledger",
        PaperSize::Photo4x6 => "media=4x6",
        PaperSize::Photo5x7 => "media=5x7",
        PaperSize::Photo8x10 => "media=8x10",
        PaperSize::Photo11x17 => "media=11x17",
        PaperSize::Photo13x19 => "media=13x19",
        // For custom sizes, try to use closest standard or specify dimensions
        _ => {
            // Use custom size in mm
            let w = job.layout.page.width_mm;
            let h = job.layout.page.height_mm;
            log::debug!("Using custom media size: {}x{}mm", w, h);
            "media=A4" // Fallback to A4, most printers support it
        }
    };
    cmd.arg("-o").arg(paper_option);
    
    // For proper scaling, tell CUPS to fit the image to the page
    cmd.arg("-o").arg("fit-to-page");

    // Add the file to print
    cmd.arg(temp_file);

    log::debug!("Executing: {:?}", cmd);

    // Execute print command
    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Print command failed: {}", stderr);
        return Err(PrintError::CommandFailed(stderr.to_string()));
    }

    // Parse job ID from output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let job_id = stdout
        .split_whitespace()
        .find(|s| s.starts_with("request"))
        .and_then(|s| s.split('-').next_back())
        .unwrap_or("unknown")
        .to_string();

    log::info!("Print job submitted successfully: {}", job_id);
    Ok(job_id)
}

/// Create a temporary file for printing
pub fn create_temp_print_file(img: &RgbaImage) -> Result<PathBuf, PrintError> {
    let temp_dir = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temp_path = temp_dir.join(format!("print_layout_{}.png", timestamp));

    log::debug!("Creating temporary print file: {:?}", temp_path);

    img.save(&temp_path)
        .map_err(|e| PrintError::RenderError(format!("Failed to save temporary file: {}", e)))?;

    Ok(temp_path)
}

/// Execute a complete print job
pub fn execute_print_job(job: PrintJob) -> Result<String, PrintError> {
    log::info!("Executing print job");

    // Render layout to image
    let img = render_layout_to_image(&job.layout, job.dpi)?;

    // Save to temporary file
    let temp_file = create_temp_print_file(&img)?;

    // Send to printer
    let job_id = send_to_printer(&job, &temp_file)?;

    // Note: Temporary file cleanup should be handled by caller
    // after confirming successful print submission

    Ok(job_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer_discovery() {
        // This test will only work on systems with CUPS installed
        match discover_printers() {
            Ok(printers) => {
                println!("Found {} printers", printers.len());
                for printer in printers {
                    println!("  - {}: {:?}", printer.name, printer.state);
                }
            }
            Err(e) => {
                println!(
                    "Printer discovery failed (expected on systems without CUPS): {}",
                    e
                );
            }
        }
    }
}
