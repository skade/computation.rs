use smallstep::Node;
use smallstep::environment::Environment;

pub struct Machine {
    pub expression: Node
}

impl Machine {
    pub fn new(expression: Node) -> Machine {
        Machine{ expression: expression }
    }

    pub fn run(&self, environment: &mut Environment) {
        let mut node = self.expression.clone();
        while node.reducable() {
            println!("{}", node);
            node = node.reduce(environment);
        }
        println!("{}", node);
    }
}
