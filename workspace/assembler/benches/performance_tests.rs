extern crate assembler;
extern crate criterion;

use assembler::{
  ExecutableAnonymousMemoryMap,
  InstructionStreamHints
};

use criterion::{
  criterion_group,
  criterion_main,
  Criterion,
  black_box
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple function", |b| b.iter(|| {
      let mut map = ExecutableAnonymousMemoryMap::new(4096, false, false).expect("Could not anonymously mmap");
      
      let _function_pointer =
      {
        let mut instruction_stream = map.instruction_stream(&InstructionStreamHints::default());
        
        instruction_stream.emit_alignment(64);
        
        let function_pointer: unsafe extern "C" fn() -> i32 = instruction_stream.nullary_function_pointer();
        
        instruction_stream.push_stack_frame();
        
        instruction_stream.zero_RAX();
        
        instruction_stream.pop_stack_frame_and_return();
        
        let (encoded_bytes, _hints) = instruction_stream.finish();
        
       
        function_pointer
      };
      
      let result = unsafe { _function_pointer() };
    })
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
