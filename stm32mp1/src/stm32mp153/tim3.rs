#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM3 control register 1"]
    pub tim3_cr1: TIM3_CR1,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - TIM3 control register 2"]
    pub tim3_cr2: TIM3_CR2,
    #[doc = "0x08 - TIM3 slave mode control register"]
    pub tim3_smcr: TIM3_SMCR,
    #[doc = "0x0c - TIM3 DMA/interrupt enable register"]
    pub tim3_dier: TIM3_DIER,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - TIM3 status register"]
    pub tim3_sr: TIM3_SR,
    #[doc = "0x14 - TIM3 event generation register"]
    pub tim3_egr: TIM3_EGR,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim3_ccmr1alternate3: TIM3_CCMR1ALTERNATE3,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim3_ccmr2alternate19: TIM3_CCMR2ALTERNATE19,
    #[doc = "0x20 - TIM3 capture/compare enable register"]
    pub tim3_ccer: TIM3_CCER,
    #[doc = "0x24 - TIM3 counter"]
    pub tim3_cnt: TIM3_CNT,
    #[doc = "0x28 - TIM3 prescaler"]
    pub tim3_psc: TIM3_PSC,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - TIM3 auto-reload register"]
    pub tim3_arr: TIM3_ARR,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - TIM3 repetition counter register"]
    pub tim3_rcr: TIM3_RCR,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - TIM3 capture/compare register 1"]
    pub tim3_ccr1: TIM3_CCR1,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - TIM3 capture/compare register 2"]
    pub tim3_ccr2: TIM3_CCR2,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - TIM3 capture/compare register 3"]
    pub tim3_ccr3: TIM3_CCR3,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - TIM3 capture/compare register 4"]
    pub tim3_ccr4: TIM3_CCR4,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim3_bdtr: TIM3_BDTR,
    #[doc = "0x48 - TIM3 DMA control register"]
    pub tim3_dcr: TIM3_DCR,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - TIM3 DMA address for full transfer"]
    pub tim3_dmar: TIM3_DMAR,
    _reserved20: [u8; 0x04],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim3_ccmr3: TIM3_CCMR3,
    #[doc = "0x58 - TIM3 capture/compare register 5"]
    pub tim3_ccr5: TIM3_CCR5,
    #[doc = "0x5c - TIM3 capture/compare register 6"]
    pub tim3_ccr6: TIM3_CCR6,
}
#[doc = "TIM3_CR1 (rw) register accessor: an alias for `Reg<TIM3_CR1_SPEC>`"]
pub type TIM3_CR1 = crate::Reg<tim3_cr1::TIM3_CR1_SPEC>;
#[doc = "TIM3 control register 1"]
pub mod tim3_cr1;
#[doc = "TIM3_CR2 (rw) register accessor: an alias for `Reg<TIM3_CR2_SPEC>`"]
pub type TIM3_CR2 = crate::Reg<tim3_cr2::TIM3_CR2_SPEC>;
#[doc = "TIM3 control register 2"]
pub mod tim3_cr2;
#[doc = "TIM3_SMCR (rw) register accessor: an alias for `Reg<TIM3_SMCR_SPEC>`"]
pub type TIM3_SMCR = crate::Reg<tim3_smcr::TIM3_SMCR_SPEC>;
#[doc = "TIM3 slave mode control register"]
pub mod tim3_smcr;
#[doc = "TIM3_DIER (rw) register accessor: an alias for `Reg<TIM3_DIER_SPEC>`"]
pub type TIM3_DIER = crate::Reg<tim3_dier::TIM3_DIER_SPEC>;
#[doc = "TIM3 DMA/interrupt enable register"]
pub mod tim3_dier;
#[doc = "TIM3_SR (rw) register accessor: an alias for `Reg<TIM3_SR_SPEC>`"]
pub type TIM3_SR = crate::Reg<tim3_sr::TIM3_SR_SPEC>;
#[doc = "TIM3 status register"]
pub mod tim3_sr;
#[doc = "TIM3_EGR (w) register accessor: an alias for `Reg<TIM3_EGR_SPEC>`"]
pub type TIM3_EGR = crate::Reg<tim3_egr::TIM3_EGR_SPEC>;
#[doc = "TIM3 event generation register"]
pub mod tim3_egr;
#[doc = "TIM3_CCMR1ALTERNATE3 (rw) register accessor: an alias for `Reg<TIM3_CCMR1ALTERNATE3_SPEC>`"]
pub type TIM3_CCMR1ALTERNATE3 = crate::Reg<tim3_ccmr1alternate3::TIM3_CCMR1ALTERNATE3_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr1alternate3;
#[doc = "TIM3_CCMR2ALTERNATE19 (rw) register accessor: an alias for `Reg<TIM3_CCMR2ALTERNATE19_SPEC>`"]
pub type TIM3_CCMR2ALTERNATE19 = crate::Reg<tim3_ccmr2alternate19::TIM3_CCMR2ALTERNATE19_SPEC>;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr2alternate19;
#[doc = "TIM3_CCER (rw) register accessor: an alias for `Reg<TIM3_CCER_SPEC>`"]
pub type TIM3_CCER = crate::Reg<tim3_ccer::TIM3_CCER_SPEC>;
#[doc = "TIM3 capture/compare enable register"]
pub mod tim3_ccer;
#[doc = "TIM3_CNT (rw) register accessor: an alias for `Reg<TIM3_CNT_SPEC>`"]
pub type TIM3_CNT = crate::Reg<tim3_cnt::TIM3_CNT_SPEC>;
#[doc = "TIM3 counter"]
pub mod tim3_cnt;
#[doc = "TIM3_PSC (rw) register accessor: an alias for `Reg<TIM3_PSC_SPEC>`"]
pub type TIM3_PSC = crate::Reg<tim3_psc::TIM3_PSC_SPEC>;
#[doc = "TIM3 prescaler"]
pub mod tim3_psc;
#[doc = "TIM3_ARR (rw) register accessor: an alias for `Reg<TIM3_ARR_SPEC>`"]
pub type TIM3_ARR = crate::Reg<tim3_arr::TIM3_ARR_SPEC>;
#[doc = "TIM3 auto-reload register"]
pub mod tim3_arr;
#[doc = "TIM3_RCR (rw) register accessor: an alias for `Reg<TIM3_RCR_SPEC>`"]
pub type TIM3_RCR = crate::Reg<tim3_rcr::TIM3_RCR_SPEC>;
#[doc = "TIM3 repetition counter register"]
pub mod tim3_rcr;
#[doc = "TIM3_CCR1 (rw) register accessor: an alias for `Reg<TIM3_CCR1_SPEC>`"]
pub type TIM3_CCR1 = crate::Reg<tim3_ccr1::TIM3_CCR1_SPEC>;
#[doc = "TIM3 capture/compare register 1"]
pub mod tim3_ccr1;
#[doc = "TIM3_CCR2 (rw) register accessor: an alias for `Reg<TIM3_CCR2_SPEC>`"]
pub type TIM3_CCR2 = crate::Reg<tim3_ccr2::TIM3_CCR2_SPEC>;
#[doc = "TIM3 capture/compare register 2"]
pub mod tim3_ccr2;
#[doc = "TIM3_CCR3 (rw) register accessor: an alias for `Reg<TIM3_CCR3_SPEC>`"]
pub type TIM3_CCR3 = crate::Reg<tim3_ccr3::TIM3_CCR3_SPEC>;
#[doc = "TIM3 capture/compare register 3"]
pub mod tim3_ccr3;
#[doc = "TIM3_CCR4 (rw) register accessor: an alias for `Reg<TIM3_CCR4_SPEC>`"]
pub type TIM3_CCR4 = crate::Reg<tim3_ccr4::TIM3_CCR4_SPEC>;
#[doc = "TIM3 capture/compare register 4"]
pub mod tim3_ccr4;
#[doc = "TIM3_BDTR (rw) register accessor: an alias for `Reg<TIM3_BDTR_SPEC>`"]
pub type TIM3_BDTR = crate::Reg<tim3_bdtr::TIM3_BDTR_SPEC>;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim3_bdtr;
#[doc = "TIM3_DCR (rw) register accessor: an alias for `Reg<TIM3_DCR_SPEC>`"]
pub type TIM3_DCR = crate::Reg<tim3_dcr::TIM3_DCR_SPEC>;
#[doc = "TIM3 DMA control register"]
pub mod tim3_dcr;
#[doc = "TIM3_DMAR (rw) register accessor: an alias for `Reg<TIM3_DMAR_SPEC>`"]
pub type TIM3_DMAR = crate::Reg<tim3_dmar::TIM3_DMAR_SPEC>;
#[doc = "TIM3 DMA address for full transfer"]
pub mod tim3_dmar;
#[doc = "TIM3_CCMR3 (rw) register accessor: an alias for `Reg<TIM3_CCMR3_SPEC>`"]
pub type TIM3_CCMR3 = crate::Reg<tim3_ccmr3::TIM3_CCMR3_SPEC>;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim3_ccmr3;
#[doc = "TIM3_CCR5 (rw) register accessor: an alias for `Reg<TIM3_CCR5_SPEC>`"]
pub type TIM3_CCR5 = crate::Reg<tim3_ccr5::TIM3_CCR5_SPEC>;
#[doc = "TIM3 capture/compare register 5"]
pub mod tim3_ccr5;
#[doc = "TIM3_CCR6 (rw) register accessor: an alias for `Reg<TIM3_CCR6_SPEC>`"]
pub type TIM3_CCR6 = crate::Reg<tim3_ccr6::TIM3_CCR6_SPEC>;
#[doc = "TIM3 capture/compare register 6"]
pub mod tim3_ccr6;
