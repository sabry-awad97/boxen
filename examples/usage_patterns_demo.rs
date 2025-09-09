/// Demonstration of common usage patterns and best practices for boxen
use ::boxen::{
    BorderStyle, TextAlignment, TitleAlignment, builder, double_box, round_box, simple_box,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Boxen Usage Patterns Demo ===\n");

    // 1. Quick and simple patterns
    demonstrate_quick_patterns()?;

    // 2. CLI application patterns
    demonstrate_cli_patterns()?;

    // 3. Configuration and settings display
    demonstrate_config_patterns()?;

    // 4. Status and notification patterns
    demonstrate_status_patterns()?;

    // 5. Data presentation patterns
    demonstrate_data_patterns()?;

    // 6. Interactive application patterns
    demonstrate_interactive_patterns()?;

    // 7. Logging and debugging patterns
    demonstrate_logging_patterns()?;

    // 8. Documentation and help patterns
    demonstrate_documentation_patterns()?;

    println!("\n=== Usage Patterns Demo Complete ===");
    Ok(())
}

fn demonstrate_quick_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Quick and Simple Patterns:");

    // One-liner for simple messages
    println!("Quick message:");
    println!("{}", simple_box("Quick message"));

    // Emphasis with double border
    println!("Important message:");
    println!("{}", double_box("⚠️  Important: Please read carefully"));

    // Friendly rounded corners
    println!("Friendly notification:");
    println!("{}", round_box("✨ Welcome to our application!"));

    // Builder for slightly more complex needs
    println!("Builder one-liner:");
    println!(
        "{}",
        builder()
            .border_color("green")
            .padding(1)
            .render("✓ Success: Operation completed")?
    );

    println!();
    Ok(())
}

fn demonstrate_cli_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. CLI Application Patterns:");

    // Command help display
    println!("Command help:");
    let help_text = "USAGE:\n    myapp [OPTIONS] <FILE>\n\nOPTIONS:\n    -h, --help       Print help information\n    -v, --verbose    Enable verbose output\n    -o, --output     Output file path";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Single)
            .title("Help")
            .title_alignment(TitleAlignment::Center)
            .padding(1)
            .width(50)
            .render(help_text)?
    );

    // Progress indication
    println!("Progress display:");
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .border_color("blue")
            .title("⏳ Processing")
            .padding(1)
            .render("████████░░ 80% complete\nProcessing file: data.csv\nETA: 30 seconds")?
    );

    // Error reporting
    println!("Error reporting:");
    println!("{}", builder()
        .border_style(BorderStyle::Bold)
        .border_color("red")
        .title("❌ Error")
        .padding(1)
        .render("Failed to open file: permission denied\n\nSuggestion: Check file permissions\nRun with: sudo myapp file.txt")?);

    // Success confirmation
    println!("Success confirmation:");
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Double)
            .border_color("green")
            .title("✅ Success")
            .padding(1)
            .text_alignment(TextAlignment::Center)
            .render("File processed successfully!\n\n3 records updated\n1 new record created")?
    );

    println!();
    Ok(())
}

fn demonstrate_config_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Configuration and Settings Display:");

    // System configuration
    println!("System configuration:");
    let config_text = "Database URL: postgresql://localhost:5432/mydb\nRedis URL: redis://localhost:6379\nLog Level: INFO\nDebug Mode: false\nPort: 8080\nEnvironment: production";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Single)
            .title("🔧 Configuration")
            .title_alignment(TitleAlignment::Left)
            .padding(2)
            .width(55)
            .render(config_text)?
    );

    // Environment variables
    println!("Environment variables:");
    let env_text = "NODE_ENV=production\nPORT=3000\nDATABASE_URL=***hidden***\nAPI_KEY=***hidden***\nDEBUG=false";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .border_color("cyan")
            .title("Environment")
            .padding(1)
            .margin(1)
            .render(env_text)?
    );

    // Feature flags
    println!("Feature flags:");
    let features_text = "✅ New Dashboard: enabled\n❌ Beta Features: disabled\n✅ Analytics: enabled\n⚠️  Experimental API: testing\n✅ Auto-backup: enabled";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Double)
            .title("🚩 Feature Flags")
            .title_alignment(TitleAlignment::Center)
            .padding(1)
            .width(40)
            .render(features_text)?
    );

    println!();
    Ok(())
}

fn demonstrate_status_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("4. Status and Notification Patterns:");

    // System status dashboard
    println!("System status:");
    let status_text = "🟢 API Server: Online\n🟢 Database: Connected\n🟡 Cache: Degraded (high latency)\n🔴 Email Service: Offline\n🟢 File Storage: Available";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Bold)
            .title("📊 System Status")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .width(45)
            .text_alignment(TextAlignment::Left)
            .render(status_text)?
    );

    // Health check results
    println!("Health check:");
    println!("{}", builder()
        .border_style(BorderStyle::Round)
        .border_color("green")
        .background_color("black")
        .title("💚 Health Check")
        .padding(1)
        .render("All systems operational\nResponse time: 45ms\nUptime: 99.9%\nLast check: 2 seconds ago")?);

    // Warning notification
    println!("Warning notification:");
    println!("{}", builder()
        .border_style(BorderStyle::SingleDouble)
        .border_color("yellow")
        .title("⚠️  Warning")
        .padding(1)
        .render("Disk space is running low\n\nCurrent usage: 85%\nRecommended action: Clean up old logs")?);

    // Critical alert
    println!("Critical alert:");
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::DoubleSingle)
            .border_color("red")
            .background_color("black")
            .title("🚨 CRITICAL ALERT")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .text_alignment(TextAlignment::Center)
            .render(
                "SYSTEM OVERLOAD DETECTED\n\nCPU: 95%\nMemory: 98%\nImmediate action required!"
            )?
    );

    println!();
    Ok(())
}

fn demonstrate_data_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("5. Data Presentation Patterns:");

    // Table-like data
    println!("Tabular data:");
    let table_text = "Name          | Status    | Last Seen\n──────────────┼───────────┼──────────────\nAlice Johnson | Online    | 2 min ago\nBob Smith     | Away      | 15 min ago\nCarol Davis   | Offline   | 2 hours ago\nDave Wilson   | Online    | Just now";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Single)
            .title("👥 User Status")
            .padding(1)
            .render(table_text)?
    );

    // Statistics display
    println!("Statistics:");
    let stats_text = "📈 Performance Metrics\n\nRequests/sec:     1,247\nAvg Response:     125ms\nError Rate:       0.02%\nActive Users:     3,456\nMemory Usage:     67%";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Double)
            .border_color("blue")
            .padding(2)
            .width(35)
            .render(stats_text)?
    );

    // Key-value pairs
    println!("Configuration values:");
    let kv_text = "server.host ................. localhost\nserver.port ................. 8080\ndb.connections.max .......... 100\ndb.connections.timeout ...... 30s\ncache.ttl ................... 3600s\nlog.level ................... INFO";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .title("⚙️  Settings")
            .padding(1)
            .width(50)
            .render(kv_text)?
    );

    println!();
    Ok(())
}

fn demonstrate_interactive_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("6. Interactive Application Patterns:");

    // Menu display
    println!("Menu interface:");
    let menu_text = "1. Create new project\n2. Open existing project\n3. Import from Git\n4. Settings\n5. Exit\n\nPress number to select option...";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .title("📋 Main Menu")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .width(35)
            .render(menu_text)?
    );

    // Form display
    println!("Form interface:");
    let form_text = "Username: [________________]\nPassword: [****************]\nEmail:    [________________]\n\n[ ] Remember me\n[x] Accept terms\n\n[Submit] [Cancel]";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Double)
            .title("🔐 Login Form")
            .padding(2)
            .width(35)
            .render(form_text)?
    );

    // Dialog box
    println!("Confirmation dialog:");
    let dialog_text = "Are you sure you want to delete\nthis file?\n\nThis action cannot be undone.\n\n[Yes] [No] [Cancel]";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Bold)
            .border_color("red")
            .title("⚠️  Confirm Delete")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .text_alignment(TextAlignment::Center)
            .width(40)
            .render(dialog_text)?
    );

    println!();
    Ok(())
}

fn demonstrate_logging_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("7. Logging and Debugging Patterns:");

    // Debug information
    println!("Debug output:");
    let debug_text = "Function: process_data()\nInput: 1,247 records\nFiltered: 1,203 valid records\nProcessed: 1,203 records\nOutput: success.json\nDuration: 2.34s";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Single)
            .border_color("cyan")
            .title("🐛 Debug Info")
            .padding(1)
            .render(debug_text)?
    );

    // Error trace
    println!("Error trace:");
    let trace_text = "ERROR: Database connection failed\n\nStack trace:\n  at connect() line 45\n  at init() line 23\n  at main() line 12\n\nCause: Connection timeout after 30s";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Bold)
            .border_color("red")
            .title("💥 Error Trace")
            .padding(1)
            .width(45)
            .render(trace_text)?
    );

    // Performance metrics
    println!("Performance log:");
    let perf_text = "⏱️  Execution Time: 1.23s\n📊 Memory Peak: 45.2 MB\n🔄 Cache Hits: 89%\n📈 Throughput: 2,456 ops/sec\n🎯 Success Rate: 99.8%";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .border_color("green")
            .title("Performance")
            .padding(1)
            .render(perf_text)?
    );

    println!();
    Ok(())
}

fn demonstrate_documentation_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("8. Documentation and Help Patterns:");

    // API documentation
    println!("API documentation:");
    let api_text = "GET /api/users/{id}\n\nReturns user information by ID.\n\nParameters:\n  id (required): User ID\n\nResponse: User object\nStatus: 200 OK | 404 Not Found";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Single)
            .title("📚 API Reference")
            .padding(2)
            .width(50)
            .render(api_text)?
    );

    // Quick reference
    println!("Quick reference:");
    let ref_text = "Keyboard Shortcuts:\n\nCtrl+N    New file\nCtrl+O    Open file\nCtrl+S    Save file\nCtrl+Z    Undo\nCtrl+Y    Redo\nF1        Help";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Round)
            .title("⌨️  Shortcuts")
            .padding(1)
            .render(ref_text)?
    );

    // Tips and tricks
    println!("Tips display:");
    let tips_text = "💡 Pro Tip:\n\nUse Ctrl+Shift+P to open the\ncommand palette for quick access\nto all available commands.\n\nTry typing 'help' to see more tips!";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Double)
            .border_color("yellow")
            .title("Tips & Tricks")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .width(45)
            .text_alignment(TextAlignment::Left)
            .render(tips_text)?
    );

    // Version information
    println!("Version info:");
    let version_text = "MyApp v2.1.0\nBuild: 20241201-abc123\nRust: 1.70.0\nOS: Windows 11\n\n© 2024 MyCompany\nLicense: MIT";
    println!(
        "{}",
        builder()
            .border_style(BorderStyle::Classic)
            .title("ℹ️  About")
            .title_alignment(TitleAlignment::Center)
            .padding(2)
            .text_alignment(TextAlignment::Center)
            .width(30)
            .render(version_text)?
    );

    println!();
    Ok(())
}
