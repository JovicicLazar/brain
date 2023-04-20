use crate::token::Token;

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens        = vec![];
    let mut comment       = String::new();
    let mut bracket_count = 0; 

    for char in src.chars() {
        let token;

        match char {
            '>' => token = Some(Token::MoveRight),
            '<' => token = Some(Token::MoveLeft),
            '+' => token = Some(Token::Increment),
            '-' => token = Some(Token::Decrement),
            '.' => token = Some(Token::Print),
            ',' => token = Some(Token::Input),
            '[' => {
                bracket_count += 1;
                token = Some(Token::OpenBracket);
            }
            ']' => {
                bracket_count -= 1;
                token = Some(Token::CloseBracket);
            }
            c => {
                token = None;
                comment.push(c);
            }
        };

        if let Some(token) = token {
            if comment.len() > 0 {
                tokens.push(Token::Comment(comment.clone()));
                comment.clear();
            }
            tokens.push(token);
        }

        if bracket_count < 0 {
            panic!("Unmatched brackets!");
        }
    }

    if bracket_count != 0 {
        panic!("Unbalanced brackets!");
    }

    if comment.len() > 0 {
        tokens.push(Token::Comment(comment.clone()));
        comment.clear();
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tokens(){
        let src = "<>+-.,[]";
        let tokens = tokenize(src);

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0], Token::MoveLeft);
        assert_eq!(tokens[1], Token::MoveRight);
        assert_eq!(tokens[2], Token::Increment);
        assert_eq!(tokens[3], Token::Decrement);
        assert_eq!(tokens[4], Token::Print);
        assert_eq!(tokens[5], Token::Input);
        assert_eq!(tokens[6], Token::OpenBracket);
        assert_eq!(tokens[7], Token::CloseBracket);
    }  

    #[test]
    fn simple_tokenize() {
        let src = "<<>>";
        let tokens = tokenize(src);

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::MoveLeft);
        assert_eq!(tokens[1], Token::MoveLeft);
        assert_eq!(tokens[2], Token::MoveRight);
        assert_eq!(tokens[3], Token::MoveRight);
    }

    #[test]
    fn comment_tokenize() {
        let src = "<<>>ThisIsAComment+++Comment2";
        let tokens = tokenize(src);

        println!("{:#?}", tokens);

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0], Token::MoveLeft);
        assert_eq!(tokens[1], Token::MoveLeft);
        assert_eq!(tokens[2], Token::MoveRight);
        assert_eq!(tokens[3], Token::MoveRight);
        assert_eq!(tokens[4], Token::Comment("ThisIsAComment".to_string()));
        assert_eq!(tokens[5], Token::Increment);
        assert_eq!(tokens[6], Token::Increment);
        assert_eq!(tokens[7], Token::Increment);
        assert_eq!(tokens[8], Token::Comment("Comment2".to_string()));
    }

    #[test]
    #[should_panic]
    fn unbalanced_tokenize() {
        let src = "<<>>[[]][][[]";
        let _tokens = tokenize(src);
    }

    #[test]
    #[should_panic]
    fn unmatched_tokenize() {
        let src = "<<>>][";
        let _tokens = tokenize(src);
    }
}
