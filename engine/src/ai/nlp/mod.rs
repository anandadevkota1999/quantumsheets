//! Natural Language to Excel Formula translator
//! "add A1 and F6" → "=A1+F6"

use regex::{Regex, RegexBuilder};

/// Translates natural language to Excel formulas
pub struct NaturalLanguageTranslator {
    patterns: Vec<(Regex, Box<dyn Fn(&regex::Captures) -> String>)>,
}

impl NaturalLanguageTranslator {
    /// Create a new translator with built-in patterns
    pub fn new() -> Self {
        let mut translator = Self {
            patterns: Vec::new(),
        };
        
        translator.add_patterns();
        translator
    }
    
    /// Add all translation patterns
    fn add_patterns(&mut self) {
        // Pattern: "add A1 and B2" → "=A1+B2"
        self.add_pattern(
            r"(?:add|sum|plus)\s+([A-Za-z]+\d+)\s+(?:and|to|with)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("={}+{}", cell1, cell2)
            },
        );
        
        // Pattern: "subtract B2 from A1" → "=A1-B2"
        self.add_pattern(
            r"(?:subtract|minus|take away)\s+([A-Za-z]+\d+)\s+(?:from)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("={}-{}", cell2, cell1)  // Note: subtract B2 from A1 = A1-B2
            },
        );
        
        // Pattern: "multiply A1 by B2" → "=A1*B2"
        self.add_pattern(
            r"(?:multiply|times)\s+([A-Za-z]+\d+)\s+(?:by|with)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("={}*{}", cell1, cell2)
            },
        );
        
        // Pattern: "divide A1 by B2" → "=A1/B2"
        self.add_pattern(
            r"(?:divide)\s+([A-Za-z]+\d+)\s+(?:by)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("={}/{}", cell1, cell2)
            },
        );
        
        // Pattern: "sum of A1 to A10" → "=SUM(A1:A10)"
        self.add_pattern(
            r"(?:sum|total)\s+(?:of|for)\s+([A-Za-z]+\d+)\s+(?:to|through)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("=SUM({}:{})", cell1, cell2)
            },
        );
        
        // Pattern: "average of A1 to A10" → "=AVERAGE(A1:A10)"
        self.add_pattern(
            r"(?:average|mean)\s+(?:of|for)\s+([A-Za-z]+\d+)\s+(?:to|through)\s+([A-Za-z]+\d+)",
            |caps| {
                let cell1 = caps[1].to_uppercase();
                let cell2 = caps[2].to_uppercase();
                format!("=AVERAGE({}:{})", cell1, cell2)
            },
        );
    }
    
    /// Add a translation pattern (CASE-INSENSITIVE!)
    fn add_pattern(&mut self, pattern: &str, transformer: impl Fn(&regex::Captures) -> String + 'static) {
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(true)  // KEY FIX: Makes regex match "a1" and "A1"
            .build()
            .expect("Invalid regex pattern");
        
        self.patterns.push((regex, Box::new(transformer)));
    }
    
    /// Translate natural language to Excel formula
    pub fn translate(&self, text: &str) -> Option<String> {
        let text_lower = text.to_lowercase();
        
        for (regex, transformer) in &self.patterns {
            if let Some(caps) = regex.captures(&text_lower) {
                return Some(transformer(&caps));
            }
        }
        
        None
    }
    
    /// Check if text looks like a natural language formula request
    pub fn is_formula_request(&self, text: &str) -> bool {
        let keywords = vec![
            "add", "sum", "plus", "total",
            "subtract", "minus", "take away",
            "multiply", "times", 
            "divide", 
            "average", "mean",
            "cell", "column", "row",
        ];
        
        let text_lower = text.to_lowercase();
        keywords.iter().any(|&kw| text_lower.contains(kw))
    }
}

// Unit tests for the translator
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translator_basics() {
        let translator = NaturalLanguageTranslator::new();
        
        // Test basic translations
        assert_eq!(translator.translate("add A1 and B2"), Some("=A1+B2".to_string()));
        assert_eq!(translator.translate("ADD a1 AND b2"), Some("=A1+B2".to_string()));
        assert_eq!(translator.translate("sum A1 and B2"), Some("=A1+B2".to_string()));
        assert_eq!(translator.translate("plus A1 and B2"), Some("=A1+B2".to_string()));
        
        assert_eq!(translator.translate("subtract B2 from A1"), Some("=A1-B2".to_string()));
        assert_eq!(translator.translate("SUBTRACT b2 FROM a1"), Some("=A1-B2".to_string()));
        
        assert_eq!(translator.translate("multiply A1 by B2"), Some("=A1*B2".to_string()));
        assert_eq!(translator.translate("divide A1 by B2"), Some("=A1/B2".to_string()));
        
        assert_eq!(translator.translate("sum of A1 to A10"), Some("=SUM(A1:A10)".to_string()));
        assert_eq!(translator.translate("average of A1 to A10"), Some("=AVERAGE(A1:A10)".to_string()));
        
        // Test non-formula text
        assert_eq!(translator.translate("hello world"), None);
        assert_eq!(translator.translate("what is the weather"), None);
    }
    
    #[test]
    fn test_formula_request_detection() {
        let translator = NaturalLanguageTranslator::new();
        
        assert!(translator.is_formula_request("add cell A1 and B2"));
        assert!(translator.is_formula_request("calculate the sum"));
        assert!(translator.is_formula_request("multiply revenue by tax"));
        
        assert!(!translator.is_formula_request("hello world"));
        assert!(!translator.is_formula_request("what is the time"));
        assert!(!translator.is_formula_request(""));
    }
}