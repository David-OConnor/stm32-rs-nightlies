#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch: [CH; 8],
    #[doc = "0x100..0x500 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_CR2, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    pub flt: [FLT; 4],
}
#[doc = "DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
pub mod ch;
#[doc = "Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_CR2, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
pub use self::flt::FLT;
#[doc = r"Cluster"]
#[doc = "Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_CR2, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
pub mod flt;
