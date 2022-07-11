use std::collections::HashMap;

pub struct Dictionary<'a> {
    opcodes: HashMap<u8, &'a str>,
}

impl Dictionary<'_> {
    pub fn new() -> Dictionary<'static> {
        let mut opcodes = HashMap::<u8, &str>::new();

        opcodes.insert(0xFF, "HLT");
        opcodes.insert(0x3F, "LSP");
        opcodes.insert(0x24, "JMP");

        opcodes.insert(0x00, "MIA");
        opcodes.insert(0x01, "MIB");
        opcodes.insert(0x02, "MIC");
        opcodes.insert(0x03, "MID");
        opcodes.insert(0x04, "MIE");
        opcodes.insert(0x05, "MIH");
        opcodes.insert(0x06, "MIL");

        opcodes.insert(0x78, "MMA");
        opcodes.insert(0x79, "MMB");
        opcodes.insert(0x7A, "MMC");
        opcodes.insert(0x7B, "MMD");
        opcodes.insert(0x7C, "MME");
        opcodes.insert(0x7D, "MMH");
        opcodes.insert(0x7E, "MML");

        opcodes.insert(0x47, "MAM");
        opcodes.insert(0x4F, "MBM");
        opcodes.insert(0x57, "MCM");
        opcodes.insert(0x5F, "MDM");
        opcodes.insert(0x67, "MEM");
        opcodes.insert(0x6F, "MHM");
        opcodes.insert(0x77, "MLM");

        opcodes.insert(0x41, "MAB");
        opcodes.insert(0x42, "MAC");
        opcodes.insert(0x43, "MAD");
        opcodes.insert(0x44, "MAE");
        opcodes.insert(0x45, "MAH");
        opcodes.insert(0x46, "MAL");
        opcodes.insert(0x48, "MBA");
        opcodes.insert(0x4A, "MBC");
        opcodes.insert(0x4B, "MBD");
        opcodes.insert(0x4C, "MBE");
        opcodes.insert(0x4D, "MBH");
        opcodes.insert(0x4E, "MBL");
        opcodes.insert(0x50, "MCA");
        opcodes.insert(0x51, "MCB");
        opcodes.insert(0x53, "MCD");
        opcodes.insert(0x54, "MCE");
        opcodes.insert(0x55, "MCH");
        opcodes.insert(0x56, "MCL");
        opcodes.insert(0x58, "MDA");
        opcodes.insert(0x59, "MDB");
        opcodes.insert(0x5A, "MDC");
        opcodes.insert(0x5C, "MDE");
        opcodes.insert(0x5D, "MDH");
        opcodes.insert(0x5E, "MDL");
        opcodes.insert(0x60, "MEA");
        opcodes.insert(0x61, "MEB");
        opcodes.insert(0x62, "MEC");
        opcodes.insert(0x63, "MED");
        opcodes.insert(0x65, "MEH");
        opcodes.insert(0x66, "MEL");
        opcodes.insert(0x68, "MHA");
        opcodes.insert(0x69, "MHB");
        opcodes.insert(0x6A, "MHC");
        opcodes.insert(0x6B, "MHD");
        opcodes.insert(0x6C, "MHE");
        opcodes.insert(0x6E, "MHL");
        opcodes.insert(0x70, "MLA");
        opcodes.insert(0x71, "MLB");
        opcodes.insert(0x72, "MLC");
        opcodes.insert(0x73, "MLD");
        opcodes.insert(0x74, "MLE");
        opcodes.insert(0x75, "MLH");

        opcodes.insert(0x07, "PUA");
        opcodes.insert(0x0F, "PUB");
        opcodes.insert(0x17, "PUC");
        opcodes.insert(0x1F, "PUD");
        opcodes.insert(0x27, "PUE");
        opcodes.insert(0x2F, "PUH");
        opcodes.insert(0x37, "PUL");

        opcodes.insert(0x38, "POA");
        opcodes.insert(0x39, "POB");
        opcodes.insert(0x3A, "POC");
        opcodes.insert(0x3B, "POD");
        opcodes.insert(0x3C, "POE");
        opcodes.insert(0x3D, "POH");
        opcodes.insert(0x3E, "POL");

        opcodes.insert(0xCC, "JSZ");
        opcodes.insert(0xDC, "JSS");
        opcodes.insert(0xEC, "JSP");
        opcodes.insert(0xFC, "JSC");

        opcodes.insert(0xC4, "JRZ");
        opcodes.insert(0xD4, "JRS");
        opcodes.insert(0xE4, "JRP");
        opcodes.insert(0xF4, "JRC");

        opcodes.insert(0x80, "ADA");
        opcodes.insert(0x88, "ADB");
        opcodes.insert(0x90, "ADC");
        opcodes.insert(0x98, "ADD");
        opcodes.insert(0xA0, "ADE");
        opcodes.insert(0xA8, "ADH");
        opcodes.insert(0xB0, "ADL");

        opcodes.insert(0x81, "ACA");
        opcodes.insert(0x89, "ACB");
        opcodes.insert(0x91, "ACC");
        opcodes.insert(0x99, "ACD");
        opcodes.insert(0xA1, "ACE");
        opcodes.insert(0xA9, "ACH");
        opcodes.insert(0xB1, "ACL");

        opcodes.insert(0x82, "SUA");
        opcodes.insert(0x8A, "SUB");
        opcodes.insert(0x92, "SUC");
        opcodes.insert(0x9A, "SUD");
        opcodes.insert(0xA2, "SUE");
        opcodes.insert(0xAA, "SUH");
        opcodes.insert(0xB2, "SUL");

        opcodes.insert(0x83, "SBA");
        opcodes.insert(0x8B, "SBB");
        opcodes.insert(0x93, "SBC");
        opcodes.insert(0x9B, "SBD");
        opcodes.insert(0xA3, "SBE");
        opcodes.insert(0xAB, "SBH");
        opcodes.insert(0xB3, "SBL");

        opcodes.insert(0x84, "CPA");
        opcodes.insert(0x8C, "CPB");
        opcodes.insert(0x94, "CPC");
        opcodes.insert(0x9C, "CPD");
        opcodes.insert(0xA4, "CPE");
        opcodes.insert(0xAC, "CPH");
        opcodes.insert(0xB4, "CPL");

        opcodes.insert(0xC0, "ANA");
        opcodes.insert(0xC8, "ANB");
        opcodes.insert(0xD0, "ANC");
        opcodes.insert(0xD8, "AND");
        opcodes.insert(0xE0, "ANE");
        opcodes.insert(0xE8, "ANH");
        opcodes.insert(0xF0, "ANL");

        opcodes.insert(0xC1, "ORA");
        opcodes.insert(0xC9, "ORB");
        opcodes.insert(0xD1, "ORC");
        opcodes.insert(0xD9, "ORD");
        opcodes.insert(0xE1, "ORE");
        opcodes.insert(0xE9, "ORH");
        opcodes.insert(0xF1, "ORL");

        opcodes.insert(0xC2, "XRA");
        opcodes.insert(0xCA, "XRB");
        opcodes.insert(0xD2, "XRC");
        opcodes.insert(0xDA, "XRD");
        opcodes.insert(0xE2, "XRE");
        opcodes.insert(0xEA, "XRH");
        opcodes.insert(0xF2, "XRL");

        opcodes.insert(0xC3, "NTA");
        opcodes.insert(0xCB, "NTB");
        opcodes.insert(0xD3, "NTC");
        opcodes.insert(0xDB, "NTD");
        opcodes.insert(0xE3, "NTE");
        opcodes.insert(0xEB, "NTH");
        opcodes.insert(0xF3, "NTL");

        Dictionary { opcodes }
    }

    pub fn get_instruction(&self, opcode: &u8) -> Option<&&str> {
        self.opcodes.get(opcode)
    }
}
