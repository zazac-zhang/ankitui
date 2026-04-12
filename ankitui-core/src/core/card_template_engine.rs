//! Card Type Template Engine
//!
//! Handles rendering of different card types with template processing

use crate::data::models::*;
use std::collections::HashMap;

/// Template engine for card rendering
pub struct CardTemplateEngine {
    templates: HashMap<CardType, CardTemplate>,
}

impl CardTemplateEngine {
    /// Create a new template engine with default templates
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Basic card template
        templates.insert(
            CardType::Basic,
            CardTemplate {
                name: "Basic".to_string(),
                card_type: CardType::Basic,
                front_template: "{{front}}".to_string(),
                back_template: "{{front}}\n\n---\n\n{{back}}".to_string(),
                fields: vec!["front".to_string(), "back".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        // Basic reversed card template
        templates.insert(
            CardType::BasicReversed,
            CardTemplate {
                name: "Basic (and reversed card)".to_string(),
                card_type: CardType::BasicReversed,
                front_template: "{{front}}".to_string(),
                back_template: "{{back}}".to_string(),
                fields: vec!["front".to_string(), "back".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        // Cloze card template
        templates.insert(
            CardType::Cloze,
            CardTemplate {
                name: "Cloze".to_string(),
                card_type: CardType::Cloze,
                front_template: "{{cloze:Text}}".to_string(),
                back_template: "{{cloze:Text}}".to_string(),
                fields: vec!["Text".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        // Input card template
        templates.insert(
            CardType::Input,
            CardTemplate {
                name: "Input".to_string(),
                card_type: CardType::Input,
                front_template: "{{question}}".to_string(),
                back_template: "{{question}}\n\nAnswer: {{answer}}".to_string(),
                fields: vec!["question".to_string(), "answer".to_string()],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        // Multiple choice card template
        templates.insert(
            CardType::MultipleChoice,
            CardTemplate {
                name: "Multiple Choice".to_string(),
                card_type: CardType::MultipleChoice,
                front_template: "{{question}}".to_string(),
                back_template: "{{question}}\n\n{{#options}}\n{{.}}\n{{/options}}\n\nCorrect: {{correct_answer}}"
                    .to_string(),
                fields: vec![
                    "question".to_string(),
                    "options".to_string(),
                    "correct_answer".to_string(),
                ],
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            },
        );

        Self { templates }
    }

    /// Render a card using the appropriate template
    pub fn render_card(
        &self,
        card_content: &ExtendedCardContent,
        side: CardSide,
    ) -> Result<RenderedCard, TemplateError> {
        let template = self
            .templates
            .get(&card_content.card_type)
            .ok_or(TemplateError::TemplateNotFound(card_content.card_type))?;

        let context = self.build_rendering_context(card_content, side)?;
        let rendered_content = self.process_template(template, &context, side)?;

        Ok(RenderedCard {
            content: rendered_content,
            media_refs: card_content.media.clone(),
            has_input: card_content.card_type == CardType::Input,
            expected_answer: self.get_expected_answer(card_content),
            multiple_choice_options: self.get_multiple_choice_options(card_content),
        })
    }

    /// Build rendering context from card content
    fn build_rendering_context(
        &self,
        card_content: &ExtendedCardContent,
        side: CardSide,
    ) -> Result<CardRenderingContext, TemplateError> {
        let mut fields = HashMap::new();
        let mut extra = HashMap::new();

        match card_content.card_type {
            CardType::Basic | CardType::BasicReversed => {
                if let Some(front) = card_content.custom.get("front").and_then(|v| v.as_str()) {
                    fields.insert("front".to_string(), front.to_string());
                }
                if let Some(back) = card_content.custom.get("back").and_then(|v| v.as_str()) {
                    fields.insert("back".to_string(), back.to_string());
                }
            }
            CardType::Cloze => {
                if let Some(cloze_str) = card_content.custom.get("cloze_data").and_then(|v| v.as_str()) {
                    if let Ok(cloze_data) = serde_json::from_str::<ClozeData>(cloze_str) {
                        let front_text = format_cloze_front(&cloze_data, cloze_data.cloze_number);
                        let back_text = format_cloze_back(&cloze_data, cloze_data.cloze_number);

                        fields.insert("Text".to_string(), cloze_data.text);
                        fields.insert(
                            "cloze:Text".to_string(),
                            if side == CardSide::Front { front_text } else { back_text },
                        );

                        extra.insert("cloze_number".to_string(), cloze_data.cloze_number.to_string());
                    }
                }
            }
            CardType::Input => {
                if let Some(input_str) = card_content.custom.get("input_data").and_then(|v| v.as_str()) {
                    if let Ok(input_data) = serde_json::from_str::<InputData>(input_str) {
                        fields.insert("question".to_string(), input_data.question);
                        fields.insert("answer".to_string(), input_data.answer);

                        if let Some(hint) = input_data.hint {
                            extra.insert("hint".to_string(), hint);
                        }
                        extra.insert("case_sensitive".to_string(), input_data.case_sensitive.to_string());
                        extra.insert("strict".to_string(), input_data.strict.to_string());
                    }
                }
            }
            CardType::MultipleChoice => {
                if let Some(mc_str) = card_content.custom.get("mc_data").and_then(|v| v.as_str()) {
                    if let Ok(mc_data) = serde_json::from_str::<MultipleChoiceData>(mc_str) {
                        fields.insert("question".to_string(), mc_data.question);

                        // Format options as a single string with newlines
                        let options_str = mc_data.options.join("\n");
                        fields.insert("options".to_string(), options_str);

                        fields.insert("correct_answer".to_string(), (mc_data.correct_answer + 1).to_string());

                        if let Some(explanation) = mc_data.explanation {
                            extra.insert("explanation".to_string(), explanation);
                        }
                    }
                }
            }
            CardType::ImageOcclusion => {
                // Image occlusion would need special handling
                // For now, just use basic fields
                if let Some(io_str) = card_content.custom.get("io_data").and_then(|v| v.as_str()) {
                    if let Ok(io_data) = serde_json::from_str::<ImageOcclusionData>(io_str) {
                        if let Some(question) = io_data.question {
                            fields.insert("question".to_string(), question);
                        }
                        if let Some(answer) = io_data.answer {
                            fields.insert("answer".to_string(), answer);
                        }
                        fields.insert("image_path".to_string(), io_data.image_path);
                    }
                }
            }
        }

        // Add tags to fields
        fields.insert("tags".to_string(), card_content.tags.join(", "));

        Ok(CardRenderingContext {
            card_type: card_content.card_type,
            side,
            fields,
            media_refs: card_content.media.clone(),
            extra,
        })
    }

    /// Process template with context
    fn process_template(
        &self,
        template: &CardTemplate,
        context: &CardRenderingContext,
        side: CardSide,
    ) -> Result<String, TemplateError> {
        let template_str = if side == CardSide::Front {
            &template.front_template
        } else {
            &template.back_template
        };

        let mut result = template_str.to_string();

        // Simple field substitution
        for (key, value) in &context.fields {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        // Handle conditional blocks (basic implementation)
        result = self.process_conditionals(&result, &context.fields);

        Ok(result)
    }

    /// Process conditional template blocks
    fn process_conditionals(&self, template: &str, fields: &HashMap<String, String>) -> String {
        let result = template.to_string();

        // Simple conditional processing: {{#field}}...{{/field}}
        // This is a basic implementation - a full implementation would be more complex
        let mut processed = result;

        for field_name in fields.keys() {
            let start_pattern = format!("{{#{}}}", field_name);
            let end_pattern = format!("{{/{}}}", field_name);

            if let Some(start_pos) = processed.find(&start_pattern) {
                if let Some(end_pos) = processed[start_pos..].find(&end_pattern) {
                    let end_pos = start_pos + end_pos + end_pattern.len();
                    let block_content =
                        &processed[start_pos + start_pattern.len()..start_pos + end_pos - end_pattern.len()];

                    // Replace the entire block with the content if field exists and is not empty
                    let replacement = if !fields[field_name].is_empty() {
                        block_content
                    } else {
                        ""
                    };

                    processed = processed[..start_pos].to_string() + replacement + &processed[end_pos..];
                }
            }
        }

        processed
    }

    /// Get expected answer for input validation
    fn get_expected_answer(&self, card_content: &ExtendedCardContent) -> Option<String> {
        match card_content.card_type {
            CardType::Input => {
                if let Some(input_str) = card_content.custom.get("input_data").and_then(|v| v.as_str()) {
                    if let Ok(input_data) = serde_json::from_str::<InputData>(input_str) {
                        return Some(input_data.answer);
                    }
                }
            }
            CardType::Cloze => {
                if let Some(cloze_str) = card_content.custom.get("cloze_data").and_then(|v| v.as_str()) {
                    if let Ok(cloze_data) = serde_json::from_str::<ClozeData>(cloze_str) {
                        if let Some(cloze_item) = cloze_data.clozes.get(cloze_data.cloze_number - 1) {
                            return Some(cloze_item.answer.clone());
                        }
                    }
                }
            }
            _ => {}
        }
        None
    }

    /// Get multiple choice options for TUI display
    fn get_multiple_choice_options(&self, card_content: &ExtendedCardContent) -> Option<Vec<String>> {
        if let Some(mc_str) = card_content.custom.get("mc_data").and_then(|v| v.as_str()) {
            if let Ok(mc_data) = serde_json::from_str::<MultipleChoiceData>(mc_str) {
                return Some(mc_data.options);
            }
        }
        None
    }

    /// Validate answer for input cards
    pub fn validate_answer(&self, card_content: &ExtendedCardContent, user_answer: &str) -> AnswerValidation {
        match card_content.card_type {
            CardType::Input => {
                if let Some(input_str) = card_content.custom.get("input_data").and_then(|v| v.as_str()) {
                    if let Ok(input_data) = serde_json::from_str::<InputData>(input_str) {
                        let expected = &input_data.answer;

                        if input_data.case_sensitive {
                            if input_data.strict {
                                if user_answer == expected {
                                    return AnswerValidation::Correct;
                                } else {
                                    return AnswerValidation::Incorrect(expected.clone());
                                }
                            } else {
                                // Allow minor variations (case sensitive but not strict)
                                if user_answer.trim() == expected.trim() {
                                    return AnswerValidation::Correct;
                                } else {
                                    return AnswerValidation::Incorrect(expected.clone());
                                }
                            }
                        } else {
                            // Case insensitive comparison
                            let user_normalized = user_answer.trim().to_lowercase();
                            let expected_normalized = expected.trim().to_lowercase();

                            if user_normalized == expected_normalized {
                                return AnswerValidation::Correct;
                            } else {
                                return AnswerValidation::Incorrect(expected.clone());
                            }
                        }
                    }
                }
                AnswerValidation::Error("Invalid input card data".to_string())
            }
            CardType::Cloze => {
                if let Some(cloze_str) = card_content.custom.get("cloze_data").and_then(|v| v.as_str()) {
                    if let Ok(cloze_data) = serde_json::from_str::<ClozeData>(cloze_str) {
                        if let Some(cloze_item) = cloze_data.clozes.get(cloze_data.cloze_number - 1) {
                            if user_answer.trim().to_lowercase() == cloze_item.answer.trim().to_lowercase() {
                                return AnswerValidation::Correct;
                            } else {
                                return AnswerValidation::Incorrect(cloze_item.answer.clone());
                            }
                        }
                    }
                }
                AnswerValidation::Error("Invalid cloze card data".to_string())
            }
            CardType::MultipleChoice => {
                if let Some(mc_str) = card_content.custom.get("mc_data").and_then(|v| v.as_str()) {
                    if let Ok(mc_data) = serde_json::from_str::<MultipleChoiceData>(mc_str) {
                        if let Ok(selected_index) = user_answer.parse::<usize>() {
                            if selected_index - 1 == mc_data.correct_answer {
                                return AnswerValidation::Correct;
                            } else {
                                return AnswerValidation::Incorrect(format!("Option {}", mc_data.correct_answer + 1));
                            }
                        }
                    }
                }
                AnswerValidation::Error("Invalid multiple choice card data".to_string())
            }
            _ => AnswerValidation::NotApplicable,
        }
    }

    /// Add or update a custom template
    pub fn add_template(&mut self, template: CardTemplate) {
        self.templates.insert(template.card_type, template);
    }

    /// Get all available templates
    pub fn get_templates(&self) -> &HashMap<CardType, CardTemplate> {
        &self.templates
    }
}

impl Default for CardTemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Template rendering errors
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template not found for card type: {0:?}")]
    TemplateNotFound(CardType),
    #[error("Invalid card data: {0}")]
    InvalidCardData(String),
    #[error("Template processing error: {0}")]
    ProcessingError(String),
}

/// Answer validation result
#[derive(Debug, Clone, PartialEq)]
pub enum AnswerValidation {
    Correct,
    Incorrect(String), // Contains the correct answer
    Error(String),
    NotApplicable,
}

/// Format cloze text for front side
fn format_cloze_front(cloze_data: &ClozeData, cloze_number: usize) -> String {
    if let Some(cloze_item) = cloze_data.clozes.get(cloze_number - 1) {
        cloze_data.text.replace(&cloze_item.answer, "...")
    } else {
        cloze_data.text.clone()
    }
}

/// Format cloze text for back side
fn format_cloze_back(cloze_data: &ClozeData, cloze_number: usize) -> String {
    if let Some(cloze_item) = cloze_data.clozes.get(cloze_number - 1) {
        cloze_data
            .text
            .replace(&cloze_item.answer, &format!("[{}]", cloze_item.answer))
    } else {
        cloze_data.text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_template_engine_creation() {
        let engine = CardTemplateEngine::new();
        assert!(!engine.get_templates().is_empty());
        assert!(engine.get_templates().contains_key(&CardType::Basic));
    }

    #[test]
    fn test_basic_card_rendering() {
        let engine = CardTemplateEngine::new();
        let card = ExtendedCardContent::basic("What is 2+2?".to_string(), "4".to_string());

        let front_result = engine.render_card(&card, CardSide::Front).unwrap();
        let back_result = engine.render_card(&card, CardSide::Back).unwrap();

        assert!(front_result.content.contains("2+2"));
        assert!(back_result.content.contains("4"));
    }

    #[test]
    fn test_input_card_creation() {
        let card = ExtendedCardContent::input(
            "What is the capital of France?".to_string(),
            "Paris".to_string(),
            Some("Hint: It's in Europe".to_string()),
        );

        assert_eq!(card.card_type, CardType::Input);
        assert_eq!(card.front(), "What is the capital of France?");
        assert!(card.back().contains("Paris"));
        assert!(card.back().contains("Hint: It's in Europe"));
    }

    #[test]
    fn test_multiple_choice_card_creation() {
        let options = vec![
            "London".to_string(),
            "Paris".to_string(),
            "Berlin".to_string(),
            "Madrid".to_string(),
        ];

        let card = ExtendedCardContent::multiple_choice(
            "What is the capital of France?".to_string(),
            options,
            1, // Paris is correct (index 1)
            Some("Paris is the capital and most populous city of France.".to_string()),
        );

        assert_eq!(card.card_type, CardType::MultipleChoice);
        assert!(card.front().contains("capital of France"));
        assert!(card.back().contains("Correct Answer: 2"));
        assert!(card.back().contains("Explanation:"));
    }

    #[test]
    fn test_answer_validation() {
        let engine = CardTemplateEngine::new();

        // Test input card
        let input_card = ExtendedCardContent::input("Question".to_string(), "Answer".to_string(), None);

        assert!(matches!(
            engine.validate_answer(&input_card, "Answer"),
            AnswerValidation::Correct
        ));
        assert!(matches!(
            engine.validate_answer(&input_card, "Wrong"),
            AnswerValidation::Incorrect(_)
        ));

        // Test multiple choice card
        let mc_card = ExtendedCardContent::multiple_choice(
            "Question".to_string(),
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            1, // B is correct
            None,
        );

        assert!(matches!(
            engine.validate_answer(&mc_card, "2"),
            AnswerValidation::Correct
        ));
        assert!(matches!(
            engine.validate_answer(&mc_card, "1"),
            AnswerValidation::Incorrect(_)
        ));
    }
}
