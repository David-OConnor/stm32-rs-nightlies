#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMMC power control register"]
    pub sdmmc_power: SDMMC_POWER,
    #[doc = "0x04 - The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
    pub sdmmc_clkcr: SDMMC_CLKCR,
    #[doc = "0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
    pub sdmmc_argr: SDMMC_ARGR,
    #[doc = "0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
    pub sdmmc_cmdr: SDMMC_CMDR,
    #[doc = "0x10 - SDMMC command response register"]
    pub sdmmc_respcmdr: SDMMC_RESPCMDR,
    #[doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp1r: SDMMC_RESP1R,
    #[doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp2r: SDMMC_RESP2R,
    #[doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp3r: SDMMC_RESP3R,
    #[doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
    pub sdmmc_resp4r: SDMMC_RESP4R,
    #[doc = "0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
    pub sdmmc_dtimer: SDMMC_DTIMER,
    #[doc = "0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
    pub sdmmc_dlenr: SDMMC_DLENR,
    #[doc = "0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM)."]
    pub sdmmc_dctrl: SDMMC_DCTRL,
    #[doc = "0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
    pub sdmmc_dcntr: SDMMC_DCNTR,
    #[doc = "0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
    pub sdmmc_star: SDMMC_STAR,
    #[doc = "0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
    pub sdmmc_icr: SDMMC_ICR,
    #[doc = "0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub sdmmc_maskr: SDMMC_MASKR,
    #[doc = "0x40 - The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
    pub sdmmc_acktimer: SDMMC_ACKTIMER,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
    pub sdmmc_idmactrlr: SDMMC_IDMACTRLR,
    #[doc = "0x54 - The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
    pub sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    #[doc = "0x58 - The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
    pub sdmmc_idmabase0r: SDMMC_IDMABASE0R,
    #[doc = "0x5c - The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
    pub sdmmc_idmabase1r: SDMMC_IDMABASE1R,
    _reserved21: [u8; 0x20],
    #[doc = "0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
    pub sdmmc_fifor: SDMMC_FIFOR,
    _reserved22: [u8; 0x0370],
    #[doc = "0x3f4 - SDMMC IP version register"]
    pub sdmmc_ver: SDMMC_VER,
    #[doc = "0x3f8 - SDMMC IP identification register"]
    pub sdmmc_id: SDMMC_ID,
}
#[doc = "SDMMC_POWER (rw) register accessor: an alias for `Reg<SDMMC_POWER_SPEC>`"]
pub type SDMMC_POWER = crate::Reg<sdmmc_power::SDMMC_POWER_SPEC>;
#[doc = "SDMMC power control register"]
pub mod sdmmc_power;
#[doc = "SDMMC_CLKCR (rw) register accessor: an alias for `Reg<SDMMC_CLKCR_SPEC>`"]
pub type SDMMC_CLKCR = crate::Reg<sdmmc_clkcr::SDMMC_CLKCR_SPEC>;
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width."]
pub mod sdmmc_clkcr;
#[doc = "SDMMC_ARGR (rw) register accessor: an alias for `Reg<SDMMC_ARGR_SPEC>`"]
pub type SDMMC_ARGR = crate::Reg<sdmmc_argr::SDMMC_ARGR_SPEC>;
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message."]
pub mod sdmmc_argr;
#[doc = "SDMMC_CMDR (rw) register accessor: an alias for `Reg<SDMMC_CMDR_SPEC>`"]
pub type SDMMC_CMDR = crate::Reg<sdmmc_cmdr::SDMMC_CMDR_SPEC>;
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM)."]
pub mod sdmmc_cmdr;
#[doc = "SDMMC_RESP1R (r) register accessor: an alias for `Reg<SDMMC_RESP1R_SPEC>`"]
pub type SDMMC_RESP1R = crate::Reg<sdmmc_resp1r::SDMMC_RESP1R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp1r;
#[doc = "SDMMC_RESP2R (r) register accessor: an alias for `Reg<SDMMC_RESP2R_SPEC>`"]
pub type SDMMC_RESP2R = crate::Reg<sdmmc_resp2r::SDMMC_RESP2R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp2r;
#[doc = "SDMMC_RESP3R (r) register accessor: an alias for `Reg<SDMMC_RESP3R_SPEC>`"]
pub type SDMMC_RESP3R = crate::Reg<sdmmc_resp3r::SDMMC_RESP3R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp3r;
#[doc = "SDMMC_RESP4R (r) register accessor: an alias for `Reg<SDMMC_RESP4R_SPEC>`"]
pub type SDMMC_RESP4R = crate::Reg<sdmmc_resp4r::SDMMC_RESP4R_SPEC>;
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."]
pub mod sdmmc_resp4r;
#[doc = "SDMMC_DTIMER (rw) register accessor: an alias for `Reg<SDMMC_DTIMER_SPEC>`"]
pub type SDMMC_DTIMER = crate::Reg<sdmmc_dtimer::SDMMC_DTIMER_SPEC>;
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set."]
pub mod sdmmc_dtimer;
#[doc = "SDMMC_DLENR (rw) register accessor: an alias for `Reg<SDMMC_DLENR_SPEC>`"]
pub type SDMMC_DLENR = crate::Reg<sdmmc_dlenr::SDMMC_DLENR_SPEC>;
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts."]
pub mod sdmmc_dlenr;
#[doc = "SDMMC_DCTRL (rw) register accessor: an alias for `Reg<SDMMC_DCTRL_SPEC>`"]
pub type SDMMC_DCTRL = crate::Reg<sdmmc_dctrl::SDMMC_DCTRL_SPEC>;
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM)."]
pub mod sdmmc_dctrl;
#[doc = "SDMMC_DCNTR (r) register accessor: an alias for `Reg<SDMMC_DCNTR_SPEC>`"]
pub type SDMMC_DCNTR = crate::Reg<sdmmc_dcntr::SDMMC_DCNTR_SPEC>;
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set."]
pub mod sdmmc_dcntr;
#[doc = "SDMMC_STAR (r) register accessor: an alias for `Reg<SDMMC_STAR_SPEC>`"]
pub type SDMMC_STAR = crate::Reg<sdmmc_star::SDMMC_STAR_SPEC>;
#[doc = "The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \\[29,21,11:0\\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \\[20:12\\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)"]
pub mod sdmmc_star;
#[doc = "SDMMC_ICR (rw) register accessor: an alias for `Reg<SDMMC_ICR_SPEC>`"]
pub type SDMMC_ICR = crate::Reg<sdmmc_icr::SDMMC_ICR_SPEC>;
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register."]
pub mod sdmmc_icr;
#[doc = "SDMMC_MASKR (rw) register accessor: an alias for `Reg<SDMMC_MASKR_SPEC>`"]
pub type SDMMC_MASKR = crate::Reg<sdmmc_maskr::SDMMC_MASKR_SPEC>;
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod sdmmc_maskr;
#[doc = "SDMMC_ACKTIMER (rw) register accessor: an alias for `Reg<SDMMC_ACKTIMER_SPEC>`"]
pub type SDMMC_ACKTIMER = crate::Reg<sdmmc_acktimer::SDMMC_ACKTIMER_SPEC>;
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set."]
pub mod sdmmc_acktimer;
#[doc = "SDMMC_IDMACTRLR (rw) register accessor: an alias for `Reg<SDMMC_IDMACTRLR_SPEC>`"]
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLR_SPEC>;
#[doc = "The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs contain 32 entries on 32 sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO."]
pub mod sdmmc_idmactrlr;
#[doc = "SDMMC_IDMABSIZER (rw) register accessor: an alias for `Reg<SDMMC_IDMABSIZER_SPEC>`"]
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZER_SPEC>;
#[doc = "The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration."]
pub mod sdmmc_idmabsizer;
#[doc = "SDMMC_IDMABASE0R (rw) register accessor: an alias for `Reg<SDMMC_IDMABASE0R_SPEC>`"]
pub type SDMMC_IDMABASE0R = crate::Reg<sdmmc_idmabase0r::SDMMC_IDMABASE0R_SPEC>;
#[doc = "The SDMMC_IDMABASE0R register contains the memory buffer base address in single buffer configuration and the buffer 0 base address in double buffer configuration."]
pub mod sdmmc_idmabase0r;
#[doc = "SDMMC_IDMABASE1R (rw) register accessor: an alias for `Reg<SDMMC_IDMABASE1R_SPEC>`"]
pub type SDMMC_IDMABASE1R = crate::Reg<sdmmc_idmabase1r::SDMMC_IDMABASE1R_SPEC>;
#[doc = "The SDMMC_IDMABASE1R register contains the double buffer configuration second buffer memory base address."]
pub mod sdmmc_idmabase1r;
#[doc = "SDMMC_FIFOR (rw) register accessor: an alias for `Reg<SDMMC_FIFOR_SPEC>`"]
pub type SDMMC_FIFOR = crate::Reg<sdmmc_fifor::SDMMC_FIFOR_SPEC>;
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated."]
pub mod sdmmc_fifor;
#[doc = "SDMMC_VER (r) register accessor: an alias for `Reg<SDMMC_VER_SPEC>`"]
pub type SDMMC_VER = crate::Reg<sdmmc_ver::SDMMC_VER_SPEC>;
#[doc = "SDMMC IP version register"]
pub mod sdmmc_ver;
#[doc = "SDMMC_ID (r) register accessor: an alias for `Reg<SDMMC_ID_SPEC>`"]
pub type SDMMC_ID = crate::Reg<sdmmc_id::SDMMC_ID_SPEC>;
#[doc = "SDMMC IP identification register"]
pub mod sdmmc_id;
#[doc = "SDMMC_RESPCMDR (r) register accessor: an alias for `Reg<SDMMC_RESPCMDR_SPEC>`"]
pub type SDMMC_RESPCMDR = crate::Reg<sdmmc_respcmdr::SDMMC_RESPCMDR_SPEC>;
#[doc = "SDMMC command response register"]
pub mod sdmmc_respcmdr;
