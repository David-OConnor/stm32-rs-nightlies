#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA low interrupt status register"]
    pub dma_lisr: DMA_LISR,
    #[doc = "0x04 - DMA high interrupt status register"]
    pub dma_hisr: DMA_HISR,
    #[doc = "0x08 - DMA low interrupt flag clear register"]
    pub dma_lifcr: DMA_LIFCR,
    #[doc = "0x0c - DMA high interrupt flag clear register"]
    pub dma_hifcr: DMA_HIFCR,
    #[doc = "0x10 - This register is used to configure the concerned stream."]
    pub dma_s0cr: DMA_S0CR,
    #[doc = "0x14 - DMA stream 0 number of data register"]
    pub dma_s0ndtr: DMA_S0NDTR,
    #[doc = "0x18 - DMA stream 0 peripheral address register"]
    pub dma_s0par: DMA_S0PAR,
    #[doc = "0x1c - DMA stream 0 memory 0 address register"]
    pub dma_s0m0ar: DMA_S0M0AR,
    #[doc = "0x20 - DMA stream 0 memory 1 address register"]
    pub dma_s0m1ar: DMA_S0M1AR,
    #[doc = "0x24 - DMA stream 0 FIFO control register"]
    pub dma_s0fcr: DMA_S0FCR,
    #[doc = "0x28 - This register is used to configure the concerned stream."]
    pub dma_s1cr: DMA_S1CR,
    #[doc = "0x2c - DMA stream 1 number of data register"]
    pub dma_s1ndtr: DMA_S1NDTR,
    #[doc = "0x30 - DMA stream 1 peripheral address register"]
    pub dma_s1par: DMA_S1PAR,
    #[doc = "0x34 - DMA stream 1 memory 0 address register"]
    pub dma_s1m0ar: DMA_S1M0AR,
    #[doc = "0x38 - DMA stream 1 memory 1 address register"]
    pub dma_s1m1ar: DMA_S1M1AR,
    #[doc = "0x3c - DMA stream 1 FIFO control register"]
    pub dma_s1fcr: DMA_S1FCR,
    #[doc = "0x40 - This register is used to configure the concerned stream."]
    pub dma_s2cr: DMA_S2CR,
    #[doc = "0x44 - DMA stream 2 number of data register"]
    pub dma_s2ndtr: DMA_S2NDTR,
    #[doc = "0x48 - DMA stream 2 peripheral address register"]
    pub dma_s2par: DMA_S2PAR,
    #[doc = "0x4c - DMA stream 2 memory 0 address register"]
    pub dma_s2m0ar: DMA_S2M0AR,
    #[doc = "0x50 - DMA stream 2 memory 1 address register"]
    pub dma_s2m1ar: DMA_S2M1AR,
    #[doc = "0x54 - DMA stream 2 FIFO control register"]
    pub dma_s2fcr: DMA_S2FCR,
    #[doc = "0x58 - This register is used to configure the concerned stream."]
    pub dma_s3cr: DMA_S3CR,
    #[doc = "0x5c - DMA stream 3 number of data register"]
    pub dma_s3ndtr: DMA_S3NDTR,
    #[doc = "0x60 - DMA stream 3 peripheral address register"]
    pub dma_s3par: DMA_S3PAR,
    #[doc = "0x64 - DMA stream 3 memory 0 address register"]
    pub dma_s3m0ar: DMA_S3M0AR,
    #[doc = "0x68 - DMA stream 3 memory 1 address register"]
    pub dma_s3m1ar: DMA_S3M1AR,
    #[doc = "0x6c - DMA stream 3 FIFO control register"]
    pub dma_s3fcr: DMA_S3FCR,
    #[doc = "0x70 - This register is used to configure the concerned stream."]
    pub dma_s4cr: DMA_S4CR,
    #[doc = "0x74 - DMA stream 4 number of data register"]
    pub dma_s4ndtr: DMA_S4NDTR,
    #[doc = "0x78 - DMA stream 4 peripheral address register"]
    pub dma_s4par: DMA_S4PAR,
    #[doc = "0x7c - DMA stream 4 memory 0 address register"]
    pub dma_s4m0ar: DMA_S4M0AR,
    #[doc = "0x80 - DMA stream 4 memory 1 address register"]
    pub dma_s4m1ar: DMA_S4M1AR,
    #[doc = "0x84 - DMA stream 4 FIFO control register"]
    pub dma_s4fcr: DMA_S4FCR,
    #[doc = "0x88 - This register is used to configure the concerned stream."]
    pub dma_s5cr: DMA_S5CR,
    #[doc = "0x8c - DMA stream 5 number of data register"]
    pub dma_s5ndtr: DMA_S5NDTR,
    #[doc = "0x90 - DMA stream 5 peripheral address register"]
    pub dma_s5par: DMA_S5PAR,
    #[doc = "0x94 - DMA stream 5 memory 0 address register"]
    pub dma_s5m0ar: DMA_S5M0AR,
    #[doc = "0x98 - DMA stream 5 memory 1 address register"]
    pub dma_s5m1ar: DMA_S5M1AR,
    #[doc = "0x9c - DMA stream 5 FIFO control register"]
    pub dma_s5fcr: DMA_S5FCR,
    #[doc = "0xa0 - This register is used to configure the concerned stream."]
    pub dma_s6cr: DMA_S6CR,
    #[doc = "0xa4 - DMA stream 6 number of data register"]
    pub dma_s6ndtr: DMA_S6NDTR,
    #[doc = "0xa8 - DMA stream 6 peripheral address register"]
    pub dma_s6par: DMA_S6PAR,
    #[doc = "0xac - DMA stream 6 memory 0 address register"]
    pub dma_s6m0ar: DMA_S6M0AR,
    #[doc = "0xb0 - DMA stream 6 memory 1 address register"]
    pub dma_s6m1ar: DMA_S6M1AR,
    #[doc = "0xb4 - DMA stream 6 FIFO control register"]
    pub dma_s6fcr: DMA_S6FCR,
    #[doc = "0xb8 - This register is used to configure the concerned stream."]
    pub dma_s7cr: DMA_S7CR,
    #[doc = "0xbc - DMA stream 7 number of data register"]
    pub dma_s7ndtr: DMA_S7NDTR,
    #[doc = "0xc0 - DMA stream 7 peripheral address register"]
    pub dma_s7par: DMA_S7PAR,
    #[doc = "0xc4 - DMA stream 7 memory 0 address register"]
    pub dma_s7m0ar: DMA_S7M0AR,
    #[doc = "0xc8 - DMA stream 7 memory 1 address register"]
    pub dma_s7m1ar: DMA_S7M1AR,
    #[doc = "0xcc - DMA stream 7 FIFO control register"]
    pub dma_s7fcr: DMA_S7FCR,
    _reserved52: [u8; 0x031c],
    #[doc = "0x3ec - DMA hardware configuration 2register"]
    pub dma_hwcfgr2: DMA_HWCFGR2,
    #[doc = "0x3f0 - DMA hardware configuration 1 register"]
    pub dma_hwcfgr1: DMA_HWCFGR1,
    #[doc = "0x3f4 - This register identifies the version of the IP."]
    pub dma_verr: DMA_VERR,
    #[doc = "0x3f8 - DMA IP identification register"]
    pub dma_ipdr: DMA_IPDR,
    #[doc = "0x3fc - DMA size identification register"]
    pub dma_sidr: DMA_SIDR,
}
#[doc = "DMA_LISR (r) register accessor: an alias for `Reg<DMA_LISR_SPEC>`"]
pub type DMA_LISR = crate::Reg<dma_lisr::DMA_LISR_SPEC>;
#[doc = "DMA low interrupt status register"]
pub mod dma_lisr;
#[doc = "DMA_HISR (r) register accessor: an alias for `Reg<DMA_HISR_SPEC>`"]
pub type DMA_HISR = crate::Reg<dma_hisr::DMA_HISR_SPEC>;
#[doc = "DMA high interrupt status register"]
pub mod dma_hisr;
#[doc = "DMA_LIFCR (w) register accessor: an alias for `Reg<DMA_LIFCR_SPEC>`"]
pub type DMA_LIFCR = crate::Reg<dma_lifcr::DMA_LIFCR_SPEC>;
#[doc = "DMA low interrupt flag clear register"]
pub mod dma_lifcr;
#[doc = "DMA_HIFCR (w) register accessor: an alias for `Reg<DMA_HIFCR_SPEC>`"]
pub type DMA_HIFCR = crate::Reg<dma_hifcr::DMA_HIFCR_SPEC>;
#[doc = "DMA high interrupt flag clear register"]
pub mod dma_hifcr;
#[doc = "DMA_S0CR (rw) register accessor: an alias for `Reg<DMA_S0CR_SPEC>`"]
pub type DMA_S0CR = crate::Reg<dma_s0cr::DMA_S0CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s0cr;
#[doc = "DMA_S0NDTR (rw) register accessor: an alias for `Reg<DMA_S0NDTR_SPEC>`"]
pub type DMA_S0NDTR = crate::Reg<dma_s0ndtr::DMA_S0NDTR_SPEC>;
#[doc = "DMA stream 0 number of data register"]
pub mod dma_s0ndtr;
#[doc = "DMA_S0PAR (rw) register accessor: an alias for `Reg<DMA_S0PAR_SPEC>`"]
pub type DMA_S0PAR = crate::Reg<dma_s0par::DMA_S0PAR_SPEC>;
#[doc = "DMA stream 0 peripheral address register"]
pub mod dma_s0par;
#[doc = "DMA_S0M0AR (rw) register accessor: an alias for `Reg<DMA_S0M0AR_SPEC>`"]
pub type DMA_S0M0AR = crate::Reg<dma_s0m0ar::DMA_S0M0AR_SPEC>;
#[doc = "DMA stream 0 memory 0 address register"]
pub mod dma_s0m0ar;
#[doc = "DMA_S0M1AR (rw) register accessor: an alias for `Reg<DMA_S0M1AR_SPEC>`"]
pub type DMA_S0M1AR = crate::Reg<dma_s0m1ar::DMA_S0M1AR_SPEC>;
#[doc = "DMA stream 0 memory 1 address register"]
pub mod dma_s0m1ar;
#[doc = "DMA_S0FCR (rw) register accessor: an alias for `Reg<DMA_S0FCR_SPEC>`"]
pub type DMA_S0FCR = crate::Reg<dma_s0fcr::DMA_S0FCR_SPEC>;
#[doc = "DMA stream 0 FIFO control register"]
pub mod dma_s0fcr;
#[doc = "DMA_S1CR (rw) register accessor: an alias for `Reg<DMA_S1CR_SPEC>`"]
pub type DMA_S1CR = crate::Reg<dma_s1cr::DMA_S1CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s1cr;
#[doc = "DMA_S1NDTR (rw) register accessor: an alias for `Reg<DMA_S1NDTR_SPEC>`"]
pub type DMA_S1NDTR = crate::Reg<dma_s1ndtr::DMA_S1NDTR_SPEC>;
#[doc = "DMA stream 1 number of data register"]
pub mod dma_s1ndtr;
#[doc = "DMA_S1PAR (rw) register accessor: an alias for `Reg<DMA_S1PAR_SPEC>`"]
pub type DMA_S1PAR = crate::Reg<dma_s1par::DMA_S1PAR_SPEC>;
#[doc = "DMA stream 1 peripheral address register"]
pub mod dma_s1par;
#[doc = "DMA_S1M0AR (rw) register accessor: an alias for `Reg<DMA_S1M0AR_SPEC>`"]
pub type DMA_S1M0AR = crate::Reg<dma_s1m0ar::DMA_S1M0AR_SPEC>;
#[doc = "DMA stream 1 memory 0 address register"]
pub mod dma_s1m0ar;
#[doc = "DMA_S1M1AR (rw) register accessor: an alias for `Reg<DMA_S1M1AR_SPEC>`"]
pub type DMA_S1M1AR = crate::Reg<dma_s1m1ar::DMA_S1M1AR_SPEC>;
#[doc = "DMA stream 1 memory 1 address register"]
pub mod dma_s1m1ar;
#[doc = "DMA_S1FCR (rw) register accessor: an alias for `Reg<DMA_S1FCR_SPEC>`"]
pub type DMA_S1FCR = crate::Reg<dma_s1fcr::DMA_S1FCR_SPEC>;
#[doc = "DMA stream 1 FIFO control register"]
pub mod dma_s1fcr;
#[doc = "DMA_S2CR (rw) register accessor: an alias for `Reg<DMA_S2CR_SPEC>`"]
pub type DMA_S2CR = crate::Reg<dma_s2cr::DMA_S2CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s2cr;
#[doc = "DMA_S2NDTR (rw) register accessor: an alias for `Reg<DMA_S2NDTR_SPEC>`"]
pub type DMA_S2NDTR = crate::Reg<dma_s2ndtr::DMA_S2NDTR_SPEC>;
#[doc = "DMA stream 2 number of data register"]
pub mod dma_s2ndtr;
#[doc = "DMA_S2PAR (rw) register accessor: an alias for `Reg<DMA_S2PAR_SPEC>`"]
pub type DMA_S2PAR = crate::Reg<dma_s2par::DMA_S2PAR_SPEC>;
#[doc = "DMA stream 2 peripheral address register"]
pub mod dma_s2par;
#[doc = "DMA_S2M0AR (rw) register accessor: an alias for `Reg<DMA_S2M0AR_SPEC>`"]
pub type DMA_S2M0AR = crate::Reg<dma_s2m0ar::DMA_S2M0AR_SPEC>;
#[doc = "DMA stream 2 memory 0 address register"]
pub mod dma_s2m0ar;
#[doc = "DMA_S2M1AR (rw) register accessor: an alias for `Reg<DMA_S2M1AR_SPEC>`"]
pub type DMA_S2M1AR = crate::Reg<dma_s2m1ar::DMA_S2M1AR_SPEC>;
#[doc = "DMA stream 2 memory 1 address register"]
pub mod dma_s2m1ar;
#[doc = "DMA_S2FCR (rw) register accessor: an alias for `Reg<DMA_S2FCR_SPEC>`"]
pub type DMA_S2FCR = crate::Reg<dma_s2fcr::DMA_S2FCR_SPEC>;
#[doc = "DMA stream 2 FIFO control register"]
pub mod dma_s2fcr;
#[doc = "DMA_S3CR (rw) register accessor: an alias for `Reg<DMA_S3CR_SPEC>`"]
pub type DMA_S3CR = crate::Reg<dma_s3cr::DMA_S3CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s3cr;
#[doc = "DMA_S3NDTR (rw) register accessor: an alias for `Reg<DMA_S3NDTR_SPEC>`"]
pub type DMA_S3NDTR = crate::Reg<dma_s3ndtr::DMA_S3NDTR_SPEC>;
#[doc = "DMA stream 3 number of data register"]
pub mod dma_s3ndtr;
#[doc = "DMA_S3PAR (rw) register accessor: an alias for `Reg<DMA_S3PAR_SPEC>`"]
pub type DMA_S3PAR = crate::Reg<dma_s3par::DMA_S3PAR_SPEC>;
#[doc = "DMA stream 3 peripheral address register"]
pub mod dma_s3par;
#[doc = "DMA_S3M0AR (rw) register accessor: an alias for `Reg<DMA_S3M0AR_SPEC>`"]
pub type DMA_S3M0AR = crate::Reg<dma_s3m0ar::DMA_S3M0AR_SPEC>;
#[doc = "DMA stream 3 memory 0 address register"]
pub mod dma_s3m0ar;
#[doc = "DMA_S3M1AR (rw) register accessor: an alias for `Reg<DMA_S3M1AR_SPEC>`"]
pub type DMA_S3M1AR = crate::Reg<dma_s3m1ar::DMA_S3M1AR_SPEC>;
#[doc = "DMA stream 3 memory 1 address register"]
pub mod dma_s3m1ar;
#[doc = "DMA_S3FCR (rw) register accessor: an alias for `Reg<DMA_S3FCR_SPEC>`"]
pub type DMA_S3FCR = crate::Reg<dma_s3fcr::DMA_S3FCR_SPEC>;
#[doc = "DMA stream 3 FIFO control register"]
pub mod dma_s3fcr;
#[doc = "DMA_S4CR (rw) register accessor: an alias for `Reg<DMA_S4CR_SPEC>`"]
pub type DMA_S4CR = crate::Reg<dma_s4cr::DMA_S4CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s4cr;
#[doc = "DMA_S4NDTR (rw) register accessor: an alias for `Reg<DMA_S4NDTR_SPEC>`"]
pub type DMA_S4NDTR = crate::Reg<dma_s4ndtr::DMA_S4NDTR_SPEC>;
#[doc = "DMA stream 4 number of data register"]
pub mod dma_s4ndtr;
#[doc = "DMA_S4PAR (rw) register accessor: an alias for `Reg<DMA_S4PAR_SPEC>`"]
pub type DMA_S4PAR = crate::Reg<dma_s4par::DMA_S4PAR_SPEC>;
#[doc = "DMA stream 4 peripheral address register"]
pub mod dma_s4par;
#[doc = "DMA_S4M0AR (rw) register accessor: an alias for `Reg<DMA_S4M0AR_SPEC>`"]
pub type DMA_S4M0AR = crate::Reg<dma_s4m0ar::DMA_S4M0AR_SPEC>;
#[doc = "DMA stream 4 memory 0 address register"]
pub mod dma_s4m0ar;
#[doc = "DMA_S4M1AR (rw) register accessor: an alias for `Reg<DMA_S4M1AR_SPEC>`"]
pub type DMA_S4M1AR = crate::Reg<dma_s4m1ar::DMA_S4M1AR_SPEC>;
#[doc = "DMA stream 4 memory 1 address register"]
pub mod dma_s4m1ar;
#[doc = "DMA_S4FCR (rw) register accessor: an alias for `Reg<DMA_S4FCR_SPEC>`"]
pub type DMA_S4FCR = crate::Reg<dma_s4fcr::DMA_S4FCR_SPEC>;
#[doc = "DMA stream 4 FIFO control register"]
pub mod dma_s4fcr;
#[doc = "DMA_S5CR (rw) register accessor: an alias for `Reg<DMA_S5CR_SPEC>`"]
pub type DMA_S5CR = crate::Reg<dma_s5cr::DMA_S5CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s5cr;
#[doc = "DMA_S5NDTR (rw) register accessor: an alias for `Reg<DMA_S5NDTR_SPEC>`"]
pub type DMA_S5NDTR = crate::Reg<dma_s5ndtr::DMA_S5NDTR_SPEC>;
#[doc = "DMA stream 5 number of data register"]
pub mod dma_s5ndtr;
#[doc = "DMA_S5PAR (rw) register accessor: an alias for `Reg<DMA_S5PAR_SPEC>`"]
pub type DMA_S5PAR = crate::Reg<dma_s5par::DMA_S5PAR_SPEC>;
#[doc = "DMA stream 5 peripheral address register"]
pub mod dma_s5par;
#[doc = "DMA_S5M0AR (rw) register accessor: an alias for `Reg<DMA_S5M0AR_SPEC>`"]
pub type DMA_S5M0AR = crate::Reg<dma_s5m0ar::DMA_S5M0AR_SPEC>;
#[doc = "DMA stream 5 memory 0 address register"]
pub mod dma_s5m0ar;
#[doc = "DMA_S5M1AR (rw) register accessor: an alias for `Reg<DMA_S5M1AR_SPEC>`"]
pub type DMA_S5M1AR = crate::Reg<dma_s5m1ar::DMA_S5M1AR_SPEC>;
#[doc = "DMA stream 5 memory 1 address register"]
pub mod dma_s5m1ar;
#[doc = "DMA_S5FCR (rw) register accessor: an alias for `Reg<DMA_S5FCR_SPEC>`"]
pub type DMA_S5FCR = crate::Reg<dma_s5fcr::DMA_S5FCR_SPEC>;
#[doc = "DMA stream 5 FIFO control register"]
pub mod dma_s5fcr;
#[doc = "DMA_S6CR (rw) register accessor: an alias for `Reg<DMA_S6CR_SPEC>`"]
pub type DMA_S6CR = crate::Reg<dma_s6cr::DMA_S6CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s6cr;
#[doc = "DMA_S6NDTR (rw) register accessor: an alias for `Reg<DMA_S6NDTR_SPEC>`"]
pub type DMA_S6NDTR = crate::Reg<dma_s6ndtr::DMA_S6NDTR_SPEC>;
#[doc = "DMA stream 6 number of data register"]
pub mod dma_s6ndtr;
#[doc = "DMA_S6PAR (rw) register accessor: an alias for `Reg<DMA_S6PAR_SPEC>`"]
pub type DMA_S6PAR = crate::Reg<dma_s6par::DMA_S6PAR_SPEC>;
#[doc = "DMA stream 6 peripheral address register"]
pub mod dma_s6par;
#[doc = "DMA_S6M0AR (rw) register accessor: an alias for `Reg<DMA_S6M0AR_SPEC>`"]
pub type DMA_S6M0AR = crate::Reg<dma_s6m0ar::DMA_S6M0AR_SPEC>;
#[doc = "DMA stream 6 memory 0 address register"]
pub mod dma_s6m0ar;
#[doc = "DMA_S6M1AR (rw) register accessor: an alias for `Reg<DMA_S6M1AR_SPEC>`"]
pub type DMA_S6M1AR = crate::Reg<dma_s6m1ar::DMA_S6M1AR_SPEC>;
#[doc = "DMA stream 6 memory 1 address register"]
pub mod dma_s6m1ar;
#[doc = "DMA_S6FCR (rw) register accessor: an alias for `Reg<DMA_S6FCR_SPEC>`"]
pub type DMA_S6FCR = crate::Reg<dma_s6fcr::DMA_S6FCR_SPEC>;
#[doc = "DMA stream 6 FIFO control register"]
pub mod dma_s6fcr;
#[doc = "DMA_S7CR (rw) register accessor: an alias for `Reg<DMA_S7CR_SPEC>`"]
pub type DMA_S7CR = crate::Reg<dma_s7cr::DMA_S7CR_SPEC>;
#[doc = "This register is used to configure the concerned stream."]
pub mod dma_s7cr;
#[doc = "DMA_S7NDTR (rw) register accessor: an alias for `Reg<DMA_S7NDTR_SPEC>`"]
pub type DMA_S7NDTR = crate::Reg<dma_s7ndtr::DMA_S7NDTR_SPEC>;
#[doc = "DMA stream 7 number of data register"]
pub mod dma_s7ndtr;
#[doc = "DMA_S7PAR (rw) register accessor: an alias for `Reg<DMA_S7PAR_SPEC>`"]
pub type DMA_S7PAR = crate::Reg<dma_s7par::DMA_S7PAR_SPEC>;
#[doc = "DMA stream 7 peripheral address register"]
pub mod dma_s7par;
#[doc = "DMA_S7M0AR (rw) register accessor: an alias for `Reg<DMA_S7M0AR_SPEC>`"]
pub type DMA_S7M0AR = crate::Reg<dma_s7m0ar::DMA_S7M0AR_SPEC>;
#[doc = "DMA stream 7 memory 0 address register"]
pub mod dma_s7m0ar;
#[doc = "DMA_S7M1AR (rw) register accessor: an alias for `Reg<DMA_S7M1AR_SPEC>`"]
pub type DMA_S7M1AR = crate::Reg<dma_s7m1ar::DMA_S7M1AR_SPEC>;
#[doc = "DMA stream 7 memory 1 address register"]
pub mod dma_s7m1ar;
#[doc = "DMA_S7FCR (rw) register accessor: an alias for `Reg<DMA_S7FCR_SPEC>`"]
pub type DMA_S7FCR = crate::Reg<dma_s7fcr::DMA_S7FCR_SPEC>;
#[doc = "DMA stream 7 FIFO control register"]
pub mod dma_s7fcr;
#[doc = "DMA_HWCFGR2 (r) register accessor: an alias for `Reg<DMA_HWCFGR2_SPEC>`"]
pub type DMA_HWCFGR2 = crate::Reg<dma_hwcfgr2::DMA_HWCFGR2_SPEC>;
#[doc = "DMA hardware configuration 2register"]
pub mod dma_hwcfgr2;
#[doc = "DMA_HWCFGR1 (r) register accessor: an alias for `Reg<DMA_HWCFGR1_SPEC>`"]
pub type DMA_HWCFGR1 = crate::Reg<dma_hwcfgr1::DMA_HWCFGR1_SPEC>;
#[doc = "DMA hardware configuration 1 register"]
pub mod dma_hwcfgr1;
#[doc = "DMA_VERR (r) register accessor: an alias for `Reg<DMA_VERR_SPEC>`"]
pub type DMA_VERR = crate::Reg<dma_verr::DMA_VERR_SPEC>;
#[doc = "This register identifies the version of the IP."]
pub mod dma_verr;
#[doc = "DMA_IPDR (r) register accessor: an alias for `Reg<DMA_IPDR_SPEC>`"]
pub type DMA_IPDR = crate::Reg<dma_ipdr::DMA_IPDR_SPEC>;
#[doc = "DMA IP identification register"]
pub mod dma_ipdr;
#[doc = "DMA_SIDR (r) register accessor: an alias for `Reg<DMA_SIDR_SPEC>`"]
pub type DMA_SIDR = crate::Reg<dma_sidr::DMA_SIDR_SPEC>;
#[doc = "DMA size identification register"]
pub mod dma_sidr;
