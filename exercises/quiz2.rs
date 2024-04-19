// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
/*
    让我们建立一个函数形式的小机器。作为输入，我们将给出一个字符串和命令列表。
    这些命令决定将对字符串应用什么操作。它可以是:
    -将字符串大写
    -修剪字符串
    -将"bar"添加到字符串的指定次数，其确切形式为:
    -输入将是一个2长度元组的Vector，第一个元素是字符串，第二个是命令。
    -输出元素将是一个Vector类型的字符串。
 */
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!

        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!

            let transformed = match command {
                Command::Uppercase => string.to_uppercase(), // 转换为大写
                Command::Trim => string.trim().to_string(), // 去除首尾空白
                Command::Append(times) => {
                    let append_str = "bar".repeat(*times); // 重复"bar"指定次数
                    string.to_string() + &append_str // 拼接字符串
                },
            };
            output.push(transformed);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
