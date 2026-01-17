#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Instr {
    I32Const(i32),
    I32Add,
    I32Load { offset: u32 },
    I32Store { offset: u32 },
    Call(String),
    LocalGet(u32),
    LocalSet(u32),
}

impl Instr {
    pub fn to_wat(&self) -> String {
        match self {
            Instr::I32Const(v) => format!("i32.const {}", v),
            Instr::I32Add => "i32.add".to_string(),
            Instr::I32Load { offset } => format!("i32.load offset={}", offset),
            Instr::I32Store { offset } => format!("i32.store offset={}", offset),
            Instr::Call(name) => format!("call ${}", name),
            Instr::LocalGet(idx) => format!("local.get {}", idx),
            Instr::LocalSet(idx) => format!("local.set {}", idx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WasmCode {
    pub instrs: Vec<Instr>,
}

impl WasmCode {
    pub fn empty() -> Self {
        Self { instrs: Vec::new() }
    }
}

pub trait Language {
    type Int;
    type Bool;
    type Ptr<T>;
    type Struct;

    // Staging
    type Code<T>;

    // values
    fn int(&mut self, v: i32) -> Self::Int;
    fn bool(&mut self, v: bool) -> Self::Bool;

    // arithmetic
    fn add(&mut self, a: Self::Int, b: Self::Int) -> Self::Int;

    // control
    fn if_<T>(
        &mut self,
        cond: Self::Bool,
        then_: impl FnOnce(&mut Self) -> T,
        else_: impl FnOnce(&mut Self) -> T,
    ) -> T;

    // functions
    fn func<A, R>(&mut self, body: impl FnOnce(&mut Self, A) -> R) -> fn(A) -> R;
    fn call<A, R>(&mut self, f: fn(A) -> R, arg: A) -> R;

    // memory
    fn alloc_struct(&mut self, layout: StructLayout) -> Self::Ptr<Self::Struct>;
    fn load_i32(&mut self, p: Self::Ptr<Self::Struct>, offset: u32) -> Self::Int;
    fn store_i32(&mut self, p: Self::Ptr<Self::Struct>, offset: u32, v: Self::Int);

    // Staging operations
    fn quote<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> Self::Code<T>;
    fn splice<T>(&mut self, code: Self::Code<T>) -> T;
    fn lift<T: Liftable>(&mut self, val: T) -> Self::Code<T>;

    // MOP
    fn reflect_struct(&mut self, name: &str) -> Option<StructLayout>;
    fn define_struct(&mut self, layout: StructLayout);

    // Capabilities
    type Cap;
    fn use_cap(&mut self, name: &str) -> Self::Cap;
}

pub trait Liftable: Sized {
    fn lift_to_wasm(&self) -> WasmCode;
}

impl Liftable for i32 {
    fn lift_to_wasm(&self) -> WasmCode {
        WasmCode { instrs: vec![Instr::I32Const(*self)] }
    }
}

impl Liftable for bool {
    fn lift_to_wasm(&self) -> WasmCode {
        WasmCode { instrs: vec![Instr::I32Const(if *self { 1 } else { 0 })] }
    }
}

#[derive(Debug, Clone)]
pub struct StructLayout {
    pub name: String,
    pub size: u32,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub offset: u32,
    pub ty: WasmType,
}