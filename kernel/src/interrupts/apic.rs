mod legacy_pic {
    use x86::io::{inb, outb};

    const PIC1_CMD_PORT: u16 = 0x20;
    const PIC1_DATA_PORT: u16 = (PIC1_CMD_PORT + 1);
    const PIC1_OFFSET: u8 = 0x20;
    const PIC2_CMD_PORT: u16 = 0xA0;
    const PIC2_DATA_PORT: u16 = (PIC2_CMD_PORT + 1);
    const PIC2_OFFSET: u8 = 0x28;

    const ICW_INIT: u8 = 0x11;

    const CHAIN_MASTER: u8 = 0x4;
    const CHAIN_SLAVE: u8 = 0x2;

    pub fn disable() {
        remap_pic(PIC1_OFFSET, PIC2_OFFSET);
        unsafe {
            outb(PIC1_DATA_PORT, 0xFF);
            outb(PIC2_DATA_PORT, 0xFF);
        }
    }

    pub fn remap_pic(new_pic1_offset: u8, new_pic2_offset: u8) {
        let a1 = unsafe { inb(PIC1_DATA_PORT) };
        let a2 = unsafe { inb(PIC2_DATA_PORT) };

        unsafe {
            // send the initialise command to both PICs.
            outb(PIC1_CMD_PORT, ICW_INIT);
            outb(PIC2_CMD_PORT, ICW_INIT);

            // send the new offsets to the data ports
            outb(PIC1_DATA_PORT, new_pic1_offset);
            outb(PIC2_DATA_PORT, new_pic2_offset);

            // chain the slave PIC to the master PIC
            outb(PIC1_DATA_PORT, CHAIN_MASTER);
            outb(PIC2_DATA_PORT, CHAIN_SLAVE);

            // finalize the PIC setup.
            outb(PIC1_DATA_PORT, 0x1);
            outb(PIC2_DATA_PORT, 0x1);

            // reapply the previous PIC masks.
            outb(PIC1_DATA_PORT, a1);
            outb(PIC2_DATA_PORT, a2);
        }
    }
}

pub fn init() {
    legacy_pic::disable();
}