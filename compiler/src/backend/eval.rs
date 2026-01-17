use crate::semantic::{Language, StructLayout, Liftable, WasmCode, Instr};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EvalVal {
    Value(i32),      // Simplified: all values as i32 for now
    Code(WasmCode),
}

impl EvalVal {
    fn as_int(&self) -> i32 {
        match self {
            EvalVal::Value(v) => *v,
            _ => panic!("Expected value, found code"),
        }
    }

    fn as_code(self) -> WasmCode {
        match self {
            EvalVal::Code(c) => c,
            EvalVal::Value(v) => WasmCode { instrs: vec![Instr::I32Const(v)] },
        }
    }
}

pub struct Eval {
    pub stage: u32,
    pub structs: HashMap<String, StructLayout>,
    pub caps: HashMap<String, bool>,
}

impl Eval {
    pub fn new() -> Self {
        Self {
            stage: 0,
            structs: HashMap::new(),
            caps: HashMap::new(),
        }
    }
}

impl Language for Eval {
    type Int = EvalVal;
    type Bool = EvalVal;
    type Ptr<T> = EvalVal;
    type Struct = ();
    type Code<T> = WasmCode;
    type Cap = bool;

    fn int(&mut self, v: i32) -> EvalVal {
        if self.stage == 0 {
            EvalVal::Value(v)
        } else {
            EvalVal::Code(WasmCode { instrs: vec![Instr::I32Const(v)] })
        }
    }

    fn bool(&mut self, v: bool) -> EvalVal {
        if self.stage == 0 {
            EvalVal::Value(if v { 1 } else { 0 })
        } else {
            EvalVal::Code(WasmCode { instrs: vec![Instr::I32Const(if v { 1 } else { 0 })] })
        }
    }

    fn add(&mut self, a: EvalVal, b: EvalVal) -> EvalVal {
        if self.stage == 0 {
            EvalVal::Value(a.as_int() + b.as_int())
        } else {
            let mut c1 = a.as_code();
            let c2 = b.as_code();
            c1.instrs.extend(c2.instrs);
            c1.instrs.push(Instr::I32Add);
            EvalVal::Code(c1)
        }
    }

    fn if_<T>(
        &mut self,
        cond: EvalVal,
        then_: impl FnOnce(&mut Self) -> T,
        else_: impl FnOnce(&mut Self) -> T,
    ) -> T {
        if self.stage == 0 {
            if cond.as_int() != 0 { then_(self) } else { else_(self) }
        } else {
            // This would require generating WASM 'if' instructions
            unimplemented!("Staged if-generation")
        }
    }

    fn func<A, R>(&mut self, _body: impl FnOnce(&mut Self, A) -> R) -> fn(A) -> R {
        unimplemented!()
    }

    fn call<A, R>(&mut self, f: fn(A) -> R, arg: A) -> R {
        f(arg)
    }

    fn alloc_struct(&mut self, layout: StructLayout) -> EvalVal {
        if self.stage == 0 {
            EvalVal::Value(0) // Mock pointer
        } else {
            EvalVal::Code(WasmCode {
                instrs: vec![
                    Instr::I32Const(layout.size as i32),
                    Instr::Call("malloc".to_string()),
                ]
            })
        }
    }

    fn load_i32(&mut self, p: EvalVal, offset: u32) -> EvalVal {
        if self.stage == 0 {
            EvalVal::Value(0)
        } else {
            let mut c = p.as_code();
            c.instrs.push(Instr::I32Load { offset });
            EvalVal::Code(c)
        }
    }

    fn store_i32(&mut self, p: EvalVal, offset: u32, v: EvalVal) {
        if self.stage != 0 {
            let mut c = p.as_code();
            c.instrs.extend(v.as_code().instrs);
            c.instrs.push(Instr::I32Store { offset });
            // In this prototype, we don't have a way to emit effects easily
            // unless we return the code.
        }
    }

    // Staging
    fn quote<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> Self::Code<T> {
        self.stage += 1;
        let val = f(self);
        self.stage -= 1;

        // Safety: for the prototype, we assume T is EvalVal and move it out
        // properly to avoid double-dropping the internal Vec.
        unsafe {
            let ev: EvalVal = std::mem::transmute_copy(&val);
            std::mem::forget(val);
            ev.as_code()
        }
    }

    fn splice<T>(&mut self, code: Self::Code<T>) -> T {
        if self.stage > 0 {
            let ev = EvalVal::Code(code);
            // Safety: assume T is EvalVal and move it into the return value
            unsafe {
                let t: T = std::mem::transmute_copy(&ev);
                std::mem::forget(ev);
                t
            }
        } else {
            panic!("Cannot splice outside of quote in this prototype")
        }
    }

    fn lift<T: Liftable>(&mut self, val: T) -> Self::Code<T> {
        val.lift_to_wasm()
    }

    // MOP
    fn reflect_struct(&mut self, name: &str) -> Option<StructLayout> {
        self.structs.get(name).cloned()
    }

    fn define_struct(&mut self, layout: StructLayout) {
        self.structs.insert(layout.name.clone(), layout);
    }

    // Capabilities
    fn use_cap(&mut self, name: &str) -> Self::Cap {
        *self.caps.get(name).unwrap_or(&false)
    }
}