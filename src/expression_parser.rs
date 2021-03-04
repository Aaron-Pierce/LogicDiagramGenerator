use core::panic;
use std::{
    char, cmp,
    collections::VecDeque,
    convert::TryInto,
    fmt::{self, Display},
    u32, vec,
};

use crate::spritesheet::Sprite;

pub fn operator_precedence(c: &char) -> i8 {
    match c {
        '*' => 3,
        '+' => 2,
        '\'' => 4,
        '(' => -1,
        ')' => -1,
        _ => 0,
    }
}

fn infix_to_postfix(string: &str) -> String {
    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();
    String::from(string).chars().for_each(|c| {
        println!("checking char {:}", c);

        if operator_precedence(&c) == 0 {
            println!("\t{:} is variable", c);
            postfix.push(c);
        } else {
            println!("\t{:} is operator", c);

            if c == ')' {
                let mut popped = stack.pop().unwrap();
                println!("first ) pop is {:}", popped);
                while popped != '(' && stack.len() > 0 {
                    postfix.push(popped);
                    popped = stack.pop().unwrap();
                }
            } else {
                if stack.len() == 0
                    || operator_precedence(&c) >= operator_precedence(stack.last().unwrap())
                    || c == '('
                {
                    println!("\t and was higher precidence");
                    stack.push(c);
                } else {
                    println!("\t and was lower precidence");
                    let mut popped = stack.pop().unwrap();
                    while operator_precedence(&c) <= operator_precedence(&popped) {
                        println!("\t and was lower precidence, so we're pushing {:?}", popped);
                        postfix.push(popped);
                        let next = stack.last();
                        match next {
                            Some(_) => popped = stack.pop().unwrap(),
                            None => break,
                        }
                    }
                    if operator_precedence(&c) > operator_precedence(&popped) {
                        stack.push(popped);
                    }
                    stack.push(c)
                }
            }
            println!("Stack state: {:?}", stack);
        }
    });

    println!("Leftovers {:?}", stack);
    for _ in 0..stack.len() {
        postfix.push(stack.pop().unwrap())
    }

    println!("Finished {:?}", postfix);

    postfix
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    AND,
    OR,
    NOT,
    INPUT,
}

impl Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Gate {
    gate_type: GateType,
    inputs: Vec<Gate>,
    name: String,
}

impl Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:} {{ \n\t{:#?}}}", self.gate_type, self.inputs)
    }
}

impl Gate {
    pub fn depth(&self) -> u32 {
        if self.inputs.len() == 0 {
            return 1;
        } else {
            let mut max_depth = 0;
            self.inputs.iter().for_each(|e| {
                max_depth = cmp::max(max_depth, e.depth());
            });
            return max_depth + 1;
        }
    }

    pub fn get_type(&self) -> GateType {
        return self.gate_type.clone();
    }
    pub fn get_inputs(&self) -> &Vec<Gate> {
        return &self.inputs;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn calculate_drawn_image_width(&self) -> u32 {
        100 * self.depth() + 30
    }

    pub fn sprite_type(&self) -> Sprite {
        match self.get_type() {
            GateType::AND => Sprite::AND,
            GateType::OR => Sprite::OR,
            GateType::NOT => Sprite::NOT,
            GateType::INPUT => Sprite::INPUT,
        }
    }

    pub fn child_yoffset_function(
        &self,
        last_origin: u32,
        index_relative_to_siblings: u32,
        num_parent_inputs: u32,
    ) -> u32 {
        let signed_index = (index_relative_to_siblings * 2) as i32 - 1;

        println!("Signed_index is {}", signed_index);

        // if self.get_type() == GateType::INPUT {
        //     return (last_origin as i32 + signed_index*(12)) as u32;
        // }

        if num_parent_inputs == 1 {
            return last_origin;
        }

        let last_origin_i32 = last_origin as i32;
        let signed_modifier = signed_index * (self.size_of_largest_column() as i32 * 40);

        if signed_modifier.abs() > last_origin_i32.abs() {
            println!(
                "***** {} overflow: {} > {}",
                self.get_type(),
                signed_modifier,
                last_origin_i32
            );
        }

        (last_origin_i32 + signed_modifier) as u32
    }

    fn find_min_y_element(
        &self,
        prev_root: u32,
        index_relative_to_siblings: u32,
        num_parent_inputs: u32,
    ) -> u32 {
        let this_y =
            self.child_yoffset_function(prev_root, index_relative_to_siblings, num_parent_inputs);

        if self.inputs.len() == 0 {
            return this_y;
        } else {
            let mut child_inputs: Vec<u32> = Vec::new();
            for i in 0..self.inputs.len() {
                child_inputs.push(self.inputs.get(i).unwrap().find_min_y_element(
                    this_y,
                    i as u32,
                    self.inputs.len() as u32,
                ));
            }

            return this_y.min(*child_inputs.iter().min().unwrap_or(&0));
        }
    }

    fn find_max_y_element(
        &self,
        prev_root: u32,
        index_relative_to_siblings: u32,
        num_parent_inputs: u32,
    ) -> u32 {
        let this_y =
            self.child_yoffset_function(prev_root, index_relative_to_siblings, num_parent_inputs);

        if self.inputs.len() == 0 {
            return this_y;
        } else {
            let mut child_inputs: Vec<u32> = Vec::new();
            for i in 0..self.inputs.len() {
                let found_value = self.inputs.get(i).unwrap().find_max_y_element(
                    this_y,
                    i as u32,
                    self.inputs.len() as u32,
                );
                child_inputs.push(found_value);
            }

            return this_y.max(*child_inputs.iter().max().unwrap_or(&0)) + 40;
        }
    }

    pub fn calculate_drawn_image_height(&self) -> u32 {
        println!("Calculating drawn image height....");
        let max = self.find_max_y_element((2 as u32).pow(self.depth()) * 50, 0, 0);
        let min = self.find_min_y_element((2 as u32).pow(self.depth()) * 50, 0, 0);
        println!("Found max: {}", max);
        println!("Found min: {}", min);
        2 * (max - min) + 20
    }

    pub fn adjusted_origin(&self, init_origin: u32) -> u32 {
        let max = self.find_max_y_element((2 as u32).pow(self.depth()) * 50, 0, 0);
        let min = self.find_min_y_element((2 as u32).pow(self.depth()) * 50, 0, 0);
        // return (init_origin as i32 + (((2 as u32).pow(self.depth()) * 50) as i32) - ((max + min)/2) as i32) as u32;

        let diff = ((max + min) / 2) as i32 - ((2 as u32).pow(self.depth()) * 50) as i32;
        println!("We think the origin is off by {}", diff);
        println!("Informed by max: {}, min: {}, average: {}, origin: {}", max, min, (max + min) / 2, (2 as u32).pow(self.depth()) * 50);
        // return (init_origin as i32 - diff) as u32;
        // (init_origin as i32 + diff) as u32
        (init_origin as i32) as u32
    }

    pub fn column_sizes(&self) -> Vec<u32> {
        let mut queue: VecDeque<&Gate> = VecDeque::new();
        queue.push_front(self);
        let mut columns: Vec<u32> = vec![0];

        let mut this_layer = 1;
        let mut next_layer = 0;
        while !queue.is_empty() {
            *columns.last_mut().unwrap() += 1;
            this_layer -= 1;

            let this = queue.pop_back().unwrap();
            for c in &this.inputs {
                queue.push_front(c);
                next_layer += 1;
            }

            if this_layer == 0 {
                this_layer = next_layer;
                next_layer = 0;
                if !queue.is_empty() {
                    columns.push(0);
                }
            }
        }

        columns.reverse();
        columns
    }

    pub fn size_of_largest_column(&self) -> u32 {
        *self.column_sizes().iter().max().unwrap_or(&0)
    }
}


fn gate_type_to_operator_symbol(gate_type: GateType) -> String {
    match gate_type{
        GateType::AND => String::from(""),
        GateType::OR =>  String::from("+"),
        GateType::NOT =>  String::from("'"),
        GateType::INPUT =>  String::from("")
    }
}

fn create_tree(postfix_string: &str) -> Gate {
    let mut stack: Vec<Gate> = Vec::new();
    postfix_string.chars().for_each(|c| {
        if operator_precedence(&c) == 0 {
            stack.push(Gate {
                gate_type: GateType::INPUT,
                inputs: Vec::new(),
                name: c.to_string(),
            })
        } else {
            let num_to_pop = match c {
                '+' => 2,
                '*' => 2,
                '\'' => 1,
                _ => panic!("found operator outside of supported possibilities in create_tree"),
            };

            let mut popped: Vec<Gate> = Vec::new();
            for _ in 0..num_to_pop {
                popped.push(stack.pop().unwrap());
            }
            let gate_type = match c {
                '+' => GateType::OR,
                '*' => GateType::AND,
                '\'' => GateType::NOT,
                _ => panic!("found operator outside of supported possibilities in create_tree"),
            };

            let mut names_of_inputs: Vec<String> = Vec::new();
            for gate in &popped {
                names_of_inputs.push(gate.name.clone());
            }
            names_of_inputs.reverse();

            let mut created_name = names_of_inputs.join(&gate_type_to_operator_symbol(gate_type.clone())[..]);
            if(names_of_inputs.len() == 1){
                created_name.push_str(&gate_type_to_operator_symbol(gate_type.clone()))
            }

            stack.push(Gate {
                gate_type: gate_type,
                inputs: popped,
                name: created_name
            })
        }
    });
    stack.remove(0)
}

pub fn parse_boolean_expression(string: &str) -> Gate {
    let mut condensed = String::from(string);
    condensed.retain(|c| !c.is_whitespace());

    let mut explicitly_multiplied = String::new();

    let chars: Vec<char> = condensed.chars().collect();
    for i in 0..(condensed.len() - 1) {
        let c1 = chars.get(i).unwrap();
        let c2 = chars.get(i + 1).unwrap();
        let p1 = operator_precedence(c1);
        let p2 = operator_precedence(c2);

        explicitly_multiplied.push(*chars.get(i).unwrap());

        if (p1 == 0 && p2 == 0)
            || (p1 == 0 && *c2 == '(')
            || (*c1 == ')' && p2 == 0)
            || (*c1 == '\'' && p2 == 0)
        {
            explicitly_multiplied.push('*');
        }
    }
    explicitly_multiplied.push(*chars.last().unwrap());

    println!("{:?}", explicitly_multiplied);

    let postfix = infix_to_postfix(&explicitly_multiplied);
    let tree = create_tree(&postfix);

    println!("{:?}", tree);

    tree
}
