use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};
use crate::gdt;
use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler); //Breakpoint exception.
        idt.divide_error.set_handler_fn(divide_error_handler); //divide error.
        unsafe {idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);}
        //double fault exception.
        idt //returns idt so initialisation can be finished.
    };
} ///is lazy static so only created once and persisted.

pub fn init_idt() {
    IDT.load(); //initialise IDT.
}


//breakpoint exception.
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame,
) {
    crate::println!(
        "EXCEPTION: BREAKPOINT\n{:#?}",
        stack_frame
    );
}

//divide error.
extern "x86-interrupt" fn divide_error_handler(
    stack_frame: InterruptStackFrame,
) {
    crate::println!(
        "EXCEPTION: DIVIDE ERROR\n{:#?}",
        stack_frame
    );

    loop {
        x86_64::instructions::hlt();
    }
}



extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

//PIC - Handles interrupts from hardware.
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) //wrap in mutex to protect from contention.
});

//initialise PIC hardware.
pub fn init_pics() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut pics = PICS.lock();

        unsafe {
            pics.initialize();
            pics.write_masks(0xff, 0xff);
        }
    });
}
