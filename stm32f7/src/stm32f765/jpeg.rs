#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec configuration register 0"]
    pub confr0: CONFR0,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub confr1: CONFR1,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub confr2: CONFR2,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub confr3: CONFR3,
    #[doc = "0x10 - JPEG codec configuration register 4"]
    pub confr4: CONFR4,
    #[doc = "0x14 - JPEG codec configuration register 5"]
    pub confr5: CONFR5,
    #[doc = "0x18 - JPEG codec configuration register 6"]
    pub confr6: CONFR6,
    #[doc = "0x1c - JPEG codec configuration register 7"]
    pub confr7: CONFR7,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - JPEG control register"]
    pub cr: CR,
    #[doc = "0x34 - JPEG status register"]
    pub sr: SR,
    #[doc = "0x38 - JPEG clear flag register"]
    pub cfr: CFR,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - JPEG data input register"]
    pub dir: DIR,
    #[doc = "0x44 - JPEG data output register"]
    pub dor: DOR,
    _reserved13: [u8; 0x08],
    #[doc = "0x50..0x90 - JPEG quantization tables"]
    pub qmem0: [QMEM0; 16],
    #[doc = "0x90..0xd0 - JPEG quantization tables"]
    pub qmem1: [QMEM1; 16],
    #[doc = "0xd0..0x110 - JPEG quantization tables"]
    pub qmem2: [QMEM2; 16],
    #[doc = "0x110..0x150 - JPEG quantization tables"]
    pub qmem3: [QMEM3; 16],
    #[doc = "0x150..0x190 - JPEG HuffMin tables"]
    pub huffmin: [HUFFMIN; 16],
    #[doc = "0x190..0x210 - JPEG HuffSymb tables"]
    pub huffbase: [HUFFBASE; 32],
    #[doc = "0x210..0x360 - JPEG HUFFSYMB tables"]
    pub huffsymb: [HUFFSYMB; 84],
    #[doc = "0x360..0x4fc - JPEG DHTMem tables"]
    pub dhtmem: [DHTMEM; 103],
    _reserved21: [u8; 0x04],
    #[doc = "0x500..0x660 - JPEG encoder, AC Huffman table %s"]
    pub huffenc_ac0: [HUFFENC_AC0; 88],
    #[doc = "0x660..0x7c0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1: [HUFFENC_AC1; 88],
    #[doc = "0x7c0..0x7e0 - JPEG encoder, DC Huffman table %s"]
    pub huffenc_dc0: [HUFFENC_DC0; 8],
    #[doc = "0x7e0..0x800 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1: [HUFFENC_DC1; 8],
}
impl RegisterBlock {
    #[doc = "0x360 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem0(&self) -> &DHTMEM {
        &self.dhtmem[0]
    }
    #[doc = "0x364 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem2(&self) -> &DHTMEM {
        &self.dhtmem[1]
    }
    #[doc = "0x368 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem3(&self) -> &DHTMEM {
        &self.dhtmem[2]
    }
    #[doc = "0x36c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem4(&self) -> &DHTMEM {
        &self.dhtmem[3]
    }
    #[doc = "0x370 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem5(&self) -> &DHTMEM {
        &self.dhtmem[4]
    }
    #[doc = "0x374 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem6(&self) -> &DHTMEM {
        &self.dhtmem[5]
    }
    #[doc = "0x378 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem7(&self) -> &DHTMEM {
        &self.dhtmem[6]
    }
    #[doc = "0x37c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem8(&self) -> &DHTMEM {
        &self.dhtmem[7]
    }
    #[doc = "0x380 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem9(&self) -> &DHTMEM {
        &self.dhtmem[8]
    }
    #[doc = "0x384 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem10(&self) -> &DHTMEM {
        &self.dhtmem[9]
    }
    #[doc = "0x388 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem11(&self) -> &DHTMEM {
        &self.dhtmem[10]
    }
    #[doc = "0x38c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem12(&self) -> &DHTMEM {
        &self.dhtmem[11]
    }
    #[doc = "0x390 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem13(&self) -> &DHTMEM {
        &self.dhtmem[12]
    }
    #[doc = "0x394 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem14(&self) -> &DHTMEM {
        &self.dhtmem[13]
    }
    #[doc = "0x398 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem15(&self) -> &DHTMEM {
        &self.dhtmem[14]
    }
    #[doc = "0x39c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem16(&self) -> &DHTMEM {
        &self.dhtmem[15]
    }
    #[doc = "0x3a0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem17(&self) -> &DHTMEM {
        &self.dhtmem[16]
    }
    #[doc = "0x3a4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem18(&self) -> &DHTMEM {
        &self.dhtmem[17]
    }
    #[doc = "0x3a8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem19(&self) -> &DHTMEM {
        &self.dhtmem[18]
    }
    #[doc = "0x3ac - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem20(&self) -> &DHTMEM {
        &self.dhtmem[19]
    }
    #[doc = "0x3b0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem21(&self) -> &DHTMEM {
        &self.dhtmem[20]
    }
    #[doc = "0x3b4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem22(&self) -> &DHTMEM {
        &self.dhtmem[21]
    }
    #[doc = "0x3b8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem23(&self) -> &DHTMEM {
        &self.dhtmem[22]
    }
    #[doc = "0x3bc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem24(&self) -> &DHTMEM {
        &self.dhtmem[23]
    }
    #[doc = "0x3c0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem25(&self) -> &DHTMEM {
        &self.dhtmem[24]
    }
    #[doc = "0x3c4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem26(&self) -> &DHTMEM {
        &self.dhtmem[25]
    }
    #[doc = "0x3c8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem27(&self) -> &DHTMEM {
        &self.dhtmem[26]
    }
    #[doc = "0x3cc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem28(&self) -> &DHTMEM {
        &self.dhtmem[27]
    }
    #[doc = "0x3d0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem29(&self) -> &DHTMEM {
        &self.dhtmem[28]
    }
    #[doc = "0x3d4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem30(&self) -> &DHTMEM {
        &self.dhtmem[29]
    }
    #[doc = "0x3d8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem31(&self) -> &DHTMEM {
        &self.dhtmem[30]
    }
    #[doc = "0x3dc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem32(&self) -> &DHTMEM {
        &self.dhtmem[31]
    }
    #[doc = "0x3e0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem33(&self) -> &DHTMEM {
        &self.dhtmem[32]
    }
    #[doc = "0x3e4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem34(&self) -> &DHTMEM {
        &self.dhtmem[33]
    }
    #[doc = "0x3e8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem35(&self) -> &DHTMEM {
        &self.dhtmem[34]
    }
    #[doc = "0x3ec - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem36(&self) -> &DHTMEM {
        &self.dhtmem[35]
    }
    #[doc = "0x3f0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem37(&self) -> &DHTMEM {
        &self.dhtmem[36]
    }
    #[doc = "0x3f4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem38(&self) -> &DHTMEM {
        &self.dhtmem[37]
    }
    #[doc = "0x3f8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem39(&self) -> &DHTMEM {
        &self.dhtmem[38]
    }
    #[doc = "0x3fc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem40(&self) -> &DHTMEM {
        &self.dhtmem[39]
    }
    #[doc = "0x400 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem41(&self) -> &DHTMEM {
        &self.dhtmem[40]
    }
    #[doc = "0x404 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem42(&self) -> &DHTMEM {
        &self.dhtmem[41]
    }
    #[doc = "0x408 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem43(&self) -> &DHTMEM {
        &self.dhtmem[42]
    }
    #[doc = "0x40c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem44(&self) -> &DHTMEM {
        &self.dhtmem[43]
    }
    #[doc = "0x410 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem45(&self) -> &DHTMEM {
        &self.dhtmem[44]
    }
    #[doc = "0x414 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem46(&self) -> &DHTMEM {
        &self.dhtmem[45]
    }
    #[doc = "0x418 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem47(&self) -> &DHTMEM {
        &self.dhtmem[46]
    }
    #[doc = "0x41c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem48(&self) -> &DHTMEM {
        &self.dhtmem[47]
    }
    #[doc = "0x420 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem49(&self) -> &DHTMEM {
        &self.dhtmem[48]
    }
    #[doc = "0x424 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem50(&self) -> &DHTMEM {
        &self.dhtmem[49]
    }
    #[doc = "0x428 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem51(&self) -> &DHTMEM {
        &self.dhtmem[50]
    }
    #[doc = "0x42c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem52(&self) -> &DHTMEM {
        &self.dhtmem[51]
    }
    #[doc = "0x430 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem53(&self) -> &DHTMEM {
        &self.dhtmem[52]
    }
    #[doc = "0x434 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem54(&self) -> &DHTMEM {
        &self.dhtmem[53]
    }
    #[doc = "0x438 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem55(&self) -> &DHTMEM {
        &self.dhtmem[54]
    }
    #[doc = "0x43c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem56(&self) -> &DHTMEM {
        &self.dhtmem[55]
    }
    #[doc = "0x440 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem57(&self) -> &DHTMEM {
        &self.dhtmem[56]
    }
    #[doc = "0x444 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem58(&self) -> &DHTMEM {
        &self.dhtmem[57]
    }
    #[doc = "0x448 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem59(&self) -> &DHTMEM {
        &self.dhtmem[58]
    }
    #[doc = "0x44c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem60(&self) -> &DHTMEM {
        &self.dhtmem[59]
    }
    #[doc = "0x450 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem61(&self) -> &DHTMEM {
        &self.dhtmem[60]
    }
    #[doc = "0x454 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem62(&self) -> &DHTMEM {
        &self.dhtmem[61]
    }
    #[doc = "0x458 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem63(&self) -> &DHTMEM {
        &self.dhtmem[62]
    }
    #[doc = "0x45c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem64(&self) -> &DHTMEM {
        &self.dhtmem[63]
    }
    #[doc = "0x460 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem65(&self) -> &DHTMEM {
        &self.dhtmem[64]
    }
    #[doc = "0x464 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem66(&self) -> &DHTMEM {
        &self.dhtmem[65]
    }
    #[doc = "0x468 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem67(&self) -> &DHTMEM {
        &self.dhtmem[66]
    }
    #[doc = "0x46c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem68(&self) -> &DHTMEM {
        &self.dhtmem[67]
    }
    #[doc = "0x470 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem69(&self) -> &DHTMEM {
        &self.dhtmem[68]
    }
    #[doc = "0x474 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem70(&self) -> &DHTMEM {
        &self.dhtmem[69]
    }
    #[doc = "0x478 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem71(&self) -> &DHTMEM {
        &self.dhtmem[70]
    }
    #[doc = "0x47c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem72(&self) -> &DHTMEM {
        &self.dhtmem[71]
    }
    #[doc = "0x480 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem73(&self) -> &DHTMEM {
        &self.dhtmem[72]
    }
    #[doc = "0x484 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem74(&self) -> &DHTMEM {
        &self.dhtmem[73]
    }
    #[doc = "0x488 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem75(&self) -> &DHTMEM {
        &self.dhtmem[74]
    }
    #[doc = "0x48c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem76(&self) -> &DHTMEM {
        &self.dhtmem[75]
    }
    #[doc = "0x490 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem77(&self) -> &DHTMEM {
        &self.dhtmem[76]
    }
    #[doc = "0x494 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem78(&self) -> &DHTMEM {
        &self.dhtmem[77]
    }
    #[doc = "0x498 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem79(&self) -> &DHTMEM {
        &self.dhtmem[78]
    }
    #[doc = "0x49c - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem80(&self) -> &DHTMEM {
        &self.dhtmem[79]
    }
    #[doc = "0x4a0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem81(&self) -> &DHTMEM {
        &self.dhtmem[80]
    }
    #[doc = "0x4a4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem82(&self) -> &DHTMEM {
        &self.dhtmem[81]
    }
    #[doc = "0x4a8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem83(&self) -> &DHTMEM {
        &self.dhtmem[82]
    }
    #[doc = "0x4ac - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem84(&self) -> &DHTMEM {
        &self.dhtmem[83]
    }
    #[doc = "0x4b0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem85(&self) -> &DHTMEM {
        &self.dhtmem[84]
    }
    #[doc = "0x4b4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem86(&self) -> &DHTMEM {
        &self.dhtmem[85]
    }
    #[doc = "0x4b8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem87(&self) -> &DHTMEM {
        &self.dhtmem[86]
    }
    #[doc = "0x4bc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem88(&self) -> &DHTMEM {
        &self.dhtmem[87]
    }
    #[doc = "0x4c0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem89(&self) -> &DHTMEM {
        &self.dhtmem[88]
    }
    #[doc = "0x4c4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem90(&self) -> &DHTMEM {
        &self.dhtmem[89]
    }
    #[doc = "0x4c8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem91(&self) -> &DHTMEM {
        &self.dhtmem[90]
    }
    #[doc = "0x4cc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem92(&self) -> &DHTMEM {
        &self.dhtmem[91]
    }
    #[doc = "0x4d0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem93(&self) -> &DHTMEM {
        &self.dhtmem[92]
    }
    #[doc = "0x4d4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem94(&self) -> &DHTMEM {
        &self.dhtmem[93]
    }
    #[doc = "0x4d8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem95(&self) -> &DHTMEM {
        &self.dhtmem[94]
    }
    #[doc = "0x4dc - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem96(&self) -> &DHTMEM {
        &self.dhtmem[95]
    }
    #[doc = "0x4e0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem97(&self) -> &DHTMEM {
        &self.dhtmem[96]
    }
    #[doc = "0x4e4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem98(&self) -> &DHTMEM {
        &self.dhtmem[97]
    }
    #[doc = "0x4e8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem99(&self) -> &DHTMEM {
        &self.dhtmem[98]
    }
    #[doc = "0x4ec - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem100(&self) -> &DHTMEM {
        &self.dhtmem[99]
    }
    #[doc = "0x4f0 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem101(&self) -> &DHTMEM {
        &self.dhtmem[100]
    }
    #[doc = "0x4f4 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem102(&self) -> &DHTMEM {
        &self.dhtmem[101]
    }
    #[doc = "0x4f8 - JPEG DHTMem tables"]
    #[inline(always)]
    pub fn dhtmem103(&self) -> &DHTMEM {
        &self.dhtmem[102]
    }
}
#[doc = "CONFR0 (w) register accessor: an alias for `Reg<CONFR0_SPEC>`"]
pub type CONFR0 = crate::Reg<confr0::CONFR0_SPEC>;
#[doc = "JPEG codec configuration register 0"]
pub mod confr0;
#[doc = "CONFR1 (rw) register accessor: an alias for `Reg<CONFR1_SPEC>`"]
pub type CONFR1 = crate::Reg<confr1::CONFR1_SPEC>;
#[doc = "JPEG codec configuration register 1"]
pub mod confr1;
#[doc = "CONFR2 (rw) register accessor: an alias for `Reg<CONFR2_SPEC>`"]
pub type CONFR2 = crate::Reg<confr2::CONFR2_SPEC>;
#[doc = "JPEG codec configuration register 2"]
pub mod confr2;
#[doc = "CONFR3 (rw) register accessor: an alias for `Reg<CONFR3_SPEC>`"]
pub type CONFR3 = crate::Reg<confr3::CONFR3_SPEC>;
#[doc = "JPEG codec configuration register 3"]
pub mod confr3;
#[doc = "CONFR4 (rw) register accessor: an alias for `Reg<CONFR4_SPEC>`"]
pub type CONFR4 = crate::Reg<confr4::CONFR4_SPEC>;
#[doc = "JPEG codec configuration register 4"]
pub mod confr4;
#[doc = "CONFR5 (rw) register accessor: an alias for `Reg<CONFR5_SPEC>`"]
pub type CONFR5 = crate::Reg<confr5::CONFR5_SPEC>;
#[doc = "JPEG codec configuration register 5"]
pub mod confr5;
#[doc = "CONFR6 (rw) register accessor: an alias for `Reg<CONFR6_SPEC>`"]
pub type CONFR6 = crate::Reg<confr6::CONFR6_SPEC>;
#[doc = "JPEG codec configuration register 6"]
pub mod confr6;
#[doc = "CONFR7 (rw) register accessor: an alias for `Reg<CONFR7_SPEC>`"]
pub type CONFR7 = crate::Reg<confr7::CONFR7_SPEC>;
#[doc = "JPEG codec configuration register 7"]
pub mod confr7;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "JPEG control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "JPEG status register"]
pub mod sr;
#[doc = "CFR (w) register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "JPEG clear flag register"]
pub mod cfr;
#[doc = "DIR (w) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "JPEG data input register"]
pub mod dir;
#[doc = "DOR (r) register accessor: an alias for `Reg<DOR_SPEC>`"]
pub type DOR = crate::Reg<dor::DOR_SPEC>;
#[doc = "JPEG data output register"]
pub mod dor;
#[doc = "QMEM0 (rw) register accessor: an alias for `Reg<QMEM0_SPEC>`"]
pub type QMEM0 = crate::Reg<qmem0::QMEM0_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem0;
#[doc = "QMEM1 (rw) register accessor: an alias for `Reg<QMEM1_SPEC>`"]
pub type QMEM1 = crate::Reg<qmem1::QMEM1_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem1;
#[doc = "QMEM2 (rw) register accessor: an alias for `Reg<QMEM2_SPEC>`"]
pub type QMEM2 = crate::Reg<qmem2::QMEM2_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem2;
#[doc = "QMEM3 (rw) register accessor: an alias for `Reg<QMEM3_SPEC>`"]
pub type QMEM3 = crate::Reg<qmem3::QMEM3_SPEC>;
#[doc = "JPEG quantization tables"]
pub mod qmem3;
#[doc = "HUFFMIN (rw) register accessor: an alias for `Reg<HUFFMIN_SPEC>`"]
pub type HUFFMIN = crate::Reg<huffmin::HUFFMIN_SPEC>;
#[doc = "JPEG HuffMin tables"]
pub mod huffmin;
#[doc = "HUFFBASE (rw) register accessor: an alias for `Reg<HUFFBASE_SPEC>`"]
pub type HUFFBASE = crate::Reg<huffbase::HUFFBASE_SPEC>;
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase;
#[doc = "HUFFSYMB (rw) register accessor: an alias for `Reg<HUFFSYMB_SPEC>`"]
pub type HUFFSYMB = crate::Reg<huffsymb::HUFFSYMB_SPEC>;
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb;
#[doc = "DHTMEM (rw) register accessor: an alias for `Reg<DHTMEM_SPEC>`"]
pub type DHTMEM = crate::Reg<dhtmem::DHTMEM_SPEC>;
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem;
#[doc = "HUFFENC_AC0 (rw) register accessor: an alias for `Reg<HUFFENC_AC0_SPEC>`"]
pub type HUFFENC_AC0 = crate::Reg<huffenc_ac0::HUFFENC_AC0_SPEC>;
#[doc = "JPEG encoder, AC Huffman table %s"]
pub mod huffenc_ac0;
#[doc = "HUFFENC_AC1 (rw) register accessor: an alias for `Reg<HUFFENC_AC1_SPEC>`"]
pub type HUFFENC_AC1 = crate::Reg<huffenc_ac1::HUFFENC_AC1_SPEC>;
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1;
#[doc = "HUFFENC_DC0 (rw) register accessor: an alias for `Reg<HUFFENC_DC0_SPEC>`"]
pub type HUFFENC_DC0 = crate::Reg<huffenc_dc0::HUFFENC_DC0_SPEC>;
#[doc = "JPEG encoder, DC Huffman table %s"]
pub mod huffenc_dc0;
#[doc = "HUFFENC_DC1 (rw) register accessor: an alias for `Reg<HUFFENC_DC1_SPEC>`"]
pub type HUFFENC_DC1 = crate::Reg<huffenc_dc1::HUFFENC_DC1_SPEC>;
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1;
