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

enum OpType {
    ADD,
    SUB,
    INC,
    DEC,
    AND,
    OR,
    XOR,
    CP,
    SHIFT,
}

impl CPU {

    fn init() -> CPU {
        CPU { registers: Registers::init(), flags: Flags::init() } 
        
    }

    fn execute(&mut self, code: u8, val: u8) {
        match code {
            // add register
            0x80 => self.adda_r8(self.registers.b),
            0x81 => self.adda_r8(self.registers.c),
            0x82 => self.adda_r8(self.registers.d),
            0x83 => self.adda_r8(self.registers.e),
            0x84 => self.adda_r8(self.registers.h),
            0x85 => self.adda_r8(self.registers.l),
            0x87 => self.adda_r8(self.registers.a),
            // add immediate
            0xC6 => self.adda_r8(val),
            // adc
            0x88 => self.add_with_carry(self.registers.b),
            0x89 => self.add_with_carry(self.registers.c),
            0x8A => self.add_with_carry(self.registers.d),
            0x8B => self.add_with_carry(self.registers.e),
            0x8C => self.add_with_carry(self.registers.h),
            0x8D => self.add_with_carry(self.registers.l),
            0x8F => self.add_with_carry(self.registers.a),
            // adc immediate
            0xCE => self.add_with_carry(val),
            // sub register
            0x90 => self.suba_r8(self.registers.b),
            0x91 => self.suba_r8(self.registers.c),
            0x92 => self.suba_r8(self.registers.d),
            0x93 => self.suba_r8(self.registers.e),
            0x94 => self.suba_r8(self.registers.h),
            0x95 => self.suba_r8(self.registers.l),
            0x97 => self.suba_r8(self.registers.a),
            // sub immediate
            0xD6 => self.suba_r8(val),
            // sbc
            0x98 => self.sub_with_carry(self.registers.b),
            0x99 => self.sub_with_carry(self.registers.c),
            0x9A => self.sub_with_carry(self.registers.d),
            0x9B => self.sub_with_carry(self.registers.e),
            0x9C => self.sub_with_carry(self.registers.h),
            0x9D => self.sub_with_carry(self.registers.l),
            0x9F => self.sub_with_carry(self.registers.a),
            // sbc immediate
            0xDE => self.sub_with_carry(val),
            // inc
            0x04 => self.registers.b = self.inc(self.registers.b),
            0x0C => self.registers.c = self.inc(self.registers.c),
            0x14 => self.registers.d = self.inc(self.registers.d),
            0x1C => self.registers.e = self.inc(self.registers.e),
            0x24 => self.registers.h = self.inc(self.registers.h),
            0x2C => self.registers.l = self.inc(self.registers.l),
            0x3C => self.registers.a = self.inc(self.registers.a),
            // dec
            0x05 => self.registers.b = self.dec(self.registers.b),
            0x0D => self.registers.c = self.dec(self.registers.c),
            0x15 => self.registers.d = self.dec(self.registers.d),
            0x1D => self.registers.e = self.dec(self.registers.e),
            0x25 => self.registers.h = self.dec(self.registers.h),
            0x2D => self.registers.l = self.dec(self.registers.l),
            0x3D => self.registers.a = self.dec(self.registers.a),
            // and
            0xA0 => self.and(self.registers.b),
            0xA1 => self.and(self.registers.c),
            0xA2 => self.and(self.registers.d),
            0xA3 => self.and(self.registers.e),
            0xA4 => self.and(self.registers.h),
            0xA5 => self.and(self.registers.l),
            0xA7 => self.and(self.registers.a),
            // and immediate
            0xE6 => self.and(val),
            // or
            0xB0 => self.or(self.registers.b),
            0xB1 => self.or(self.registers.c),
            0xB2 => self.or(self.registers.d),
            0xB3 => self.or(self.registers.e),
            0xB4 => self.or(self.registers.h),
            0xB5 => self.or(self.registers.l),
            0xB7 => self.or(self.registers.a),
            // or immediate
            0xF6 => self.xor(val),
            // xor
            0xA8 => self.xor(self.registers.b),
            0xA9 => self.xor(self.registers.c),
            0xAA => self.xor(self.registers.d),
            0xAB => self.xor(self.registers.e),
            0xAC => self.xor(self.registers.h),
            0xAD => self.xor(self.registers.l),
            0xAF => self.xor(self.registers.a),
            // xor immediate
            0xEE => self.xor(val),
            // cp
            0xB8 => self.compare(self.registers.b),
            0xB9 => self.compare(self.registers.c),
            0xBA => self.compare(self.registers.d),
            0xBB => self.compare(self.registers.e),
            0xBC => self.compare(self.registers.h),
            0xBD => self.compare(self.registers.l),
            0xBF => self.compare(self.registers.a),
            // cp immediate
            0xFE => self.compare(val),
            // prefix
            0xCB => self.execute_cb(val, val),
            _ => println!("ciao"),
        }
    }

    fn execute_cb(&mut self, code: u8, val: u8) {
        match code {
            // rotates
            // left
            0x10 => self.registers.b = self.rl(self.registers.b),
            0x11 => self.registers.c = self.rl(self.registers.c),
            0x12 => self.registers.d = self.rl(self.registers.d),
            0x13 => self.registers.e = self.rl(self.registers.e),
            0x14 => self.registers.h = self.rl(self.registers.h),
            0x15 => self.registers.l = self.rl(self.registers.l),
            0x17 => self.registers.a = self.rl(self.registers.a),
            // right
            0x18 => self.registers.b = self.rr(self.registers.b),
            0x19 => self.registers.c = self.rr(self.registers.c),
            0x1A => self.registers.d = self.rr(self.registers.d),
            0x1B => self.registers.e = self.rr(self.registers.e),
            0x1C => self.registers.h = self.rr(self.registers.h),
            0x1D => self.registers.l = self.rr(self.registers.l),
            0x1F => self.registers.a = self.rr(self.registers.a),
            // left circular
            0x00 => self.registers.b = self.rlc(self.registers.b),
            0x01 => self.registers.c = self.rlc(self.registers.c),
            0x02 => self.registers.d = self.rlc(self.registers.d),
            0x03 => self.registers.e = self.rlc(self.registers.e),
            0x04 => self.registers.h = self.rlc(self.registers.h),
            0x05 => self.registers.l = self.rlc(self.registers.l),
            0x07 => self.registers.a = self.rlc(self.registers.a),
            // right circular
            0x08 => self.registers.b = self.rrc(self.registers.b),
            0x09 => self.registers.c = self.rrc(self.registers.c),
            0x0A => self.registers.d = self.rrc(self.registers.d),
            0x0B => self.registers.e = self.rrc(self.registers.e),
            0x0C => self.registers.h = self.rrc(self.registers.h),
            0x0D => self.registers.l = self.rrc(self.registers.l),
            0x0F => self.registers.a = self.rrc(self.registers.a),
            // shift
            // left
            0x20 => self.registers.b = self.shift_left(self.registers.b),
            0x21 => self.registers.c = self.shift_left(self.registers.c),
            0x22 => self.registers.d = self.shift_left(self.registers.d),
            0x23 => self.registers.e = self.shift_left(self.registers.e),
            0x24 => self.registers.h = self.shift_left(self.registers.h),
            0x25 => self.registers.l = self.shift_left(self.registers.l),
            0x27 => self.registers.a = self.shift_left(self.registers.a),
            // right
            0x28 => self.registers.b = self.shift_right(self.registers.b),
            0x29 => self.registers.c = self.shift_right(self.registers.c),
            0x2A => self.registers.d = self.shift_right(self.registers.d),
            0x2B => self.registers.e = self.shift_right(self.registers.e),
            0x2C => self.registers.h = self.shift_right(self.registers.h),
            0x2D => self.registers.l = self.shift_right(self.registers.l),
            0x2F => self.registers.a = self.shift_right(self.registers.a),
            // right logical
            0x38 => self.registers.b = self.srl(self.registers.b),
            0x39 => self.registers.c = self.srl(self.registers.c),
            0x3A => self.registers.d = self.srl(self.registers.d),
            0x3B => self.registers.e = self.srl(self.registers.e),
            0x3C => self.registers.h = self.srl(self.registers.h),
            0x3D => self.registers.l = self.srl(self.registers.l),
            0x3F => self.registers.a = self.srl(self.registers.a),
            _ => println!("ciao"),
        }
    }

    fn srl(&mut self, val: u8) -> u8 {
        self.shift_op(false, val, 0)
    }

    fn shift_right(&mut self, val: u8) -> u8 {
        self.shift_op(false, val, val & 0x80)
    }

    fn shift_left(&mut self, val: u8) -> u8 {
        self.shift_op(true, val, 0xFF)
    }

    fn rrc(&mut self, val: u8) -> u8 {
        self.shift_op(false, val, (val & 0x01) << 7)
    }

    fn rlc(&mut self, val: u8) -> u8 {
        self.shift_op(true, val, (val & 0x80) >> 7)
    }

    fn rr(&mut self, val: u8) -> u8 {
        let cb = u8::from(self.flags.carry_flag) << 7;
        self.shift_op(false, val, cb)
    }

    fn rl(&mut self, val: u8) -> u8 {
        let cb = u8::from(self.flags.carry_flag);
        self.shift_op(true, val, cb)
    }

    fn shift_op(&mut self, is_left: bool, val: u8, or_op: u8) -> u8 {
        let res: u8;
        if is_left {
            let f = |x: u8, y: u8| ((x << y) | (or_op), bool::from(x & 0x80 == 0x80));
            res = self.reg_operation(f, val, 1, OpType::SHIFT);
        } else {
            let f = |x: u8, y: u8| ((x >> y) | (or_op), bool::from(x & 0x01 == 0x01));
            res = self.reg_operation(f, val, 1, OpType::SHIFT);
        }
        res
    }

    fn or(&mut self, val: u8) {
        self.logical_op(|x,y| x | y, self.registers.a, val, OpType::OR);
    }

    fn and(&mut self, val: u8) {
        self.logical_op(|x,y| x & y, self.registers.a, val, OpType::AND);
    }

    fn xor(&mut self, val: u8) {
        self.logical_op(|x,y| x ^ y, self.registers.a, val, OpType::XOR);
    }

    fn logical_op<F: Fn(u8, u8) -> u8>(&mut self, op: F, x: u8, y: u8, optype: OpType) {
        let res = op(x,y);
        let op1 = |x: u8, y: u8| (x&y, false);
        self.set_flags(x,y,op1,optype);
        self.registers.set_a(res);
    }

    fn compare(&mut self, val: u8) {
        self.reg_operation(|x,y| x.overflowing_sub(y), self.registers.a, val, OpType::CP);
    }

    fn inc(&mut self, reg_val: u8) -> u8 {
        let res = self.reg_operation(|x,y| x.overflowing_add(y), reg_val, 1, OpType::INC);
        res
    }

    fn dec(&mut self, reg_val: u8) -> u8 {
        let res = self.reg_operation(|x,y| x.overflowing_sub(y), reg_val, 1, OpType::DEC);
        res
    }

    fn adda_r8(&mut self, val:u8) { 
        let res = self.reg_operation(|x,y| x.overflowing_add(y), self.registers.a, val, OpType::ADD);
        self.registers.set_a(res);
    }

    fn suba_r8(&mut self, val:u8) {
        let res = self.reg_operation(|x,y| x.overflowing_sub(y), self.registers.a, val, OpType::SUB);
        self.registers.set_a(res);
    }

    fn add_with_carry(&mut self, val: u8) {
        let y = val+u8::from(self.flags.carry_flag);
        self.reg_operation(|x,y| x.overflowing_add(y), self.registers.a, y, OpType::ADD);
    }

    fn sub_with_carry(&mut self, val: u8) {
        let y = val+u8::from(self.flags.carry_flag);
        self.reg_operation(|x,y| x.overflowing_sub(y), self.registers.a, y, OpType::SUB);
    }

    fn reg_operation<F>(&mut self, op: F, reg1: u8, reg2: u8, optype: OpType) -> u8
        where F: Fn(u8, u8) -> (u8, bool) {

            let (res, _) = op(reg1, reg2);
            self.set_flags(reg1, reg2, op, optype);

            res
    }

    fn set_flags<F: Fn(u8, u8) -> (u8, bool)>(&mut self, x: u8, y: u8, f: F, optype: OpType) {
        let (res, of) = f(x, y);
        self.flags.zero_flag = res == 0;
        match optype {
            OpType::ADD => {
                self.flags.carry_flag = of;
                self.flags.sub_flag = false;
                self.flags.hc_flag = (x & 0x0F) + (y & 0x0F) > 0x0F
            },
            OpType::SUB | OpType::CP => {
                self.flags.carry_flag = of;
                self.flags.sub_flag = true;
                self.flags.hc_flag = (x & 0x0F) < (y & 0x0F)
            },
            OpType::INC => {
                self.flags.sub_flag = false;
                self.flags.hc_flag = (x & 0x0F) + 1 > 0x0F
            },
            OpType::DEC => {
                self.flags.sub_flag = false;
                self.flags.hc_flag = (x & 0x0F) < (0x01)
            },
            OpType::AND => {
                self.flags.carry_flag = false;
                self.flags.sub_flag = false;
                self.flags.hc_flag = true;
            },
            OpType::OR | OpType::XOR => {
                self.flags.carry_flag = false;
                self.flags.sub_flag = false;
                self.flags.hc_flag = false;
            },
            OpType::SHIFT => {
                self.flags.carry_flag = of;
                self.flags.sub_flag = false;
                self.flags.hc_flag = false
            }
            _ => println!("ciao")
        }
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
        cpu.execute(0x80, 0);
        assert_eq!(cpu.registers.a, 24)
    }

    #[test]
    fn sub_test() {
        let mut cpu = CPU::init();
        cpu.registers.a = 16;
        cpu.registers.b = 8;
        cpu.execute(0x90, 0);
        assert_eq!(cpu.registers.a, 8);
        assert_eq!(cpu.flags.hc_flag, true)
    }

    #[test]
    fn inc_test() {
        let mut cpu = CPU::init();
        cpu.execute(0x04, 0);
        assert_eq!(cpu.registers.b, 1)
    }

    #[test]
    fn and_test() {
        let mut cpu = CPU::init();
        cpu.registers.a = 0b0110;
        cpu.registers.b = 0b0010;
        cpu.execute(0xa0, 0);
        assert_eq!(cpu.registers.a, 0b0010)
    }

    #[test]
    fn or_test() {
        let mut cpu = CPU::init();
        cpu.registers.a = 0x5;
        cpu.registers.b = 0xA;
        cpu.execute(0xb0, 0);
        assert_eq!(cpu.registers.a, 0xF)
    }

    #[test]
    fn shift_test() {
        let mut cpu = CPU::init();
        cpu.registers.b = 0b10010000;
        cpu.flags.carry_flag = true;
        cpu.execute(0xcb, 0x10);
        assert_eq!(cpu.registers.b, 0b00100001);
        assert_eq!(cpu.flags.carry_flag, true);
    }

    #[test]
    fn right_shift_test() {
        let mut cpu = CPU::init();
        cpu.registers.b = 0b10010000;
        cpu.flags.carry_flag = true;
        cpu.execute(0xcb, 0x28);
        assert_eq!(cpu.registers.b, 0b11001000);
        assert_eq!(cpu.flags.carry_flag, false);
    }

    #[test]
    fn srl_test() {
        let mut cpu = CPU::init();
        cpu.registers.b = 0b10010000;
        cpu.flags.carry_flag = true;
        cpu.execute(0xcb, 0x38);
        assert_eq!(cpu.registers.b, 0b01001000);
        assert_eq!(cpu.flags.carry_flag, false);
    }

}
