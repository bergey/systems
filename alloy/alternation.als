enum State { Critical, Normal, Wait }

abstract sig Process {
  	var state: State
}

one sig P1 extends Process {}
one sig P2 extends Process {}

assert safe {
	not (P1.state = Critical and P2.state = Critical)
}

enum Bool { True, False }

// mutex
one sig M {
  var lock: Bool,
  var turn: Process
}

fun other[p: Process] : Process  {
  p = P1 implies P2 else P1
}  

pred init {
  P1.state = Normal
  P2.state = Normal
  M.lock = False
  // turn can be P1 or P2
}

pred wait[p: Process] {
  p.state = Normal
  p.state' = Wait
  // rest unchanged
  M.lock' = M.lock
  M.turn' = M.turn
}

pred finish_critical[p: Process] {
  p.state = Critical
  p.state' = Normal
  M.lock' = False
  M.turn' = M.turn
}

pred enter_critical[p: Process] {
  p.state = Wait
  M.lock = False
  M.turn = p
  M.lock' = True
  M.turn' = p.other
  p.state' = Critical
}

// does not include the no-op / stutter step
pred step[p: Process] {
  wait[p] or enter_critical[p] or finish_critical[p]
}

fact {
  once init && always (one p: Process | step[p] and p.other.state = p.other.state')
}

check safe

assert live {
  eventually P1.state = Critical and eventually P2.state = Critical
}

check live
