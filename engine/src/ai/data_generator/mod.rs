//! AI Data Generator
//! "Give me 100 rows with Nepal phone numbers, Indian cities, random gender"

use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;

/// Represents a generated data record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: u32,
    pub phone: String,     // Nepal phone: 98XXXXXXXX
    pub city: String,      // Indian city
    pub gender: String,    // "Male", "Female", "Other"
}

/// AI Data Generator - creates realistic test data
pub struct AIDataGenerator {
    rng: rand::rngs::ThreadRng,
    indian_cities: Vec<&'static str>,
}

impl AIDataGenerator {
    /// Create a new data generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            indian_cities: vec![
                "Mumbai", "Delhi", "Bangalore", "Hyderabad", "Ahmedabad",
                "Chennai", "Kolkata", "Surat", "Pune", "Jaipur",
                "Lucknow", "Kanpur", "Nagpur", "Indore", "Thane",
                "Bhopal", "Visakhapatnam", "Pimpri-Chinchwad", "Patna", "Vadodara",
            ],
        }
    }
    
    /// Generate a Nepal phone number (98XXXXXXXX format)
    pub fn generate_nepal_phone(&mut self) -> String {
        // Nepal mobile numbers start with 98, 97, 96
        let prefix = self.rng.gen_range(98..=99); // 98 or 99 for now
        let suffix = self.rng.gen_range(10000000..=99999999); // 8 digits
        
        format!("{}{}", prefix, suffix)
    }
    
    /// Generate a random Indian city
    pub fn generate_indian_city(&mut self) -> String {
        let index = self.rng.gen_range(0..self.indian_cities.len());
        self.indian_cities[index].to_string()
    }
    
    /// Generate random gender
    pub fn generate_gender(&mut self) -> String {
        let options = ["Male", "Female", "Other"];
        let index = self.rng.gen_range(0..options.len());
        options[index].to_string()
    }
    
    /// Generate a single data record
    pub fn generate_record(&mut self, id: u32) -> DataRecord {
        DataRecord {
            id,
            phone: self.generate_nepal_phone(),
            city: self.generate_indian_city(),
            gender: self.generate_gender(),
        }
    }
    
    /// Generate multiple records
    pub fn generate_records(&mut self, count: u32) -> Vec<DataRecord> {
        (1..=count)
            .map(|id| self.generate_record(id))
            .collect()
    }
    
    /// Parse natural language request and generate data
    /// Example: "100 rows with Nepal phone numbers, Indian cities, random gender"
    pub fn generate_from_request(&mut self, request: &str) -> Result<Vec<DataRecord>, String> {
        // Simple parsing for now - extract number
        let words: Vec<&str> = request.split_whitespace().collect();
        
        // Find the number in the request
        let mut count = 10; // Default
        for word in &words {
            if let Ok(num) = word.parse::<u32>() {
                count = num;
                break;
            }
        }
        
        // Check if request mentions our supported features
        let has_phone = request.to_lowercase().contains("phone") || 
                        request.to_lowercase().contains("nepal");
        let has_city = request.to_lowercase().contains("city") || 
                       request.to_lowercase().contains("india");
        let has_gender = request.to_lowercase().contains("gender");
        
        if !has_phone && !has_city && !has_gender {
            return Err("Request should mention phone, city, or gender".to_string());
        }
        
        Ok(self.generate_records(count))
    }
    
    /// Export records to CSV file
    pub fn export_csv(&self, records: &[DataRecord], filename: &str) -> Result<(), String> {
        let mut file = fs::File::create(filename)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        // Write header
        writeln!(file, "ID,Phone,City,Gender")
            .map_err(|e| format!("Failed to write header: {}", e))?;
        
        // Write records
        for record in records {
            writeln!(file, "{},{},{},{}", 
                     record.id, record.phone, record.city, record.gender)
                .map_err(|e| format!("Failed to write record: {}", e))?;
        }
        
        Ok(())
    }
    
    /// Export records to JSON file
    pub fn export_json(&self, records: &[DataRecord], filename: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(records)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        
        fs::write(filename, json)
            .map_err(|e| format!("Failed to write JSON file: {}", e))?;
        
        Ok(())
    }
    
    /// Display records in a table format
    pub fn display_table(&self, records: &[DataRecord]) {
        println!("┌─────┬──────────────┬────────────────────┬────────┐");
        println!("│ ID  │ Phone        │ City               │ Gender │");
        println!("├─────┼──────────────┼────────────────────┼────────┤");
        
        for record in records.iter().take(10) { // Show first 10
            println!("│ {:3} │ {:12} │ {:18} │ {:6} │", 
                     record.id, record.phone, record.city, record.gender);
        }
        
        if records.len() > 10 {
            println!("├─────┼──────────────┼────────────────────┼────────┤");
            println!("│ ... and {} more records", records.len() - 10);
        }
        
        println!("└─────┴──────────────┴────────────────────┴────────┘");
    }
}
impl std::fmt::Display for DataRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)  // Use Debug representation
    }
}