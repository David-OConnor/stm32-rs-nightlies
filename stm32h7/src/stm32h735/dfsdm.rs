#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR"]
    pub ch: [CH; 8],
    #[doc = "0x100..0x300 - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt: [FLT; 4],
}
#[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR"]
pub mod ch;
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub use self::flt::FLT;
#[doc = r"Cluster"]
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub mod flt;
