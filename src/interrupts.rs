pub mod legacy_pic {
    use x86_64::instructions::port::Port;

    const CMD_INIT: u8 = 0x11;
    const CMD_END_OF_INTERRUPT: u8 = 0x20;
    const CMD_8086_MODE:u8 = 0x01;

    const PIC1_CMD: u16 = 0x20;
    const PIC1_DATA: u16 = 0x21;
    const PIC2_CMD:u16 = 0xA0;
    const PIC2_DATA:u16 = 0xA1;

    struct Pic {
        offset: u8,
        command: Port<u8>,
        data: Port<u8>
    }

    impl Pic {
        fn handles_interrupt(&self, interrupt_id: u8) -> bool {
            self.offset <= interrupt_id && interrupt_id < self.offset + 8
        }

        unsafe fn end_of_interrupt(&mut self) {
            self.command.write(CMD_END_OF_INTERRUPT);
        }
    }

    pub struct ChainedPics {
        pics: [Pic; 2]
    }

    impl ChainedPics {
        pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
            ChainedPics {
                pics: [
                    Pic {
                        offset: offset1,
                        command: Port::<u8>::new(PIC1_CMD),
                        data: Port::<u8>::new(PIC1_DATA)
                    },
                    Pic {
                        offset: offset2,
                        command: Port::<u8>::new(PIC2_CMD),
                        data: Port::<u8>::new(PIC2_DATA)
                    }
                ]
            }
        }

        pub unsafe fn initialize(&mut self) {
            let mut wait_port = Port::<u8>::new(0x80);
            let mut wait = || { wait_port.write(0); };

            // save mask
            let mask1 = self.pics[0].data.read();
            let mask2 = self.pics[1].data.read();

            // initialize pics
            self.pics[0].command.write(CMD_INIT);
            wait();
            self.pics[1].command.write(CMD_INIT);
            wait();

            // set new offsets
            self.pics[0].data.write(self.pics[0].offset);
            wait();
            self.pics[1].data.write(self.pics[1].offset);
            wait();

            // setup pic chaining (aka cascading)
            self.pics[0].data.write(4);
            wait();
            self.pics[1].data.write(2);
            wait();

            // set '8086' mode.
            self.pics[0].data.write(CMD_8086_MODE);
            wait();
            self.pics[1].data.write(CMD_8086_MODE);
            wait();

            // reapply saved masks.
            self.pics[0].data.write(mask1);
            self.pics[1].data.write(mask2);
        }

        pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
            self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
        }

        pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
            if self.handles_interrupt(interrupt_id) {
                if self.pics[1].handles_interrupt(interrupt_id) {
                    self.pics[1].end_of_interrupt();
                }
                // pic[1] is chained through pic[0], so both pics
                // need to be notified for pic[1] interrupts.
                self.pics[0].end_of_interrupt();
            }
        }
    }
}

use self::legacy_pic::ChainedPics;
use spin;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub const TIMER_INTERRUPT_ID: u8 = PIC1_OFFSET;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {
        ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET)
    });
