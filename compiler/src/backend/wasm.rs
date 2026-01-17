use crate::semantic::{Language, StructLayout, Liftable, WasmCode, Instr};
use std::collections::HashMap;

pub struct Wasm {
    pub structs: HashMap<String, StructLayout>,
}

impl Wasm {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
        }
    }
}

impl Language for Wasm {
    type Int = WasmCode;
    type Bool = WasmCode;
    type Ptr<T> = WasmCode;
    type Struct = ();
    type Code<T> = WasmCode;
    type Cap = ();

    fn int(&mut self, v: i32) -> WasmCode {
        WasmCode { instrs: vec![Instr::I32Const(v)] }
    }

    fn bool(&mut self, v: bool) -> WasmCode {
        WasmCode { instrs: vec![Instr::I32Const(if v { 1 } else { 0 })] }
    }

    fn add(&mut self, mut a: WasmCode, b: WasmCode) -> WasmCode {
        a.instrs.extend(b.instrs);
        a.instrs.push(Instr::I32Add);
        a
    }

    fn if_<T>(
        &mut self,
        _cond: WasmCode,
        _then_: impl FnOnce(&mut Self) -> T,
        _else_: impl FnOnce(&mut Self) -> T,
    ) -> T {
        unimplemented!()
    }

    fn func<A, R>(&mut self, _body: impl FnOnce(&mut Self, A) -> R) -> fn(A) -> R {
        unimplemented!()
    }

    fn call<A, R>(&mut self, _f: fn(A) -> R, _arg: A) -> R {
        unimplemented!()
    }

    fn alloc_struct(&mut self, layout: StructLayout) -> WasmCode {
        WasmCode {
            instrs: vec![
                Instr::I32Const(layout.size as i32),
                Instr::Call("malloc".to_string()),
            ]
        }
    }

    fn load_i32(&mut self, mut p: WasmCode, offset: u32) -> WasmCode {
        p.instrs.push(Instr::I32Load { offset });
        p
    }

    fn store_i32(&mut self, mut p: WasmCode, offset: u32, v: WasmCode) {
        p.instrs.extend(v.instrs);
        p.instrs.push(Instr::I32Store { offset });
    }

    fn quote<T>(&mut self, _f: impl FnOnce(&mut Self) -> T) -> Self::Code<T> {
        unimplemented!()
    }

    fn splice<T>(&mut self, code: Self::Code<T>) -> T {
        unsafe { std::mem::transmute_copy(&code) }
    }

    fn lift<T: Liftable>(&mut self, val: T) -> Self::Code<T> {
        val.lift_to_wasm()
    }

    fn reflect_struct(&mut self, name: &str) -> Option<StructLayout> {
        self.structs.get(name).cloned()
    }

    fn define_struct(&mut self, layout: StructLayout) {
        self.structs.insert(layout.name.clone(), layout);
    }

    fn use_cap(&mut self, _name: &str) -> Self::Cap {
        ()
    }
}