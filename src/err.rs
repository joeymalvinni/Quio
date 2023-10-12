const ERROR_CONTEXT_LENGTH: usize = 6; // context length on either side of error char

pub fn generate_error_message(input: String, index: usize, message: &str) -> String {
    let start_index = index.saturating_sub(ERROR_CONTEXT_LENGTH);
    let end_index = (index + ERROR_CONTEXT_LENGTH + 1).min(input.len());

    let context = &input[start_index..end_index];
    let pointer_position = index - start_index;

    format!("Error at position {}:\n{}\n{}^ {}", index, context, " ".repeat(pointer_position), message)
}