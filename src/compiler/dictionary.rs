use std::collections::HashMap;

pub struct Dictionary<'a> {
    opcodes: HashMap<&'a str, u8>,
}

impl Dictionary<'_> {
    pub fn new() -> Dictionary<'static> {
        let mut opcodes = HashMap::<&str, u8>::new();

        opcodes.insert("HLT", 0xFF);
        opcodes.insert("LSP", 0x3F);
        opcodes.insert("JMP", 0x24);

        opcodes.insert("MIA", 0x00);
        opcodes.insert("MIB", 0x01);
        opcodes.insert("MIC", 0x02);
        opcodes.insert("MID", 0x03);
        opcodes.insert("MIE", 0x04);
        opcodes.insert("MIH", 0x05);
        opcodes.insert("MIL", 0x06);

        opcodes.insert("MMA", 0x78);
        opcodes.insert("MMB", 0x79);
        opcodes.insert("MMC", 0x7A);
        opcodes.insert("MMD", 0x7B);
        opcodes.insert("MME", 0x7C);
        opcodes.insert("MMH", 0x7D);
        opcodes.insert("MML", 0x7E);

        opcodes.insert("MAM", 0x47);
        opcodes.insert("MBM", 0x4F);
        opcodes.insert("MCM", 0x57);
        opcodes.insert("MDM", 0x5F);
        opcodes.insert("MEM", 0x67);
        opcodes.insert("MHM", 0x6F);
        opcodes.insert("MLM", 0x77);

        opcodes.insert("MAB", 0x41);
        opcodes.insert("MAC", 0x42);
        opcodes.insert("MAD", 0x43);
        opcodes.insert("MAE", 0x44);
        opcodes.insert("MAH", 0x45);
        opcodes.insert("MAL", 0x46);
        opcodes.insert("MBA", 0x48);
        opcodes.insert("MBC", 0x4A);
        opcodes.insert("MBD", 0x4B);
        opcodes.insert("MBE", 0x4C);
        opcodes.insert("MBH", 0x4D);
        opcodes.insert("MBL", 0x4E);
        opcodes.insert("MCA", 0x50);
        opcodes.insert("MCB", 0x51);
        opcodes.insert("MCD", 0x53);
        opcodes.insert("MCE", 0x54);
        opcodes.insert("MCH", 0x55);
        opcodes.insert("MCL", 0x56);
        opcodes.insert("MDA", 0x58);
        opcodes.insert("MDB", 0x59);
        opcodes.insert("MDC", 0x5A);
        opcodes.insert("MDE", 0x5C);
        opcodes.insert("MDH", 0x5D);
        opcodes.insert("MDL", 0x5E);
        opcodes.insert("MEA", 0x60);
        opcodes.insert("MEB", 0x61);
        opcodes.insert("MEC", 0x62);
        opcodes.insert("MED", 0x63);
        opcodes.insert("MEH", 0x65);
        opcodes.insert("MEL", 0x66);
        opcodes.insert("MHA", 0x68);
        opcodes.insert("MHB", 0x69);
        opcodes.insert("MHC", 0x6A);
        opcodes.insert("MHD", 0x6B);
        opcodes.insert("MHE", 0x6C);
        opcodes.insert("MHL", 0x6E);
        opcodes.insert("MLA", 0x70);
        opcodes.insert("MLB", 0x71);
        opcodes.insert("MLC", 0x72);
        opcodes.insert("MLD", 0x73);
        opcodes.insert("MLE", 0x74);
        opcodes.insert("MLH", 0x75);

        opcodes.insert("PUA", 0x07);
        opcodes.insert("PUB", 0x0F);
        opcodes.insert("PUC", 0x17);
        opcodes.insert("PUD", 0x1F);
        opcodes.insert("PUE", 0x27);
        opcodes.insert("PUH", 0x2F);
        opcodes.insert("PUL", 0x37);

        opcodes.insert("POA", 0x38);
        opcodes.insert("POB", 0x39);
        opcodes.insert("POC", 0x3A);
        opcodes.insert("POD", 0x3B);
        opcodes.insert("POE", 0x3C);
        opcodes.insert("POH", 0x3D);
        opcodes.insert("POL", 0x3E);

        opcodes.insert("JSZ", 0xCC);
        opcodes.insert("JSS", 0xDC);
        opcodes.insert("JSP", 0xEC);
        opcodes.insert("JSC", 0xFC);

        opcodes.insert("JRZ", 0xC4);
        opcodes.insert("JRS", 0xD4);
        opcodes.insert("JRP", 0xE4);
        opcodes.insert("JRC", 0xF4);

        opcodes.insert("ADA", 0x80);
        opcodes.insert("ADB", 0x88);
        opcodes.insert("ADC", 0x90);
        opcodes.insert("ADD", 0x98);
        opcodes.insert("ADE", 0xA0);
        opcodes.insert("ADH", 0xA8);
        opcodes.insert("ADL", 0xB0);

        opcodes.insert("ACA", 0x81);
        opcodes.insert("ACB", 0x89);
        opcodes.insert("ACC", 0x91);
        opcodes.insert("ACD", 0x99);
        opcodes.insert("ACE", 0xA1);
        opcodes.insert("ACH", 0xA9);
        opcodes.insert("ACL", 0xB1);

        opcodes.insert("SUA", 0x82);
        opcodes.insert("SUB", 0x8A);
        opcodes.insert("SUC", 0x92);
        opcodes.insert("SUD", 0x9A);
        opcodes.insert("SUE", 0xA2);
        opcodes.insert("SUH", 0xAA);
        opcodes.insert("SUL", 0xB2);

        opcodes.insert("SBA", 0x83);
        opcodes.insert("SBB", 0x8B);
        opcodes.insert("SBC", 0x93);
        opcodes.insert("SBD", 0x9B);
        opcodes.insert("SBE", 0xA3);
        opcodes.insert("SBH", 0xAB);
        opcodes.insert("SBL", 0xB3);

        opcodes.insert("CPA", 0x84);
        opcodes.insert("CPB", 0x8C);
        opcodes.insert("CPC", 0x94);
        opcodes.insert("CPD", 0x9C);
        opcodes.insert("CPE", 0xA4);
        opcodes.insert("CPH", 0xAC);
        opcodes.insert("CPL", 0xB4);

        opcodes.insert("ANA", 0xC0);
        opcodes.insert("ANB", 0xC8);
        opcodes.insert("ANC", 0xD0);
        opcodes.insert("AND", 0xD8);
        opcodes.insert("ANE", 0xE0);
        opcodes.insert("ANH", 0xE8);
        opcodes.insert("ANL", 0xF0);

        opcodes.insert("ORA", 0xC1);
        opcodes.insert("ORB", 0xC9);
        opcodes.insert("ORC", 0xD1);
        opcodes.insert("ORD", 0xD9);
        opcodes.insert("ORE", 0xE1);
        opcodes.insert("ORH", 0xE9);
        opcodes.insert("ORL", 0xF1);

        opcodes.insert("XRA", 0xC2);
        opcodes.insert("XRB", 0xCA);
        opcodes.insert("XRC", 0xD2);
        opcodes.insert("XRD", 0xDA);
        opcodes.insert("XRE", 0xE2);
        opcodes.insert("XRH", 0xEA);
        opcodes.insert("XRL", 0xF2);

        opcodes.insert("NTA", 0xC3);
        opcodes.insert("NTB", 0xCB);
        opcodes.insert("NTC", 0xD3);
        opcodes.insert("NTD", 0xDB);
        opcodes.insert("NTE", 0xE3);
        opcodes.insert("NTH", 0xEB);
        opcodes.insert("NTL", 0xF3);

        Dictionary { opcodes }
    }

    pub fn get_opcode(&self, string: &str) -> (Option<&u8>, bool) {
        let opcode = self.opcodes.get(string);
        let byte = if let Some(&op) = opcode {
            op < 7
        } else {
            false
        };
        (opcode, byte)
    }
}
