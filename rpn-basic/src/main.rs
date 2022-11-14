use std::fmt::format;


struct RpnCalculator {
    stack: Vec<u32>,
}

impl RpnCalculator {
    fn new() -> RpnCalculator {
        RpnCalculator{stack:Vec::new()}
    }

    fn calculate(&mut self, input:char) {

        if input.is_ascii_digit() {
            self.stack.push(input.to_digit(10).unwrap());
        } else {
            match input {
                '+' => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a+b);
                },
                '*' => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a*b);
                },
                _   => {}
            }
        } 

        let msg = format!("{input}=>{:?}",self.stack);
        println!("{msg}");
    }
}


fn main() {
    let mut my_calculator = RpnCalculator::new();

    let fake_keybd_inputs =
        vec!['2','3','+','4','1','+','*'];

    for input in fake_keybd_inputs {
        my_calculator.calculate(input);
    }
}
