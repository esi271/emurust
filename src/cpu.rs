struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Registers {

    fn init() -> Registers {
        Registers { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0 }
    }

    fn set_a(&mut self, val: u8) {
        self.a = val;
    }

}

struct Flags {
    zero_flag: bool,
    sub_flag: bool,
    hc_flag: bool,
    carry_flag: bool,
}

impl Flags {
    fn init() -> Flags {
        Flags { zero_flag: false, sub_flag: false, hc_flag: false, carry_flag: false }
    }
}

struct CPU {
    registers: Registers,
    flags: Flags,
}

impl CPU {

    fn init() -> CPU {
        CPU { registers: Registers::init(), flags: Flags::init() } 
        
    }

    fn execute(&mut self, code: u8) {
        match code {
            0x80 => self.adda_r8(self.registers.b),
            0x81 => self.adda_r8(self.registers.c),
            0x82 => self.adda_r8(self.registers.d),
            0x83 => self.adda_r8(self.registers.e),
            0x84 => self.adda_r8(self.registers.h),
            0x85 => self.adda_r8(self.registers.l),
            0x87 => self.adda_r8(self.registers.a),
            0x90 => self.suba_r8(self.registers.b),
            0x91 => self.suba_r8(self.registers.c),
            0x92 => self.suba_r8(self.registers.d),
            0x93 => self.suba_r8(self.registers.e),
            0x94 => self.suba_r8(self.registers.h),
            0x95 => self.suba_r8(self.registers.l),
            0x97 => self.suba_r8(self.registers.a),
            _ => println!("ciao"),
        }
    }

    fn adda_r8(&mut self, val:u8) { 
        let a = self.registers.a;
        let (res, of) = a.overflowing_add(val);

        self.flags.carry_flag = of;
        self.flags.zero_flag = res == 0;
        self.flags.sub_flag = false;
        self.flags.hc_flag = ((a&0xf) + (val&0xf))&0x10 == 0x10;

        self.registers.set_a(res);
    }

    fn suba_r8(&mut self, val:u8) {
        let a = self.registers.a;
        let (res, of) = a.overflowing_sub(val);

        self.flags.carry_flag = of;
        self.flags.sub_flag = true;
        self.flags.zero_flag = res == 0;
        self.flags.hc_flag = (a&0xf) < (val&0xf);

        self.registers.set_a(res);
    }


}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    
    #[test]
    fn ex_test() {
        let mut cpu = CPU::init();
        cpu.registers.a = 50;
        cpu.registers.b = 230;
        cpu.execute(0x80);
        assert_eq!(cpu.registers.a, 24)
    }

    #[test]
    fn sub_test() {
        let mut cpu = CPU::init();
        cpu.registers.a = 16;
        cpu.registers.b = 8;
        cpu.execute(0x90);
        assert_eq!(cpu.registers.a, 8);
        assert_eq!(cpu.flags.hc_flag, true)
    }

}
