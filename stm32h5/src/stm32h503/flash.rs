#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    pub acr: ACR,
    #[doc = "0x04 - FLASH key register"]
    pub nskeyr: NSKEYR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FLASH option key register"]
    pub optkeyr: OPTKEYR,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - FLASH operation status register"]
    pub opsr: OPSR,
    #[doc = "0x1c - FLASH option control register"]
    pub optcr: OPTCR,
    #[doc = "0x20 - FLASH non-secure status register"]
    pub nssr: NSSR,
    #[doc = "0x24 - FLASH secure status register"]
    pub secsr: SECSR,
    #[doc = "0x28 - FLASH Non Secure control register"]
    pub nscr: NSCR,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - FLASH non-secure clear control register"]
    pub nsccr: NSCCR,
    _reserved9: [u8; 0x08],
    #[doc = "0x3c - FLASH privilege configuration register"]
    pub privcfgr: PRIVCFGR,
    _reserved10: [u8; 0x08],
    #[doc = "0x48 - FLASH HDP extension register"]
    pub hdpextr: HDPEXTR,
    _reserved11: [u8; 0x04],
    #[doc = "0x50 - FLASH option status register"]
    pub optsr_cur: OPTSR_CUR,
    #[doc = "0x54 - FLASH option status register"]
    pub optsr_prg: OPTSR_PRG,
    _reserved13: [u8; 0x18],
    #[doc = "0x70 - FLASH option status register 2"]
    pub optsr2_cur: OPTSR2_CUR,
    #[doc = "0x74 - FLASH option status register 2"]
    pub optsr2_prg: OPTSR2_PRG,
    _reserved15: [u8; 0x08],
    #[doc = "0x80 - FLASH non-secure unique boot entry register"]
    pub nsbootr_cur: NSBOOTR_CUR,
    #[doc = "0x84 - FLASH non-secure unique boot entry address"]
    pub nsbootr_prg: NSBOOTR_PRG,
    _reserved17: [u8; 0x08],
    #[doc = "0x90 - FLASH non-secure OTP block lock"]
    pub otpblr_cur: OTPBLR_CUR,
    #[doc = "0x94 - FLASH non-secure OTP block lock"]
    pub otpblr_prg: OTPBLR_PRG,
    _reserved19: [u8; 0x28],
    #[doc = "0xc0 - FLASH privilege register for bank 1"]
    pub privbb1r: PRIVBB1R,
    _reserved20: [u8; 0x24],
    #[doc = "0xe8 - FLASH write sector protection for Bank1"]
    pub wrpsgn1r_cur: WRPSGN1R_CUR,
    #[doc = "0xec - FLASH write sector protection for Bank1"]
    pub wrpsgn1r_prg: WRPSGN1R_PRG,
    _reserved22: [u8; 0x08],
    #[doc = "0xf8 - FLASH HDP Bank1 register"]
    pub hdp1r_cur: HDP1R_CUR,
    #[doc = "0xfc - FLASH HDP Bank1 register"]
    pub hdp1r_prg: HDP1R_PRG,
    #[doc = "0x100 - FLASH Flash ECC correction register"]
    pub ecccorr: ECCCORR,
    #[doc = "0x104 - FLASH ECC detection register"]
    pub eccdetr: ECCDETR,
    #[doc = "0x108 - FLASH ECC data"]
    pub eccdr: ECCDR,
    _reserved27: [u8; 0xdc],
    #[doc = "0x1e8 - FLASH write sector protection for Bank2"]
    pub wrpsgn2r_cur: WRPSGN2R_CUR,
    #[doc = "0x1ec - FLASH write sector protection for Bank2"]
    pub wrpsgn2r_prg: WRPSGN2R_PRG,
    _reserved29: [u8; 0x08],
    #[doc = "0x1f8 - FLASH HDP Bank2 register"]
    pub hdp2r_cur: HDP2R_CUR,
    #[doc = "0x1fc - FLASH HDP Bank2 register"]
    pub hdp2r_prg: HDP2R_PRG,
}
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "NSKEYR (w) register accessor: an alias for `Reg<NSKEYR_SPEC>`"]
pub type NSKEYR = crate::Reg<nskeyr::NSKEYR_SPEC>;
#[doc = "FLASH key register"]
pub mod nskeyr;
#[doc = "OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPSR (r) register accessor: an alias for `Reg<OPSR_SPEC>`"]
pub type OPSR = crate::Reg<opsr::OPSR_SPEC>;
#[doc = "FLASH operation status register"]
pub mod opsr;
#[doc = "OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "NSSR (r) register accessor: an alias for `Reg<NSSR_SPEC>`"]
pub type NSSR = crate::Reg<nssr::NSSR_SPEC>;
#[doc = "FLASH non-secure status register"]
pub mod nssr;
#[doc = "SECSR (r) register accessor: an alias for `Reg<SECSR_SPEC>`"]
pub type SECSR = crate::Reg<secsr::SECSR_SPEC>;
#[doc = "FLASH secure status register"]
pub mod secsr;
#[doc = "NSCR (rw) register accessor: an alias for `Reg<NSCR_SPEC>`"]
pub type NSCR = crate::Reg<nscr::NSCR_SPEC>;
#[doc = "FLASH Non Secure control register"]
pub mod nscr;
#[doc = "NSCCR (w) register accessor: an alias for `Reg<NSCCR_SPEC>`"]
pub type NSCCR = crate::Reg<nsccr::NSCCR_SPEC>;
#[doc = "FLASH non-secure clear control register"]
pub mod nsccr;
#[doc = "PRIVCFGR (w) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "FLASH privilege configuration register"]
pub mod privcfgr;
#[doc = "HDPEXTR (rw) register accessor: an alias for `Reg<HDPEXTR_SPEC>`"]
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTR_SPEC>;
#[doc = "FLASH HDP extension register"]
pub mod hdpextr;
#[doc = "OPTSR_CUR (r) register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR2_CUR (r) register accessor: an alias for `Reg<OPTSR2_CUR_SPEC>`"]
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CUR_SPEC>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_cur;
#[doc = "OPTSR2_PRG (rw) register accessor: an alias for `Reg<OPTSR2_PRG_SPEC>`"]
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRG_SPEC>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_prg;
#[doc = "NSBOOTR_CUR (r) register accessor: an alias for `Reg<NSBOOTR_CUR_SPEC>`"]
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CUR_SPEC>;
#[doc = "FLASH non-secure unique boot entry register"]
pub mod nsbootr_cur;
#[doc = "NSBOOTR_PRG (rw) register accessor: an alias for `Reg<NSBOOTR_PRG_SPEC>`"]
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRG_SPEC>;
#[doc = "FLASH non-secure unique boot entry address"]
pub mod nsbootr_prg;
#[doc = "OTPBLR_CUR (r) register accessor: an alias for `Reg<OTPBLR_CUR_SPEC>`"]
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CUR_SPEC>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_cur;
#[doc = "OTPBLR_PRG (rw) register accessor: an alias for `Reg<OTPBLR_PRG_SPEC>`"]
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRG_SPEC>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_prg;
#[doc = "PRIVBB1R (rw) register accessor: an alias for `Reg<PRIVBB1R_SPEC>`"]
pub type PRIVBB1R = crate::Reg<privbb1r::PRIVBB1R_SPEC>;
#[doc = "FLASH privilege register for bank 1"]
pub mod privbb1r;
#[doc = "WRPSGN1R_CUR (r) register accessor: an alias for `Reg<WRPSGN1R_CUR_SPEC>`"]
pub type WRPSGN1R_CUR = crate::Reg<wrpsgn1r_cur::WRPSGN1R_CUR_SPEC>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_cur;
#[doc = "WRPSGN1R_PRG (rw) register accessor: an alias for `Reg<WRPSGN1R_PRG_SPEC>`"]
pub type WRPSGN1R_PRG = crate::Reg<wrpsgn1r_prg::WRPSGN1R_PRG_SPEC>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_prg;
#[doc = "HDP1R_CUR (r) register accessor: an alias for `Reg<HDP1R_CUR_SPEC>`"]
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CUR_SPEC>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_cur;
#[doc = "HDP1R_PRG (r) register accessor: an alias for `Reg<HDP1R_PRG_SPEC>`"]
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRG_SPEC>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_prg;
#[doc = "ECCCORR (rw) register accessor: an alias for `Reg<ECCCORR_SPEC>`"]
pub type ECCCORR = crate::Reg<ecccorr::ECCCORR_SPEC>;
#[doc = "FLASH Flash ECC correction register"]
pub mod ecccorr;
#[doc = "ECCDETR (rw) register accessor: an alias for `Reg<ECCDETR_SPEC>`"]
pub type ECCDETR = crate::Reg<eccdetr::ECCDETR_SPEC>;
#[doc = "FLASH ECC detection register"]
pub mod eccdetr;
#[doc = "ECCDR (r) register accessor: an alias for `Reg<ECCDR_SPEC>`"]
pub type ECCDR = crate::Reg<eccdr::ECCDR_SPEC>;
#[doc = "FLASH ECC data"]
pub mod eccdr;
#[doc = "WRPSGN2R_CUR (r) register accessor: an alias for `Reg<WRPSGN2R_CUR_SPEC>`"]
pub type WRPSGN2R_CUR = crate::Reg<wrpsgn2r_cur::WRPSGN2R_CUR_SPEC>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_cur;
#[doc = "WRPSGN2R_PRG (rw) register accessor: an alias for `Reg<WRPSGN2R_PRG_SPEC>`"]
pub type WRPSGN2R_PRG = crate::Reg<wrpsgn2r_prg::WRPSGN2R_PRG_SPEC>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_prg;
#[doc = "HDP2R_CUR (r) register accessor: an alias for `Reg<HDP2R_CUR_SPEC>`"]
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CUR_SPEC>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_cur;
#[doc = "HDP2R_PRG (rw) register accessor: an alias for `Reg<HDP2R_PRG_SPEC>`"]
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRG_SPEC>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_prg;
