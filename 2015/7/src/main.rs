use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
  advent_of_code::run(first, second);
}

fn first(input: String) -> u16 {
  run_circuit(construct_circuit(&input))
}

fn second(input: String) -> u16 {
  let evaluable_gates = construct_circuit(&input);
  let result = run_circuit(evaluable_gates);

  let evaluable_gates = construct_circuit(&input);
  let gate_b = evaluable_gates.iter().find(|x| x.borrow().identifier == "b").unwrap();
  match gate_b.borrow_mut().inner {
    GateInner::Unary { gate_type: UnaryGateType::Noop, ref mut input } => *input = Some(result),
    _ => panic!(),
  }

  run_circuit(evaluable_gates)
}

struct Gate<'a> {
  identifier: &'a str,
  inner: GateInner,
  outputs: Vec<GateOutput<'a>>,
}
enum GateInner {
  Unary { gate_type: UnaryGateType, input: Option<u16> },
  Binary { gate_type: BinaryGateType, lhs: Option<u16>, rhs: Option<u16> },
}
enum UnaryGateType {
  Noop,
  Not,
}
enum BinaryGateType {
  And,
  Or,
  Lshift,
  Rshift,
}
struct GateOutput<'a> {
  output_type: GateOutputType,
  gate: Rc<RefCell<Gate<'a>>>,
}
enum GateOutputType {
  Lhs,
  Rhs,
}

type Gates<'a> = HashMap<&'a str, Rc<RefCell<Gate<'a>>>>;
type EvaluableGates<'a> = Vec<Rc<RefCell<Gate<'a>>>>;
type GateOutputs<'a> = HashMap<&'a str, Vec<GateOutput<'a>>>;

fn construct_circuit(input: &str) -> EvaluableGates {
  let mut gates: Gates = HashMap::new();
  let mut evaluable_gates: EvaluableGates = Vec::new();
  let mut gate_outputs: GateOutputs = HashMap::new();

  fn add_gate_output<'a>(gates: &Gates<'a>, gate_outputs: &mut GateOutputs<'a>, identifier: &'a str, output_type: GateOutputType, gate: Rc<RefCell<Gate<'a>>>) {
    if let Some(x) = gates.get(identifier) {
      x.borrow_mut().outputs.push(GateOutput { output_type, gate });
    } else {
      gate_outputs.entry(identifier).or_default().push(GateOutput { output_type, gate });
    }
  }

  fn handle_unary_gate<'a>(gates: &mut Gates<'a>, evaluable_gates: &mut EvaluableGates<'a>, gate_outputs: &mut GateOutputs<'a>, input: &'a str, output: &'a str, gate_type: UnaryGateType) {
    let gate = Rc::new(RefCell::new(Gate { identifier: output, inner: GateInner::Unary { gate_type, input: input.parse().ok() }, outputs: gate_outputs.remove(output).unwrap_or_default() }));

    if let GateInner::Unary { input: Some(_), .. } = gate.borrow().inner {
      evaluable_gates.push(Rc::clone(&gate));
    } else {
      add_gate_output(gates, gate_outputs, input, GateOutputType::Rhs, Rc::clone(&gate));
    }

    gates.insert(output, gate);
  }

  fn handle_binary_gate<'a>(gates: &mut Gates<'a>, evaluable_gates: &mut EvaluableGates<'a>, gate_outputs: &mut GateOutputs<'a>, lhs: &'a str, rhs: &'a str, output: &'a str, gate_type: BinaryGateType) {
    let lhs_parsed = lhs.parse().ok();
    let rhs_parsed = rhs.parse().ok();
    let gate = Rc::new(RefCell::new(Gate { identifier: output, inner: GateInner::Binary { gate_type, lhs: lhs_parsed, rhs: rhs_parsed }, outputs: gate_outputs.remove(output).unwrap_or_default() }));

    if lhs_parsed.is_some() && rhs_parsed.is_some() {
      evaluable_gates.push(Rc::clone(&gate));
    } else {
      if lhs_parsed.is_none() {
        add_gate_output(gates, gate_outputs, lhs, GateOutputType::Lhs, Rc::clone(&gate));
      }
      if rhs_parsed.is_none() {
        add_gate_output(gates, gate_outputs, rhs, GateOutputType::Rhs, Rc::clone(&gate));
      }
    }

    gates.insert(output, gate);
  }

  for line in input.lines() {
    let mut words_iter = line.split_whitespace();
    let first_word = words_iter.next().unwrap();

    if first_word == "NOT" {
      let input = words_iter.next().unwrap();
      let output = words_iter.nth(1).unwrap();
      handle_unary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, input, output, UnaryGateType::Not);
      continue;
    }

    let second_word = words_iter.next().unwrap();

    if second_word == "->" {
      let output = words_iter.next().unwrap();
      handle_unary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, first_word, output, UnaryGateType::Noop);
      continue;
    }

    let lhs = first_word;
    let rhs = words_iter.next().unwrap();
    let output = words_iter.nth(1).unwrap();
    match second_word {
      "AND" => handle_binary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, lhs, rhs, output, BinaryGateType::And),
      "OR" => handle_binary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, lhs, rhs, output, BinaryGateType::Or),
      "LSHIFT" => handle_binary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, lhs, rhs, output, BinaryGateType::Lshift),
      "RSHIFT" => handle_binary_gate(&mut gates, &mut evaluable_gates, &mut gate_outputs, lhs, rhs, output, BinaryGateType::Rshift),
      _ => panic!(),
    }
  }

  evaluable_gates
}

fn run_circuit(mut evaluable_gates: EvaluableGates) -> u16 {
  loop {
    let mut new_evaluable_gates = Vec::new();

    for x in evaluable_gates.iter().map(|x| x.borrow()) {
      let value = match x.inner {
        GateInner::Unary { gate_type: UnaryGateType::Noop, input } => input.unwrap(),
        GateInner::Unary { gate_type: UnaryGateType::Not, input } => !input.unwrap(),
        GateInner::Binary { gate_type: BinaryGateType::And, lhs, rhs } => lhs.unwrap() & rhs.unwrap(),
        GateInner::Binary { gate_type: BinaryGateType::Or, lhs, rhs } => lhs.unwrap() | rhs.unwrap(),
        GateInner::Binary { gate_type: BinaryGateType::Lshift, lhs, rhs } => lhs.unwrap() << rhs.unwrap(),
        GateInner::Binary { gate_type: BinaryGateType::Rshift, lhs, rhs } => lhs.unwrap() >> rhs.unwrap(),
      };

      if x.identifier == "a" {
        return value;
      }

      for output in &x.outputs {
        match output.output_type {
          GateOutputType::Lhs => match output.gate.borrow_mut().inner {
            GateInner::Binary { ref mut lhs, .. } => *lhs = Some(value),
            _ => panic!(),
          },
          GateOutputType::Rhs => match output.gate.borrow_mut().inner {
            GateInner::Unary { ref mut input, .. } => *input = Some(value),
            GateInner::Binary { ref mut rhs, .. } => *rhs = Some(value),
          },
        }
        if let GateInner::Unary { input: Some(_), .. } | GateInner::Binary { lhs: Some(_), rhs: Some(_), .. } = output.gate.borrow().inner {
          new_evaluable_gates.push(Rc::clone(&output.gate));
        }
      }
    }

    evaluable_gates = new_evaluable_gates;
  }
}
