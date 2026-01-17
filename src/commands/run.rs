use clap::Args;
use compiler::semantic::{Language, StructLayout, WasmType, Field};
use compiler::backend::eval::Eval;

const ABOUT: &str = "run units and packages";

const LONG_ABOUT: &str = r#"
run is a convenience command.

By default, run will evaluate the `main` function in the current package.
"#;

const NOTE: &str = "Run `beam help Run` for more detailed information";

const LONG_NOTE: &str = r#"
See the `beam Run` command for more information on running packages and
dependencies.
"#;

#[derive(Args)]
#[command(visible_alias = "r")]
#[command(about = ABOUT, long_about = LONG_ABOUT)]
#[command(after_help = NOTE, after_long_help = LONG_NOTE)]
#[command(flatten_help = true)]
pub(crate) struct RunCommand {
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long)]
    name: String,
}

fn entry() {
    let mut l = Eval::new();

    // 1. Define a struct at "compile-time" (Stage 0)
    l.define_struct(StructLayout {
        name: "Point".to_string(),
        size: 8,
        fields: vec![
            Field { name: "x".to_string(), offset: 0, ty: WasmType::I32 },
            Field { name: "y".to_string(), offset: 4, ty: WasmType::I32 },
        ],
    });

    // 2. Use MOP to generate a specialized "Serializer" in WASM (Stage 1)
    println!("Generating specialized serializer for 'Point'...");
    
    let struct_name = "Point";

    let code = l.quote(|l0| {
        // This block runs at Stage 0 (comptime) but generates Stage 1 code (runtime)
        let layout = l0.reflect_struct(struct_name).expect("Struct not found");
        
        println!("  [MOP] Reflecting on '{}' (size: {} bytes)", layout.name, layout.size);
        
        let mut final_code = l0.int(0); // Dummy start

        for field in &layout.fields {
            println!("  [MOP] Generating load for field '{}' at offset {}", field.name, field.offset);
            let ptr = l0.int(0); // Base pointer for each load
            let val = l0.load_i32(ptr, field.offset);
            final_code = l0.add(final_code, val); // Just a dummy "collection" of values
        }
        
        final_code
    });

    println!("\nGenerated WASM instructions for 'Point' serializer:");
    for instr in &code.instrs {
        println!("  {:?}", instr);
    }

    // 3. Demonstrate Capability checking at Stage 0
    println!("\nChecking capabilities...");
    let has_fs = l.use_cap("file_system");
    println!("  Has file_system capability: {}", has_fs);

    if !has_fs {
        println!("  [Stage 0] Warning: Code will be generated without FS support.");
    }

    // 4. Loop Unrolling (Stage 0 loop generating Stage 1 code)
    println!("\nGenerating unrolled loop (3 iterations)...");
    let unrolled = l.quote(|l0| {
        let mut sum = l0.int(0);
        for i in 1..=3 {
            println!("  [Stage 0] Unrolling iteration {}", i);
            let val = l0.int(i as i32);
            sum = l0.add(sum, val);
        }
        sum
    });

    println!("\nGenerated WASM instructions for unrolled loop:");
    for instr in &unrolled.instrs {
        println!("  {:?}", instr);
    }

    println!("\n--- Final WASM (WAT) Output ---");
    println!("(module");
    println!("  (import \"env\" \"malloc\" (func $malloc (param i32) (result i32)))");
    println!("  (memory 1)");
    println!("  (func $serialize_point (param $ptr i32) (result i32)");
    for instr in &code.instrs {
        println!("    {}", instr.to_wat());
    }
    println!("    return");
    println!("  )");
    println!("");
    println!("  (func $unrolled_sum (result i32)");
    for instr in &unrolled.instrs {
        println!("    {}", instr.to_wat());
    }
    println!("  )");
    println!(")");
}

impl RunCommand {
    pub(crate) fn run(&self) {
        for i in 0..self.count {
            println!("--- Run iteration {} ---", i + 1);
            entry();
        }
    }
}
