#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x28 - Backup data register (BKP_DR)"]
    pub dr: [DR; 10],
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: RTCCR,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: CR,
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    pub csr: CSR,
    _reserved4: [u8; 0x08],
    #[doc = "0x3c..0xbc - Backup data register (BKP_DR)"]
    pub bkp_dr: [BKP_DR; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr1(&self) -> &DR {
        &self.dr[0]
    }
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr2(&self) -> &DR {
        &self.dr[1]
    }
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr3(&self) -> &DR {
        &self.dr[2]
    }
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr4(&self) -> &DR {
        &self.dr[3]
    }
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr5(&self) -> &DR {
        &self.dr[4]
    }
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr6(&self) -> &DR {
        &self.dr[5]
    }
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr7(&self) -> &DR {
        &self.dr[6]
    }
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr8(&self) -> &DR {
        &self.dr[7]
    }
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr9(&self) -> &DR {
        &self.dr[8]
    }
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn dr10(&self) -> &DR {
        &self.dr[9]
    }
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr11(&self) -> &BKP_DR {
        &self.bkp_dr[0]
    }
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr12(&self) -> &BKP_DR {
        &self.bkp_dr[1]
    }
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr13(&self) -> &BKP_DR {
        &self.bkp_dr[2]
    }
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr14(&self) -> &BKP_DR {
        &self.bkp_dr[3]
    }
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr15(&self) -> &BKP_DR {
        &self.bkp_dr[4]
    }
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr16(&self) -> &BKP_DR {
        &self.bkp_dr[5]
    }
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr17(&self) -> &BKP_DR {
        &self.bkp_dr[6]
    }
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr18(&self) -> &BKP_DR {
        &self.bkp_dr[7]
    }
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr19(&self) -> &BKP_DR {
        &self.bkp_dr[8]
    }
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr20(&self) -> &BKP_DR {
        &self.bkp_dr[9]
    }
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr21(&self) -> &BKP_DR {
        &self.bkp_dr[10]
    }
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr22(&self) -> &BKP_DR {
        &self.bkp_dr[11]
    }
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr23(&self) -> &BKP_DR {
        &self.bkp_dr[12]
    }
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr24(&self) -> &BKP_DR {
        &self.bkp_dr[13]
    }
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr25(&self) -> &BKP_DR {
        &self.bkp_dr[14]
    }
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr26(&self) -> &BKP_DR {
        &self.bkp_dr[15]
    }
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr27(&self) -> &BKP_DR {
        &self.bkp_dr[16]
    }
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr28(&self) -> &BKP_DR {
        &self.bkp_dr[17]
    }
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr29(&self) -> &BKP_DR {
        &self.bkp_dr[18]
    }
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr30(&self) -> &BKP_DR {
        &self.bkp_dr[19]
    }
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr31(&self) -> &BKP_DR {
        &self.bkp_dr[20]
    }
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr32(&self) -> &BKP_DR {
        &self.bkp_dr[21]
    }
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr33(&self) -> &BKP_DR {
        &self.bkp_dr[22]
    }
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr34(&self) -> &BKP_DR {
        &self.bkp_dr[23]
    }
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr35(&self) -> &BKP_DR {
        &self.bkp_dr[24]
    }
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr36(&self) -> &BKP_DR {
        &self.bkp_dr[25]
    }
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr37(&self) -> &BKP_DR {
        &self.bkp_dr[26]
    }
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr38(&self) -> &BKP_DR {
        &self.bkp_dr[27]
    }
    #[doc = "0xac - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr39(&self) -> &BKP_DR {
        &self.bkp_dr[28]
    }
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr40(&self) -> &BKP_DR {
        &self.bkp_dr[29]
    }
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr41(&self) -> &BKP_DR {
        &self.bkp_dr[30]
    }
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub fn bkp_dr42(&self) -> &BKP_DR {
        &self.bkp_dr[31]
    }
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr;
#[doc = "BKP_DR (rw) register accessor: an alias for `Reg<BKP_DR_SPEC>`"]
pub type BKP_DR = crate::Reg<bkp_dr::BKP_DR_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod bkp_dr;
#[doc = "RTCCR (rw) register accessor: an alias for `Reg<RTCCR_SPEC>`"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
