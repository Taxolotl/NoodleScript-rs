use std::vector;

pub enum TokenType {
    Number,
    Bool,
    String,
    Const,
    Var,
    Func,
    EndStatement,  
	If,
	Else,
	While, 
	Equals,
	UnaryOperator, 
	BinaryOperator,
	OpenParen,
	CloseParen,
	OpenBracket, 
	CloseBracket, 
	Identifier,
	Comment, 
	EndOfLine, 
	EndOfFile,
	Invalid
}

pub struct TokenIdentifier {
    identifiers: Vec<String> ,
    token_identifiers: Box<TokenIdentifier>
}

pub struct Token {
	t_type: TokenType,
	contents: String,
}

pub fn process_escape_characters(mut input: String) {
	for (idx, chr) in input.chars().enumerate() {
		// Checks for potential escape characters if a slash is found
		if chr == '\\' && idx < input.len() - 1 {
			let nextChr = input.charAt(idx + 1).unwrap(); 
			
			let mut replacementChr = ' ';

			// Replace specific characters with escape characters
			match (nextChr) {
				'n' -> {
					replacementChr = '\n'; 
					break; 
				}
				't' -> {
					replacementChr = '\t'; 
					break; 
				}
			}

			if (replacementChr != ' ') {
				// Set the slash character to the escape character
				input.replace_range(i..i+1, replacementChr.encode_utf8(&mut [0; 1])); 
				// Erase the extra character
				//input.erase(input.begin() + idx + 1); fixme
			}
		}
	}
}

pub fn unprocess_escape_characters(_input: &str) {

} 

pub fn extract_next_identifier(line: &str, _seperator: Option<char>) -> String {
	let _sep = if seperator == None { ' ' } else { seperator.unwrap() };
	line.to_owned()
}

pub fn extract_words(input: &str) -> Vec<String> {
	vec![input.to_owned()]
}

pub fn determine_token_type(next_identifier: &str) -> Token {
	Token {
		t_type: TokenType::Identifier,
		contents: next_identifier
	}
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
	vec![
		Token {
			t_type: TokenType::Identifier,
			contents: source_code
		}
	]
}

// std::ostream& operator<<(std::ostream& ostream, Token token); 
// std::ostream& operator<<(std::ostream& ostream, const std::vector<Token>& tokens);