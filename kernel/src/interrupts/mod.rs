mod idt;

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp
                      sub rsp, 8 // re-align stack pointer to 16-byte alignment.
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame) -> !)
                      : "rdi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("pop rsi
                      mov rdi, rsp
                      sub rsp, 8 // re-align stack pointer to 16-byte alignment.
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame, u64) -> !)
                      : "rdi", "rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        idt
    };
}

bitflags! {
    flags PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0,
        const CAUSED_BY_WRITE = 1 << 1,
        const USER_MODE = 1 << 2,
        const MALFORMED_TABLE = 1 << 3,
        const INSTRUCTION_FETCH = 1 << 4,
    }
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_ptr: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub fn init() { IDT.load(); }

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: DIVIDE BY ZERO!\n{:#?}", unsafe { &*stack_frame });
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}", stack_frame.instruction_ptr, stack_frame);
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    use x86::controlregs;
    print!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\
            \nwith error code: {:?}\n{:#?}",
        unsafe { controlregs::cr2() }, PageFaultErrorCode::from_bits(error_code).unwrap(), unsafe { &*stack_frame });
    loop {}
}
