pub struct Disassembler {}

impl Disassembler {
    pub fn disassemble_list(code: &[u16]) -> Vec<String> {
        let mut disassembled_code: Vec<String> = vec![String::with_capacity(10); code.len()];

        for (i, opcode) in code.iter().enumerate() {
            disassembled_code[i] = Disassembler::disassemble(opcode);
        }

        disassembled_code
    }

    pub fn convert_and_disassemble_list(code: &[u8]) -> Vec<String> {
        Disassembler::disassemble_list(&Disassembler::convert_list(&code))
    }

    pub fn convert_list(code: &[u8]) -> Vec<u16> {
        let mut converted_code: Vec<u16> = Vec::new();

        let mut index = 0;

        while index < code.len() - 1 {
            if let Some(opcode) = Disassembler::concat_opcode(code[index], code[index + 1]) {
                println!("index {}", index);
                println!("opcode {:04X}", opcode);

                index += 1;
                converted_code.push(opcode);
            }

            index += 1;
        }

        converted_code
    }

    pub fn concat_opcode(part1: u8, part2: u8) -> Option<u16> {
        let opcode: u16 = (part1 as u16) << 8 | (part2 as u16);

        if Disassembler::disassemble(&opcode) == "Unknown" {
            return None;
        }

        Some(opcode)
    }

    pub fn disassemble(opcode: &u16) -> String {
        let mut disassembled_code = String::new();

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );

        let nn = nibbles.2 << 4 | nibbles.3;
        let nnn = opcode & 0x0FFF;
        match nibbles {
            (0x0, 0x0, 0xb, _) => disassembled_code = "SCU".to_string(),
            (0x0, 0x0, 0xc, _) => disassembled_code = "SCD".to_string(),
            (0x0, 0x0, 0xe, 0x0) => disassembled_code = "CLS".to_string(),
            (0x0, 0x0, 0xe, 0xe) => disassembled_code = "RET".to_string(),
            (0x0, 0x0, 0xf, 0xb) => disassembled_code = "SCR".to_string(),
            (0x0, 0x0, 0xf, 0xc) => disassembled_code = "SCL".to_string(),
            (0x0, 0x0, 0xf, 0xd) => disassembled_code = "EXIT".to_string(),
            (0x0, 0x0, 0xf, 0xe) => disassembled_code = "LOW".to_string(),
            (0x0, 0x0, 0xf, 0xf) => disassembled_code = "HIGH".to_string(),
            (0x1, _, _, _) => disassembled_code = format!("JP   {:03X}", nnn),
            (0x2, _, _, _) => disassembled_code = format!("CALL      {:04X}", nnn),
            (0x3, _, _, _) => disassembled_code = format!("SE    V{:X},  {:02X}", nibbles.1, nn),
            (0x4, _, _, _) => disassembled_code = format!("SNE   V{:X},  {:02X}", nibbles.1, nn),
            (0x5, _, _, 0x0) => {
                disassembled_code = format!("SE    V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x6, _, _, _) => {
                disassembled_code = format!("LD    V{:X},  {:02X}", nibbles.1, nibbles.2)
            }
            (0x7, _, _, _) => disassembled_code = format!("ADD   V{:X},  {:02X}", nibbles.1, nn),
            (0x8, _, _, 0x0) => {
                disassembled_code = format!("LD    V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x1) => {
                disassembled_code = format!("OR    V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x2) => {
                disassembled_code = format!("AND   V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x3) => {
                disassembled_code = format!("XOR   V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x4) => {
                disassembled_code = format!("ADD   V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x5) => {
                disassembled_code = format!("SUB   V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0x6) => disassembled_code = format!("SHR   V{:X}", nibbles.1),
            (0x8, _, _, 0x7) => {
                disassembled_code = format!("SUBN  V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0x8, _, _, 0xE) => disassembled_code = format!("SHL   V{:X}", nibbles.1),
            (0x9, _, _, 0x0) => {
                disassembled_code = format!("SNE   V{:X},  V{:X}", nibbles.1, nibbles.2)
            }
            (0xA, _, _, _) => disassembled_code = format!("LD     I, {:04X}", nnn),
            (0xB, _, _, _) => disassembled_code = format!("JP    V0, {:04X}", nnn),
            (0xC, _, _, _) => disassembled_code = format!("RND   V{:X},  {:02X}", nibbles.1, nn),
            (0xD, _, _, 0x0) => {
                disassembled_code = format!("DRW   V{:X}   V{:X}", nibbles.1, nibbles.2)
            }
            (0xD, _, _, _) => {
                disassembled_code =
                    format!("DRW   V{:X}   V{:X}  {:X}", nibbles.1, nibbles.2, nibbles.3)
            }
            (0xE, _, 0x9, 0xE) => disassembled_code = format!("SKP   V{:X}", nibbles.1),
            (0xE, _, 0xA, 0x1) => disassembled_code = format!("SKNP  V{:X}", nibbles.1),
            (0xF, _, 0x0, 0x7) => disassembled_code = format!("LD    V{:X},   DT", nibbles.1),
            (0xF, _, 0x0, 0xA) => disassembled_code = format!("LD    V{:X},   K", nibbles.1),
            (0xF, _, 0x1, 0x5) => disassembled_code = format!("LD    DT, V{:X}", nibbles.1),
            (0xF, _, 0x1, 0x8) => disassembled_code = format!("LD    ST, V{:X}", nibbles.1),
            (0xF, _, 0x1, 0xE) => disassembled_code = format!("ADD    I, V{:X}", nibbles.1),
            (0xF, _, 0x2, 0x9) => disassembled_code = format!("LD     F, V{:X}", nibbles.1),
            (0xF, _, 0x3, 0x0) => disassembled_code = format!("LD    SF, V{:X}", nibbles.1),
            (0xF, _, 0x3, 0x3) => disassembled_code = format!("BCD  [I], V{:X}", nibbles.1),
            (0xF, _, 0x5, 0x5) => disassembled_code = format!("LD   [I], V{:X}", nibbles.1),
            (0xF, _, 0x6, 0x5) => disassembled_code = format!("LD    V{:X}, [I]", nibbles.1),
            (0xF, _, 0x7, 0x5) => disassembled_code = format!("LD    V{:X}, R", nibbles.1),
            (0xF, _, 0x8, 0x5) => disassembled_code = format!("LD     R, V{:X}", nibbles.1),
            _ => disassembled_code = "Unknown".to_string(),
        }

        disassembled_code
    }
}
