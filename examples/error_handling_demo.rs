/// Demonstration of comprehensive error handling and validation in boxen
use ::boxen::{BoxenBuilder, BoxenError};

fn main() {
    println!("=== Boxen Error Handling and Validation Demo ===\n");

    // Example 1: Valid configuration
    println!("1. Valid configuration:");
    match BoxenBuilder::new()
        .width(50)
        .height(8)
        .padding(2)
        .title("Success!")
        .border_color("green")
        .render("This configuration is valid and should work perfectly.")
    {
        Ok(result) => println!("{}\n", result),
        Err(e) => println!("Error: {}\n", e.detailed_message()),
    }

    // Example 2: Invalid configuration with detailed error messages
    println!("2. Invalid configuration with detailed error messages:");
    match BoxenBuilder::new()
        .width(5) // Too small
        .padding(10) // Excessive padding
        .title("a".repeat(250)) // Very long title
        .border_color("invalid_color") // Invalid color
        .render("This will fail validation")
    {
        Ok(result) => println!("{}\n", result),
        Err(e) => {
            println!("Error occurred:");
            println!("{}\n", e.detailed_message());

            // Show recommendations
            let recommendations = e.recommendations();
            if !recommendations.is_empty() {
                println!("Available recommendations:");
                for (i, rec) in recommendations.iter().enumerate() {
                    println!("{}. {}: {}", i + 1, rec.issue, rec.suggestion);
                    if let Some(auto_fix) = &rec.auto_fix {
                        println!("   Auto-fix: {}", auto_fix);
                    }
                }
                println!();
            }
        }
    }

    // Example 3: Auto-recovery demonstration
    println!("3. Auto-recovery demonstration:");
    match BoxenBuilder::new()
        .width(8) // Small width
        .padding(3) // Large padding that would normally cause issues
        .render_or_adjust("This should auto-adjust to work")
    {
        Ok(result) => {
            println!("Auto-recovery succeeded:");
            println!("{}\n", result);
        }
        Err(e) => println!("Auto-recovery failed: {}\n", e.detailed_message()),
    }

    // Example 4: Validation without rendering
    println!("4. Configuration validation without rendering:");
    let builder = BoxenBuilder::new()
        .width(30)
        .height(10)
        .padding(1)
        .title("Validation Test");

    match builder.validate() {
        Ok(_) => println!("✓ Configuration is valid\n"),
        Err(e) => println!("✗ Configuration is invalid: {}\n", e.detailed_message()),
    }

    // Example 5: Detailed configuration analysis
    println!("5. Detailed configuration analysis:");
    let analysis_builder = BoxenBuilder::new()
        .width(40)
        .height(6)
        .padding(2)
        .title("Analysis Demo");

    let analysis = analysis_builder.check_configuration("Sample text for analysis");
    println!("{}", analysis);

    // Example 6: Different error types
    println!("6. Demonstrating different error types:");

    // Input validation error
    println!("a) Input validation error:");
    match BoxenBuilder::new().render("a".repeat(1_000_001)) {
        Ok(_) => println!("Unexpectedly succeeded"),
        Err(e) => match e {
            BoxenError::InputValidationError { field, .. } => {
                println!("Input validation failed for field: {}", field);
            }
            BoxenError::RenderingError { .. } => {
                println!("Rendering error (wrapping input validation)");
            }
            _ => println!("Other error type: {}", e),
        },
    }

    // Dimension validation error
    println!("\nb) Dimension validation error:");
    match BoxenBuilder::new().width(0).render("Test") {
        Ok(_) => println!("Unexpectedly succeeded"),
        Err(e) => match e {
            BoxenError::InputValidationError { field, value, .. } => {
                println!("Dimension validation failed: {} = {}", field, value);
            }
            BoxenError::RenderingError { .. } => {
                println!("Rendering error (wrapping dimension validation)");
            }
            _ => println!("Other error type: {}", e),
        },
    }

    // Color validation error
    println!("\nc) Color validation error:");
    match BoxenBuilder::new()
        .border_color("nonexistent_color")
        .render("Test")
    {
        Ok(_) => println!("Unexpectedly succeeded"),
        Err(e) => match e {
            BoxenError::InvalidColor { color_value, .. } => {
                println!("Color validation failed for: {}", color_value);
            }
            BoxenError::InputValidationError { .. } => {
                println!("Input validation error (wrapping color validation)");
            }
            BoxenError::RenderingError { .. } => {
                println!("Rendering error (wrapping color validation)");
            }
            _ => println!("Other error type: {}", e),
        },
    }

    println!("\n=== Demo Complete ===");
}
