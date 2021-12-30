
const INPUT_INSTRUCTIONS: [&str; 2] = ["XIC", "XIO"];
const OUTPUT_INSTRUCTIONS: [&str; 6] = ["OTE", "OTL", "OTU", "JSR", "RET", "EMIT"];

#[derive(Default)]
pub struct CodeGenerator {
    current_code_block: String,
    indentation_level: usize,
    current_rung_name: String,
    rung_number: u32,
    output_instruction_flag: bool,
    if_block_instructions: Vec<String>,
    else_block_instructions: Vec<String>
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator::default()
    }

    fn add_to_code_block(&mut self, code: &str) {
        for _ in 0..self.indentation_level {
            self.current_code_block += "\t";
        }
        self.current_code_block += code;
        self.current_code_block += "\n";
    }

    fn finish_code_block(&mut self) -> String {
        // Add entry point of task
        self.add_to_code_block("Main()");

        // Trim off the last new line character
        let code_block = self.current_code_block[0..self.current_code_block.len() - 1].to_owned();
        self.current_code_block = String::new();
        self.indentation_level = 0;
        code_block
    }

    fn start_routine(&mut self, routine_name: &str) {
        self.add_to_code_block(format!("def {}():", routine_name).as_str());
        self.indentation_level += 1;
    }

    fn end_routine(&mut self) {
        // If we don't have any rungs, we need to add a pass
        if self.rung_number == 0 {
            self.add_to_code_block("pass");
        }
        self.indentation_level -= 1;
        self.rung_number = 0;
    }

    fn start_rung(&mut self, rung_name: &str) {
        let mut editted_rung_name = String::default();
        if rung_name.is_empty() {
            editted_rung_name = format!("rung_{}_entry", self.rung_number);
        } else {
            editted_rung_name = format!("rung_{}_entry", rung_name);
        }
        self.rung_number += 1;

        self.add_to_code_block(format!("{} = True", editted_rung_name).as_str());
        self.current_rung_name = editted_rung_name;
    }

    fn end_rung(&mut self) {
        // Actually add the output instructions now if there were any
        if !self.if_block_instructions.is_empty() {
            self.add_to_code_block(format!("if {}:", self.current_rung_name).as_str());
            self.indentation_level += 1;

            while let Some(instruction) = self.if_block_instructions.pop() {
                self.add_to_code_block(&instruction);
            }

            self.indentation_level -= 1;
        }

        if !self.else_block_instructions.is_empty() {
            self.add_to_code_block("else:");
            self.indentation_level += 1;

            while let Some(instruction) = self.else_block_instructions.pop() {
                self.add_to_code_block(&instruction);
            }

            self.indentation_level -= 1;
        }

        self.output_instruction_flag = false;
    }

    fn add_input_instruction(&mut self, instruction: &str, target: &str) {
        if self.output_instruction_flag {
            panic!("Input instruction {} appears after an output instruction", instruction);
        }

        if instruction == "XIC" {
            self.add_to_code_block(format!("{} &= {}", self.current_rung_name, target).as_str());
        } else if instruction == "XIO" {
            self.add_to_code_block(format!("{} &= not {}", self.current_rung_name, target).as_str());
        } else {
            unreachable!("Missing input instruction");
        }
    }

    fn add_output_instruction(&mut self, instruction: &str, target: &str) {

        match instruction {
            "RET" => {
                self.if_block_instructions.insert(0, "return".to_string());
            },
            "JSR" => {
                self.if_block_instructions.insert(0, format!("{}()", target));
            },
            "OTL" => {
                self.if_block_instructions.insert(0, format!("{} = True", target));
            },
            "OTU" => {
                self.if_block_instructions.insert(0, format!("{} = False", target));
            },
            "OTE" => {
                self.if_block_instructions.insert(0, format!("{} = True", target));
                self.else_block_instructions.insert(0, format!("{} = False", target));
            },
            "EMIT" => {
                self.if_block_instructions.insert(0, format!("EmitEvent('{}')", target));
            },
            _ => {
                unreachable!("Missing output instruction");
            }
        }
        self.output_instruction_flag = true;
    }

    fn add_instruction(&mut self, instruction: &str, target: &str) {
        if INPUT_INSTRUCTIONS.contains(&instruction) {
            self.add_input_instruction(instruction, target);
        } else if OUTPUT_INSTRUCTIONS.contains(&instruction) {
            self.add_output_instruction(instruction, target);
        } else {
            panic!("Invalid instruction {}", instruction);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_code_generation() {
        let mut code_generator = CodeGenerator::new();

        code_generator.start_routine("Main");
        code_generator.start_rung("firstRung");
        code_generator.add_instruction("XIO", "MyTag1");
        code_generator.add_instruction("XIC", "MyTag2");
        code_generator.add_instruction("OTL", "MyTag3");
        code_generator.add_instruction("OTU", "MyTag4");
        code_generator.add_instruction("OTE", "MyTag5");
        code_generator.add_instruction("JSR", "otherRoutine");
        code_generator.end_rung();
        code_generator.end_routine();

        code_generator.start_routine("otherRoutine");
        code_generator.start_rung("");
        code_generator.add_instruction("RET", "");
        code_generator.end_rung();
        code_generator.end_routine();

        let expected_output = "def Main():
\trung_firstRung_entry = True
\trung_firstRung_entry &= not MyTag1
\trung_firstRung_entry &= MyTag2
\tif rung_firstRung_entry:
\t\tMyTag3 = True
\t\tMyTag4 = False
\t\tMyTag5 = True
\t\totherRoutine()
\telse:
\t\tMyTag5 = False
def otherRoutine():
\trung_0_entry = True
\tif rung_0_entry:
\t\treturn
Main()";
        let actual_output = code_generator.finish_code_block();
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    #[should_panic]
    fn test_input_after_output() {
        let mut code_generator = CodeGenerator::new();

        code_generator.start_routine("Main");
        code_generator.start_rung("firstRung");
        code_generator.add_input_instruction("XIC", "MyTag");
        code_generator.add_output_instruction("OTE", "MyTag");
        code_generator.add_input_instruction("XIC", "MyTag");
    }

    #[test]
    fn test_empty_routine() {
        let mut code_generator = CodeGenerator::new();

        code_generator.start_routine("Main");
        code_generator.end_routine();

        assert_eq!(code_generator.finish_code_block(), "def Main():\n\tpass\nMain()");
    }
}