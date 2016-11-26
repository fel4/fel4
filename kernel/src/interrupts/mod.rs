mod idt;

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, divide_by_zero_handler);

        idt
    };
}

pub fn init() { IDT.load(); }

extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO!");
    loop {}
}
