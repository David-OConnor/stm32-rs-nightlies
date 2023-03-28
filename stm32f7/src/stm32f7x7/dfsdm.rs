#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - DFSDM channel configuration cluster"]
    pub ch: [CH; 8],
    #[doc = "0x100..0x300 - DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers"]
    pub flt: [FLT; 4],
}
#[doc = "DFSDM channel configuration cluster"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DFSDM channel configuration cluster"]
pub mod ch;
#[doc = "DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers"]
pub use self::flt::FLT;
#[doc = r"Cluster"]
#[doc = "DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers"]
pub mod flt;
