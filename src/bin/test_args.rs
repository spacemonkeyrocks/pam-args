use pam_args::{Flag, KeyValue, AllowedKeyValueFormats};
use std::str::FromStr;

fn main() {
    // Create flag and key-value arguments
    let debug_flag = Flag::new("DEBUG", "Enable debug mode");
    let verbose_flag = Flag::new("VERBOSE", "Enable verbose output")
        .depends_on("DEBUG");
    
    let username_kv = KeyValue::new("USER", "Username for authentication")
        .required();
    
    let port_kv = KeyValue::new("PORT", "Port number")
        .type_converter(i32::from_str);
    
    // Print the argument definitions
    println!("Flag: {}", debug_flag.name());
    println!("Flag description: {}", debug_flag.description());
    println!("Flag has binding: {}", debug_flag.has_binding());
    println!("Flag dependencies: {:?}", debug_flag.dependencies());
    println!("Flag exclusions: {:?}", debug_flag.exclusions());
    
    println!("\nVerbose Flag: {}", verbose_flag.name());
    println!("Verbose Flag dependencies: {:?}", verbose_flag.dependencies());
    
    println!("\nKeyValue: {}", username_kv.name());
    println!("KeyValue description: {}", username_kv.description());
    println!("KeyValue is required: {}", username_kv.is_required());
    println!("KeyValue has binding: {}", username_kv.has_binding());
    println!("KeyValue dependencies: {:?}", username_kv.dependencies());
    println!("KeyValue exclusions: {:?}", username_kv.exclusions());
    
    println!("\nPort KeyValue: {}", port_kv.name());
    println!("Port KeyValue has type converter: {}", port_kv.has_type_converter());
    
    // Test allowed values
    let align_kv = KeyValue::new("ALIGN", "Text alignment")
        .allowed_values(&["LEFT", "CENTER", "RIGHT"]);
    
    println!("\nAlign KeyValue: {}", align_kv.name());
    println!("Align allowed values: {:?}", align_kv.get_allowed_values());
    println!("Is 'LEFT' allowed? {}", align_kv.is_value_allowed("LEFT"));
    println!("Is 'BOTTOM' allowed? {}", align_kv.is_value_allowed("BOTTOM"));
    
    // Test allowed formats
    let debug_kv = KeyValue::new("DEBUG", "Debug mode")
        .allowed_formats(&[
            AllowedKeyValueFormats::KeyOnly,
            AllowedKeyValueFormats::KeyValue,
        ]);
    
    println!("\nDebug KeyValue: {}", debug_kv.name());
    println!("Debug allowed formats: {:?}", debug_kv.get_allowed_formats());
}