
#[derive(Debug)]
enum Operation<'a> {
    Comment { text: &'a str },
    Call { caller: &'a str, arg: &'a str, body: &'a str }
}

#[derive(Debug)]
struct TreeNode<'a> {
    operation: Operation<'a>,
    chidlren: Vec<TreeNode<'a>>
}

pub fn parse(text: &String) {
    let mut char_stack: Vec<char> = Vec::new();

    for ch in text.chars() {
        char_stack.push(ch);

        
    }

    println!("{}", char_stack[0]);
}