#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA global interrupt/status register"]
    pub mdma_gisr0: MDMA_GISR0,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - MDMA secure global interrupt/status register"]
    pub mdma_sgisr0: MDMA_SGISR0,
    _reserved2: [u8; 0x34],
    #[doc = "0x40 - MDMA channel 0 interrupt/status register"]
    pub mdma_c0isr: MDMA_C0ISR,
    #[doc = "0x44 - MDMA channel 0 interrupt flag clear register"]
    pub mdma_c0ifcr: MDMA_C0IFCR,
    #[doc = "0x48 - MDMA channel 0 error status register"]
    pub mdma_c0esr: MDMA_C0ESR,
    #[doc = "0x4c - This register is used to control the concerned channel."]
    pub mdma_c0cr: MDMA_C0CR,
    #[doc = "0x50 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c0tcr: MDMA_C0TCR,
    #[doc = "0x54 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c0bndtr: MDMA_C0BNDTR,
    #[doc = "0x58 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c0sar: MDMA_C0SAR,
    #[doc = "0x5c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c0dar: MDMA_C0DAR,
    #[doc = "0x60 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c0brur: MDMA_C0BRUR,
    #[doc = "0x64 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c0lar: MDMA_C0LAR,
    #[doc = "0x68 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c0tbr: MDMA_C0TBR,
    _reserved13: [u8; 0x04],
    #[doc = "0x70 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c0mar: MDMA_C0MAR,
    #[doc = "0x74 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c0mdr: MDMA_C0MDR,
    _reserved15: [u8; 0x08],
    #[doc = "0x80 - MDMA channel 1 interrupt/status register"]
    pub mdma_c1isr: MDMA_C1ISR,
    #[doc = "0x84 - MDMA channel 1 interrupt flag clear register"]
    pub mdma_c1ifcr: MDMA_C1IFCR,
    #[doc = "0x88 - MDMA channel 1 error status register"]
    pub mdma_c1esr: MDMA_C1ESR,
    #[doc = "0x8c - This register is used to control the concerned channel."]
    pub mdma_c1cr: MDMA_C1CR,
    #[doc = "0x90 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c1tcr: MDMA_C1TCR,
    #[doc = "0x94 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c1bndtr: MDMA_C1BNDTR,
    #[doc = "0x98 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c1sar: MDMA_C1SAR,
    #[doc = "0x9c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c1dar: MDMA_C1DAR,
    #[doc = "0xa0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c1brur: MDMA_C1BRUR,
    #[doc = "0xa4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c1lar: MDMA_C1LAR,
    #[doc = "0xa8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c1tbr: MDMA_C1TBR,
    _reserved26: [u8; 0x04],
    #[doc = "0xb0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c1mar: MDMA_C1MAR,
    #[doc = "0xb4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c1mdr: MDMA_C1MDR,
    _reserved28: [u8; 0x08],
    #[doc = "0xc0 - MDMA channel 2 interrupt/status register"]
    pub mdma_c2isr: MDMA_C2ISR,
    #[doc = "0xc4 - MDMA channel 2 interrupt flag clear register"]
    pub mdma_c2ifcr: MDMA_C2IFCR,
    #[doc = "0xc8 - MDMA channel 2 error status register"]
    pub mdma_c2esr: MDMA_C2ESR,
    #[doc = "0xcc - This register is used to control the concerned channel."]
    pub mdma_c2cr: MDMA_C2CR,
    #[doc = "0xd0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c2tcr: MDMA_C2TCR,
    #[doc = "0xd4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c2bndtr: MDMA_C2BNDTR,
    #[doc = "0xd8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c2sar: MDMA_C2SAR,
    #[doc = "0xdc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c2dar: MDMA_C2DAR,
    #[doc = "0xe0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c2brur: MDMA_C2BRUR,
    #[doc = "0xe4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c2lar: MDMA_C2LAR,
    #[doc = "0xe8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c2tbr: MDMA_C2TBR,
    _reserved39: [u8; 0x04],
    #[doc = "0xf0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c2mar: MDMA_C2MAR,
    #[doc = "0xf4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c2mdr: MDMA_C2MDR,
    _reserved41: [u8; 0x08],
    #[doc = "0x100 - MDMA channel 3 interrupt/status register"]
    pub mdma_c3isr: MDMA_C3ISR,
    #[doc = "0x104 - MDMA channel 3 interrupt flag clear register"]
    pub mdma_c3ifcr: MDMA_C3IFCR,
    #[doc = "0x108 - MDMA channel 3 error status register"]
    pub mdma_c3esr: MDMA_C3ESR,
    #[doc = "0x10c - This register is used to control the concerned channel."]
    pub mdma_c3cr: MDMA_C3CR,
    #[doc = "0x110 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c3tcr: MDMA_C3TCR,
    #[doc = "0x114 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c3bndtr: MDMA_C3BNDTR,
    #[doc = "0x118 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c3sar: MDMA_C3SAR,
    #[doc = "0x11c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c3dar: MDMA_C3DAR,
    #[doc = "0x120 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c3brur: MDMA_C3BRUR,
    #[doc = "0x124 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c3lar: MDMA_C3LAR,
    #[doc = "0x128 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c3tbr: MDMA_C3TBR,
    _reserved52: [u8; 0x04],
    #[doc = "0x130 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c3mar: MDMA_C3MAR,
    #[doc = "0x134 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c3mdr: MDMA_C3MDR,
    _reserved54: [u8; 0x08],
    #[doc = "0x140 - MDMA channel 4 interrupt/status register"]
    pub mdma_c4isr: MDMA_C4ISR,
    #[doc = "0x144 - MDMA channel 4 interrupt flag clear register"]
    pub mdma_c4ifcr: MDMA_C4IFCR,
    #[doc = "0x148 - MDMA channel 4 error status register"]
    pub mdma_c4esr: MDMA_C4ESR,
    #[doc = "0x14c - This register is used to control the concerned channel."]
    pub mdma_c4cr: MDMA_C4CR,
    #[doc = "0x150 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c4tcr: MDMA_C4TCR,
    #[doc = "0x154 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c4bndtr: MDMA_C4BNDTR,
    #[doc = "0x158 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c4sar: MDMA_C4SAR,
    #[doc = "0x15c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c4dar: MDMA_C4DAR,
    #[doc = "0x160 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c4brur: MDMA_C4BRUR,
    #[doc = "0x164 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c4lar: MDMA_C4LAR,
    #[doc = "0x168 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c4tbr: MDMA_C4TBR,
    _reserved65: [u8; 0x04],
    #[doc = "0x170 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c4mar: MDMA_C4MAR,
    #[doc = "0x174 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c4mdr: MDMA_C4MDR,
    _reserved67: [u8; 0x08],
    #[doc = "0x180 - MDMA channel 5 interrupt/status register"]
    pub mdma_c5isr: MDMA_C5ISR,
    #[doc = "0x184 - MDMA channel 5 interrupt flag clear register"]
    pub mdma_c5ifcr: MDMA_C5IFCR,
    #[doc = "0x188 - MDMA channel 5 error status register"]
    pub mdma_c5esr: MDMA_C5ESR,
    #[doc = "0x18c - This register is used to control the concerned channel."]
    pub mdma_c5cr: MDMA_C5CR,
    #[doc = "0x190 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c5tcr: MDMA_C5TCR,
    #[doc = "0x194 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c5bndtr: MDMA_C5BNDTR,
    #[doc = "0x198 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c5sar: MDMA_C5SAR,
    #[doc = "0x19c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c5dar: MDMA_C5DAR,
    #[doc = "0x1a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c5brur: MDMA_C5BRUR,
    #[doc = "0x1a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c5lar: MDMA_C5LAR,
    #[doc = "0x1a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c5tbr: MDMA_C5TBR,
    _reserved78: [u8; 0x04],
    #[doc = "0x1b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c5mar: MDMA_C5MAR,
    #[doc = "0x1b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c5mdr: MDMA_C5MDR,
    _reserved80: [u8; 0x08],
    #[doc = "0x1c0 - MDMA channel 6 interrupt/status register"]
    pub mdma_c6isr: MDMA_C6ISR,
    #[doc = "0x1c4 - MDMA channel 6 interrupt flag clear register"]
    pub mdma_c6ifcr: MDMA_C6IFCR,
    #[doc = "0x1c8 - MDMA channel 6 error status register"]
    pub mdma_c6esr: MDMA_C6ESR,
    #[doc = "0x1cc - This register is used to control the concerned channel."]
    pub mdma_c6cr: MDMA_C6CR,
    #[doc = "0x1d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c6tcr: MDMA_C6TCR,
    #[doc = "0x1d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c6bndtr: MDMA_C6BNDTR,
    #[doc = "0x1d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c6sar: MDMA_C6SAR,
    #[doc = "0x1dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c6dar: MDMA_C6DAR,
    #[doc = "0x1e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c6brur: MDMA_C6BRUR,
    #[doc = "0x1e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c6lar: MDMA_C6LAR,
    #[doc = "0x1e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c6tbr: MDMA_C6TBR,
    _reserved91: [u8; 0x04],
    #[doc = "0x1f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c6mar: MDMA_C6MAR,
    #[doc = "0x1f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c6mdr: MDMA_C6MDR,
    _reserved93: [u8; 0x08],
    #[doc = "0x200 - MDMA channel 7 interrupt/status register"]
    pub mdma_c7isr: MDMA_C7ISR,
    #[doc = "0x204 - MDMA channel 7 interrupt flag clear register"]
    pub mdma_c7ifcr: MDMA_C7IFCR,
    #[doc = "0x208 - MDMA channel 7 error status register"]
    pub mdma_c7esr: MDMA_C7ESR,
    #[doc = "0x20c - This register is used to control the concerned channel."]
    pub mdma_c7cr: MDMA_C7CR,
    #[doc = "0x210 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c7tcr: MDMA_C7TCR,
    #[doc = "0x214 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c7bndtr: MDMA_C7BNDTR,
    #[doc = "0x218 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c7sar: MDMA_C7SAR,
    #[doc = "0x21c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c7dar: MDMA_C7DAR,
    #[doc = "0x220 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c7brur: MDMA_C7BRUR,
    #[doc = "0x224 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c7lar: MDMA_C7LAR,
    #[doc = "0x228 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c7tbr: MDMA_C7TBR,
    _reserved104: [u8; 0x04],
    #[doc = "0x230 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c7mar: MDMA_C7MAR,
    #[doc = "0x234 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c7mdr: MDMA_C7MDR,
    _reserved106: [u8; 0x08],
    #[doc = "0x240 - MDMA channel 8 interrupt/status register"]
    pub mdma_c8isr: MDMA_C8ISR,
    #[doc = "0x244 - MDMA channel 8 interrupt flag clear register"]
    pub mdma_c8ifcr: MDMA_C8IFCR,
    #[doc = "0x248 - MDMA channel 8 error status register"]
    pub mdma_c8esr: MDMA_C8ESR,
    #[doc = "0x24c - This register is used to control the concerned channel."]
    pub mdma_c8cr: MDMA_C8CR,
    #[doc = "0x250 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c8tcr: MDMA_C8TCR,
    #[doc = "0x254 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c8bndtr: MDMA_C8BNDTR,
    #[doc = "0x258 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c8sar: MDMA_C8SAR,
    #[doc = "0x25c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c8dar: MDMA_C8DAR,
    #[doc = "0x260 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c8brur: MDMA_C8BRUR,
    #[doc = "0x264 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c8lar: MDMA_C8LAR,
    #[doc = "0x268 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c8tbr: MDMA_C8TBR,
    _reserved117: [u8; 0x04],
    #[doc = "0x270 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c8mar: MDMA_C8MAR,
    #[doc = "0x274 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c8mdr: MDMA_C8MDR,
    _reserved119: [u8; 0x08],
    #[doc = "0x280 - MDMA channel 9 interrupt/status register"]
    pub mdma_c9isr: MDMA_C9ISR,
    #[doc = "0x284 - MDMA channel 9 interrupt flag clear register"]
    pub mdma_c9ifcr: MDMA_C9IFCR,
    #[doc = "0x288 - MDMA channel 9 error status register"]
    pub mdma_c9esr: MDMA_C9ESR,
    #[doc = "0x28c - This register is used to control the concerned channel."]
    pub mdma_c9cr: MDMA_C9CR,
    #[doc = "0x290 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c9tcr: MDMA_C9TCR,
    #[doc = "0x294 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c9bndtr: MDMA_C9BNDTR,
    #[doc = "0x298 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c9sar: MDMA_C9SAR,
    #[doc = "0x29c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c9dar: MDMA_C9DAR,
    #[doc = "0x2a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c9brur: MDMA_C9BRUR,
    #[doc = "0x2a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c9lar: MDMA_C9LAR,
    #[doc = "0x2a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c9tbr: MDMA_C9TBR,
    _reserved130: [u8; 0x04],
    #[doc = "0x2b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c9mar: MDMA_C9MAR,
    #[doc = "0x2b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c9mdr: MDMA_C9MDR,
    _reserved132: [u8; 0x08],
    #[doc = "0x2c0 - MDMA channel 10 interrupt/status register"]
    pub mdma_c10isr: MDMA_C10ISR,
    #[doc = "0x2c4 - MDMA channel 10 interrupt flag clear register"]
    pub mdma_c10ifcr: MDMA_C10IFCR,
    #[doc = "0x2c8 - MDMA channel 10 error status register"]
    pub mdma_c10esr: MDMA_C10ESR,
    #[doc = "0x2cc - This register is used to control the concerned channel."]
    pub mdma_c10cr: MDMA_C10CR,
    #[doc = "0x2d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c10tcr: MDMA_C10TCR,
    #[doc = "0x2d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c10bndtr: MDMA_C10BNDTR,
    #[doc = "0x2d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c10sar: MDMA_C10SAR,
    #[doc = "0x2dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c10dar: MDMA_C10DAR,
    #[doc = "0x2e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c10brur: MDMA_C10BRUR,
    #[doc = "0x2e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c10lar: MDMA_C10LAR,
    #[doc = "0x2e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c10tbr: MDMA_C10TBR,
    _reserved143: [u8; 0x04],
    #[doc = "0x2f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c10mar: MDMA_C10MAR,
    #[doc = "0x2f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c10mdr: MDMA_C10MDR,
    _reserved145: [u8; 0x08],
    #[doc = "0x300 - MDMA channel 11 interrupt/status register"]
    pub mdma_c11isr: MDMA_C11ISR,
    #[doc = "0x304 - MDMA channel 11 interrupt flag clear register"]
    pub mdma_c11ifcr: MDMA_C11IFCR,
    #[doc = "0x308 - MDMA channel 11 error status register"]
    pub mdma_c11esr: MDMA_C11ESR,
    #[doc = "0x30c - This register is used to control the concerned channel."]
    pub mdma_c11cr: MDMA_C11CR,
    #[doc = "0x310 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c11tcr: MDMA_C11TCR,
    #[doc = "0x314 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c11bndtr: MDMA_C11BNDTR,
    #[doc = "0x318 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c11sar: MDMA_C11SAR,
    #[doc = "0x31c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c11dar: MDMA_C11DAR,
    #[doc = "0x320 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c11brur: MDMA_C11BRUR,
    #[doc = "0x324 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c11lar: MDMA_C11LAR,
    #[doc = "0x328 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c11tbr: MDMA_C11TBR,
    _reserved156: [u8; 0x04],
    #[doc = "0x330 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c11mar: MDMA_C11MAR,
    #[doc = "0x334 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c11mdr: MDMA_C11MDR,
    _reserved158: [u8; 0x08],
    #[doc = "0x340 - MDMA channel 12 interrupt/status register"]
    pub mdma_c12isr: MDMA_C12ISR,
    #[doc = "0x344 - MDMA channel 12 interrupt flag clear register"]
    pub mdma_c12ifcr: MDMA_C12IFCR,
    #[doc = "0x348 - MDMA channel 12 error status register"]
    pub mdma_c12esr: MDMA_C12ESR,
    #[doc = "0x34c - This register is used to control the concerned channel."]
    pub mdma_c12cr: MDMA_C12CR,
    #[doc = "0x350 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c12tcr: MDMA_C12TCR,
    #[doc = "0x354 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c12bndtr: MDMA_C12BNDTR,
    #[doc = "0x358 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c12sar: MDMA_C12SAR,
    #[doc = "0x35c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c12dar: MDMA_C12DAR,
    #[doc = "0x360 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c12brur: MDMA_C12BRUR,
    #[doc = "0x364 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c12lar: MDMA_C12LAR,
    #[doc = "0x368 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c12tbr: MDMA_C12TBR,
    _reserved169: [u8; 0x04],
    #[doc = "0x370 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c12mar: MDMA_C12MAR,
    #[doc = "0x374 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c12mdr: MDMA_C12MDR,
    _reserved171: [u8; 0x08],
    #[doc = "0x380 - MDMA channel 13 interrupt/status register"]
    pub mdma_c13isr: MDMA_C13ISR,
    #[doc = "0x384 - MDMA channel 13 interrupt flag clear register"]
    pub mdma_c13ifcr: MDMA_C13IFCR,
    #[doc = "0x388 - MDMA channel 13 error status register"]
    pub mdma_c13esr: MDMA_C13ESR,
    #[doc = "0x38c - This register is used to control the concerned channel."]
    pub mdma_c13cr: MDMA_C13CR,
    #[doc = "0x390 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c13tcr: MDMA_C13TCR,
    #[doc = "0x394 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c13bndtr: MDMA_C13BNDTR,
    #[doc = "0x398 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c13sar: MDMA_C13SAR,
    #[doc = "0x39c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c13dar: MDMA_C13DAR,
    #[doc = "0x3a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c13brur: MDMA_C13BRUR,
    #[doc = "0x3a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c13lar: MDMA_C13LAR,
    #[doc = "0x3a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c13tbr: MDMA_C13TBR,
    _reserved182: [u8; 0x04],
    #[doc = "0x3b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c13mar: MDMA_C13MAR,
    #[doc = "0x3b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c13mdr: MDMA_C13MDR,
    _reserved184: [u8; 0x08],
    #[doc = "0x3c0 - MDMA channel 14 interrupt/status register"]
    pub mdma_c14isr: MDMA_C14ISR,
    #[doc = "0x3c4 - MDMA channel 14 interrupt flag clear register"]
    pub mdma_c14ifcr: MDMA_C14IFCR,
    #[doc = "0x3c8 - MDMA channel 14 error status register"]
    pub mdma_c14esr: MDMA_C14ESR,
    #[doc = "0x3cc - This register is used to control the concerned channel."]
    pub mdma_c14cr: MDMA_C14CR,
    #[doc = "0x3d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c14tcr: MDMA_C14TCR,
    #[doc = "0x3d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c14bndtr: MDMA_C14BNDTR,
    #[doc = "0x3d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c14sar: MDMA_C14SAR,
    #[doc = "0x3dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c14dar: MDMA_C14DAR,
    #[doc = "0x3e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c14brur: MDMA_C14BRUR,
    #[doc = "0x3e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c14lar: MDMA_C14LAR,
    #[doc = "0x3e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c14tbr: MDMA_C14TBR,
    _reserved195: [u8; 0x04],
    #[doc = "0x3f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c14mar: MDMA_C14MAR,
    #[doc = "0x3f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c14mdr: MDMA_C14MDR,
    _reserved197: [u8; 0x08],
    #[doc = "0x400 - MDMA channel 15 interrupt/status register"]
    pub mdma_c15isr: MDMA_C15ISR,
    #[doc = "0x404 - MDMA channel 15 interrupt flag clear register"]
    pub mdma_c15ifcr: MDMA_C15IFCR,
    #[doc = "0x408 - MDMA channel 15 error status register"]
    pub mdma_c15esr: MDMA_C15ESR,
    #[doc = "0x40c - This register is used to control the concerned channel."]
    pub mdma_c15cr: MDMA_C15CR,
    #[doc = "0x410 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c15tcr: MDMA_C15TCR,
    #[doc = "0x414 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c15bndtr: MDMA_C15BNDTR,
    #[doc = "0x418 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c15sar: MDMA_C15SAR,
    #[doc = "0x41c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c15dar: MDMA_C15DAR,
    #[doc = "0x420 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c15brur: MDMA_C15BRUR,
    #[doc = "0x424 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c15lar: MDMA_C15LAR,
    #[doc = "0x428 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c15tbr: MDMA_C15TBR,
    _reserved208: [u8; 0x04],
    #[doc = "0x430 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c15mar: MDMA_C15MAR,
    #[doc = "0x434 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c15mdr: MDMA_C15MDR,
    _reserved210: [u8; 0x08],
    #[doc = "0x440 - MDMA channel 16 interrupt/status register"]
    pub mdma_c16isr: MDMA_C16ISR,
    #[doc = "0x444 - MDMA channel 16 interrupt flag clear register"]
    pub mdma_c16ifcr: MDMA_C16IFCR,
    #[doc = "0x448 - MDMA channel 16 error status register"]
    pub mdma_c16esr: MDMA_C16ESR,
    #[doc = "0x44c - This register is used to control the concerned channel."]
    pub mdma_c16cr: MDMA_C16CR,
    #[doc = "0x450 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c16tcr: MDMA_C16TCR,
    #[doc = "0x454 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c16bndtr: MDMA_C16BNDTR,
    #[doc = "0x458 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c16sar: MDMA_C16SAR,
    #[doc = "0x45c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c16dar: MDMA_C16DAR,
    #[doc = "0x460 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c16brur: MDMA_C16BRUR,
    #[doc = "0x464 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c16lar: MDMA_C16LAR,
    #[doc = "0x468 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c16tbr: MDMA_C16TBR,
    _reserved221: [u8; 0x04],
    #[doc = "0x470 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c16mar: MDMA_C16MAR,
    #[doc = "0x474 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c16mdr: MDMA_C16MDR,
    _reserved223: [u8; 0x08],
    #[doc = "0x480 - MDMA channel 17 interrupt/status register"]
    pub mdma_c17isr: MDMA_C17ISR,
    #[doc = "0x484 - MDMA channel 17 interrupt flag clear register"]
    pub mdma_c17ifcr: MDMA_C17IFCR,
    #[doc = "0x488 - MDMA channel 17 error status register"]
    pub mdma_c17esr: MDMA_C17ESR,
    #[doc = "0x48c - This register is used to control the concerned channel."]
    pub mdma_c17cr: MDMA_C17CR,
    #[doc = "0x490 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c17tcr: MDMA_C17TCR,
    #[doc = "0x494 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c17bndtr: MDMA_C17BNDTR,
    #[doc = "0x498 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c17sar: MDMA_C17SAR,
    #[doc = "0x49c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c17dar: MDMA_C17DAR,
    #[doc = "0x4a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c17brur: MDMA_C17BRUR,
    #[doc = "0x4a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c17lar: MDMA_C17LAR,
    #[doc = "0x4a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c17tbr: MDMA_C17TBR,
    _reserved234: [u8; 0x04],
    #[doc = "0x4b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c17mar: MDMA_C17MAR,
    #[doc = "0x4b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c17mdr: MDMA_C17MDR,
    _reserved236: [u8; 0x08],
    #[doc = "0x4c0 - MDMA channel 18 interrupt/status register"]
    pub mdma_c18isr: MDMA_C18ISR,
    #[doc = "0x4c4 - MDMA channel 18 interrupt flag clear register"]
    pub mdma_c18ifcr: MDMA_C18IFCR,
    #[doc = "0x4c8 - MDMA channel 18 error status register"]
    pub mdma_c18esr: MDMA_C18ESR,
    #[doc = "0x4cc - This register is used to control the concerned channel."]
    pub mdma_c18cr: MDMA_C18CR,
    #[doc = "0x4d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c18tcr: MDMA_C18TCR,
    #[doc = "0x4d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c18bndtr: MDMA_C18BNDTR,
    #[doc = "0x4d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c18sar: MDMA_C18SAR,
    #[doc = "0x4dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c18dar: MDMA_C18DAR,
    #[doc = "0x4e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c18brur: MDMA_C18BRUR,
    #[doc = "0x4e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c18lar: MDMA_C18LAR,
    #[doc = "0x4e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c18tbr: MDMA_C18TBR,
    _reserved247: [u8; 0x04],
    #[doc = "0x4f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c18mar: MDMA_C18MAR,
    #[doc = "0x4f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c18mdr: MDMA_C18MDR,
    _reserved249: [u8; 0x08],
    #[doc = "0x500 - MDMA channel 19 interrupt/status register"]
    pub mdma_c19isr: MDMA_C19ISR,
    #[doc = "0x504 - MDMA channel 19 interrupt flag clear register"]
    pub mdma_c19ifcr: MDMA_C19IFCR,
    #[doc = "0x508 - MDMA channel 19 error status register"]
    pub mdma_c19esr: MDMA_C19ESR,
    #[doc = "0x50c - This register is used to control the concerned channel."]
    pub mdma_c19cr: MDMA_C19CR,
    #[doc = "0x510 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c19tcr: MDMA_C19TCR,
    #[doc = "0x514 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c19bndtr: MDMA_C19BNDTR,
    #[doc = "0x518 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c19sar: MDMA_C19SAR,
    #[doc = "0x51c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c19dar: MDMA_C19DAR,
    #[doc = "0x520 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c19brur: MDMA_C19BRUR,
    #[doc = "0x524 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c19lar: MDMA_C19LAR,
    #[doc = "0x528 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c19tbr: MDMA_C19TBR,
    _reserved260: [u8; 0x04],
    #[doc = "0x530 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c19mar: MDMA_C19MAR,
    #[doc = "0x534 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c19mdr: MDMA_C19MDR,
    _reserved262: [u8; 0x08],
    #[doc = "0x540 - MDMA channel 20 interrupt/status register"]
    pub mdma_c20isr: MDMA_C20ISR,
    #[doc = "0x544 - MDMA channel 20 interrupt flag clear register"]
    pub mdma_c20ifcr: MDMA_C20IFCR,
    #[doc = "0x548 - MDMA channel 20 error status register"]
    pub mdma_c20esr: MDMA_C20ESR,
    #[doc = "0x54c - This register is used to control the concerned channel."]
    pub mdma_c20cr: MDMA_C20CR,
    #[doc = "0x550 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c20tcr: MDMA_C20TCR,
    #[doc = "0x554 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c20bndtr: MDMA_C20BNDTR,
    #[doc = "0x558 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c20sar: MDMA_C20SAR,
    #[doc = "0x55c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c20dar: MDMA_C20DAR,
    #[doc = "0x560 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c20brur: MDMA_C20BRUR,
    #[doc = "0x564 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c20lar: MDMA_C20LAR,
    #[doc = "0x568 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c20tbr: MDMA_C20TBR,
    _reserved273: [u8; 0x04],
    #[doc = "0x570 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c20mar: MDMA_C20MAR,
    #[doc = "0x574 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c20mdr: MDMA_C20MDR,
    _reserved275: [u8; 0x08],
    #[doc = "0x580 - MDMA channel 21 interrupt/status register"]
    pub mdma_c21isr: MDMA_C21ISR,
    #[doc = "0x584 - MDMA channel 21 interrupt flag clear register"]
    pub mdma_c21ifcr: MDMA_C21IFCR,
    #[doc = "0x588 - MDMA channel 21 error status register"]
    pub mdma_c21esr: MDMA_C21ESR,
    #[doc = "0x58c - This register is used to control the concerned channel."]
    pub mdma_c21cr: MDMA_C21CR,
    #[doc = "0x590 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c21tcr: MDMA_C21TCR,
    #[doc = "0x594 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c21bndtr: MDMA_C21BNDTR,
    #[doc = "0x598 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c21sar: MDMA_C21SAR,
    #[doc = "0x59c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c21dar: MDMA_C21DAR,
    #[doc = "0x5a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c21brur: MDMA_C21BRUR,
    #[doc = "0x5a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c21lar: MDMA_C21LAR,
    #[doc = "0x5a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c21tbr: MDMA_C21TBR,
    _reserved286: [u8; 0x04],
    #[doc = "0x5b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c21mar: MDMA_C21MAR,
    #[doc = "0x5b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c21mdr: MDMA_C21MDR,
    _reserved288: [u8; 0x08],
    #[doc = "0x5c0 - MDMA channel 22 interrupt/status register"]
    pub mdma_c22isr: MDMA_C22ISR,
    #[doc = "0x5c4 - MDMA channel 22 interrupt flag clear register"]
    pub mdma_c22ifcr: MDMA_C22IFCR,
    #[doc = "0x5c8 - MDMA channel 22 error status register"]
    pub mdma_c22esr: MDMA_C22ESR,
    #[doc = "0x5cc - This register is used to control the concerned channel."]
    pub mdma_c22cr: MDMA_C22CR,
    #[doc = "0x5d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c22tcr: MDMA_C22TCR,
    #[doc = "0x5d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c22bndtr: MDMA_C22BNDTR,
    #[doc = "0x5d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c22sar: MDMA_C22SAR,
    #[doc = "0x5dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c22dar: MDMA_C22DAR,
    #[doc = "0x5e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c22brur: MDMA_C22BRUR,
    #[doc = "0x5e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c22lar: MDMA_C22LAR,
    #[doc = "0x5e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c22tbr: MDMA_C22TBR,
    _reserved299: [u8; 0x04],
    #[doc = "0x5f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c22mar: MDMA_C22MAR,
    #[doc = "0x5f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c22mdr: MDMA_C22MDR,
    _reserved301: [u8; 0x08],
    #[doc = "0x600 - MDMA channel 23 interrupt/status register"]
    pub mdma_c23isr: MDMA_C23ISR,
    #[doc = "0x604 - MDMA channel 23 interrupt flag clear register"]
    pub mdma_c23ifcr: MDMA_C23IFCR,
    #[doc = "0x608 - MDMA channel 23 error status register"]
    pub mdma_c23esr: MDMA_C23ESR,
    #[doc = "0x60c - This register is used to control the concerned channel."]
    pub mdma_c23cr: MDMA_C23CR,
    #[doc = "0x610 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c23tcr: MDMA_C23TCR,
    #[doc = "0x614 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c23bndtr: MDMA_C23BNDTR,
    #[doc = "0x618 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c23sar: MDMA_C23SAR,
    #[doc = "0x61c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c23dar: MDMA_C23DAR,
    #[doc = "0x620 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c23brur: MDMA_C23BRUR,
    #[doc = "0x624 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c23lar: MDMA_C23LAR,
    #[doc = "0x628 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c23tbr: MDMA_C23TBR,
    _reserved312: [u8; 0x04],
    #[doc = "0x630 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c23mar: MDMA_C23MAR,
    #[doc = "0x634 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c23mdr: MDMA_C23MDR,
    _reserved314: [u8; 0x08],
    #[doc = "0x640 - MDMA channel 24 interrupt/status register"]
    pub mdma_c24isr: MDMA_C24ISR,
    #[doc = "0x644 - MDMA channel 24 interrupt flag clear register"]
    pub mdma_c24ifcr: MDMA_C24IFCR,
    #[doc = "0x648 - MDMA channel 24 error status register"]
    pub mdma_c24esr: MDMA_C24ESR,
    #[doc = "0x64c - This register is used to control the concerned channel."]
    pub mdma_c24cr: MDMA_C24CR,
    #[doc = "0x650 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c24tcr: MDMA_C24TCR,
    #[doc = "0x654 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c24bndtr: MDMA_C24BNDTR,
    #[doc = "0x658 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c24sar: MDMA_C24SAR,
    #[doc = "0x65c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c24dar: MDMA_C24DAR,
    #[doc = "0x660 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c24brur: MDMA_C24BRUR,
    #[doc = "0x664 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c24lar: MDMA_C24LAR,
    #[doc = "0x668 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c24tbr: MDMA_C24TBR,
    _reserved325: [u8; 0x04],
    #[doc = "0x670 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c24mar: MDMA_C24MAR,
    #[doc = "0x674 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c24mdr: MDMA_C24MDR,
    _reserved327: [u8; 0x08],
    #[doc = "0x680 - MDMA channel 25 interrupt/status register"]
    pub mdma_c25isr: MDMA_C25ISR,
    #[doc = "0x684 - MDMA channel 25 interrupt flag clear register"]
    pub mdma_c25ifcr: MDMA_C25IFCR,
    #[doc = "0x688 - MDMA channel 25 error status register"]
    pub mdma_c25esr: MDMA_C25ESR,
    #[doc = "0x68c - This register is used to control the concerned channel."]
    pub mdma_c25cr: MDMA_C25CR,
    #[doc = "0x690 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c25tcr: MDMA_C25TCR,
    #[doc = "0x694 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c25bndtr: MDMA_C25BNDTR,
    #[doc = "0x698 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c25sar: MDMA_C25SAR,
    #[doc = "0x69c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c25dar: MDMA_C25DAR,
    #[doc = "0x6a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c25brur: MDMA_C25BRUR,
    #[doc = "0x6a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c25lar: MDMA_C25LAR,
    #[doc = "0x6a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c25tbr: MDMA_C25TBR,
    _reserved338: [u8; 0x04],
    #[doc = "0x6b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c25mar: MDMA_C25MAR,
    #[doc = "0x6b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c25mdr: MDMA_C25MDR,
    _reserved340: [u8; 0x08],
    #[doc = "0x6c0 - MDMA channel 26 interrupt/status register"]
    pub mdma_c26isr: MDMA_C26ISR,
    #[doc = "0x6c4 - MDMA channel 26 interrupt flag clear register"]
    pub mdma_c26ifcr: MDMA_C26IFCR,
    #[doc = "0x6c8 - MDMA channel 26 error status register"]
    pub mdma_c26esr: MDMA_C26ESR,
    #[doc = "0x6cc - This register is used to control the concerned channel."]
    pub mdma_c26cr: MDMA_C26CR,
    #[doc = "0x6d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c26tcr: MDMA_C26TCR,
    #[doc = "0x6d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c26bndtr: MDMA_C26BNDTR,
    #[doc = "0x6d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c26sar: MDMA_C26SAR,
    #[doc = "0x6dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c26dar: MDMA_C26DAR,
    #[doc = "0x6e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c26brur: MDMA_C26BRUR,
    #[doc = "0x6e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c26lar: MDMA_C26LAR,
    #[doc = "0x6e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c26tbr: MDMA_C26TBR,
    _reserved351: [u8; 0x04],
    #[doc = "0x6f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c26mar: MDMA_C26MAR,
    #[doc = "0x6f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c26mdr: MDMA_C26MDR,
    _reserved353: [u8; 0x08],
    #[doc = "0x700 - MDMA channel 27 interrupt/status register"]
    pub mdma_c27isr: MDMA_C27ISR,
    #[doc = "0x704 - MDMA channel 27 interrupt flag clear register"]
    pub mdma_c27ifcr: MDMA_C27IFCR,
    #[doc = "0x708 - MDMA channel 27 error status register"]
    pub mdma_c27esr: MDMA_C27ESR,
    #[doc = "0x70c - This register is used to control the concerned channel."]
    pub mdma_c27cr: MDMA_C27CR,
    #[doc = "0x710 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c27tcr: MDMA_C27TCR,
    #[doc = "0x714 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c27bndtr: MDMA_C27BNDTR,
    #[doc = "0x718 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c27sar: MDMA_C27SAR,
    #[doc = "0x71c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c27dar: MDMA_C27DAR,
    #[doc = "0x720 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c27brur: MDMA_C27BRUR,
    #[doc = "0x724 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c27lar: MDMA_C27LAR,
    #[doc = "0x728 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c27tbr: MDMA_C27TBR,
    _reserved364: [u8; 0x04],
    #[doc = "0x730 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c27mar: MDMA_C27MAR,
    #[doc = "0x734 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c27mdr: MDMA_C27MDR,
    _reserved366: [u8; 0x08],
    #[doc = "0x740 - MDMA channel 28 interrupt/status register"]
    pub mdma_c28isr: MDMA_C28ISR,
    #[doc = "0x744 - MDMA channel 28 interrupt flag clear register"]
    pub mdma_c28ifcr: MDMA_C28IFCR,
    #[doc = "0x748 - MDMA channel 28 error status register"]
    pub mdma_c28esr: MDMA_C28ESR,
    #[doc = "0x74c - This register is used to control the concerned channel."]
    pub mdma_c28cr: MDMA_C28CR,
    #[doc = "0x750 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c28tcr: MDMA_C28TCR,
    #[doc = "0x754 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c28bndtr: MDMA_C28BNDTR,
    #[doc = "0x758 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c28sar: MDMA_C28SAR,
    #[doc = "0x75c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c28dar: MDMA_C28DAR,
    #[doc = "0x760 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c28brur: MDMA_C28BRUR,
    #[doc = "0x764 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c28lar: MDMA_C28LAR,
    #[doc = "0x768 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c28tbr: MDMA_C28TBR,
    _reserved377: [u8; 0x04],
    #[doc = "0x770 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c28mar: MDMA_C28MAR,
    #[doc = "0x774 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c28mdr: MDMA_C28MDR,
    _reserved379: [u8; 0x08],
    #[doc = "0x780 - MDMA channel 29 interrupt/status register"]
    pub mdma_c29isr: MDMA_C29ISR,
    #[doc = "0x784 - MDMA channel 29 interrupt flag clear register"]
    pub mdma_c29ifcr: MDMA_C29IFCR,
    #[doc = "0x788 - MDMA channel 29 error status register"]
    pub mdma_c29esr: MDMA_C29ESR,
    #[doc = "0x78c - This register is used to control the concerned channel."]
    pub mdma_c29cr: MDMA_C29CR,
    #[doc = "0x790 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c29tcr: MDMA_C29TCR,
    #[doc = "0x794 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c29bndtr: MDMA_C29BNDTR,
    #[doc = "0x798 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c29sar: MDMA_C29SAR,
    #[doc = "0x79c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c29dar: MDMA_C29DAR,
    #[doc = "0x7a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c29brur: MDMA_C29BRUR,
    #[doc = "0x7a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c29lar: MDMA_C29LAR,
    #[doc = "0x7a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c29tbr: MDMA_C29TBR,
    _reserved390: [u8; 0x04],
    #[doc = "0x7b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c29mar: MDMA_C29MAR,
    #[doc = "0x7b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c29mdr: MDMA_C29MDR,
    _reserved392: [u8; 0x08],
    #[doc = "0x7c0 - MDMA channel 30 interrupt/status register"]
    pub mdma_c30isr: MDMA_C30ISR,
    #[doc = "0x7c4 - MDMA channel 30 interrupt flag clear register"]
    pub mdma_c30ifcr: MDMA_C30IFCR,
    #[doc = "0x7c8 - MDMA channel 30 error status register"]
    pub mdma_c30esr: MDMA_C30ESR,
    #[doc = "0x7cc - This register is used to control the concerned channel."]
    pub mdma_c30cr: MDMA_C30CR,
    #[doc = "0x7d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c30tcr: MDMA_C30TCR,
    #[doc = "0x7d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c30bndtr: MDMA_C30BNDTR,
    #[doc = "0x7d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c30sar: MDMA_C30SAR,
    #[doc = "0x7dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c30dar: MDMA_C30DAR,
    #[doc = "0x7e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c30brur: MDMA_C30BRUR,
    #[doc = "0x7e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c30lar: MDMA_C30LAR,
    #[doc = "0x7e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c30tbr: MDMA_C30TBR,
    _reserved403: [u8; 0x04],
    #[doc = "0x7f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c30mar: MDMA_C30MAR,
    #[doc = "0x7f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c30mdr: MDMA_C30MDR,
    _reserved405: [u8; 0x08],
    #[doc = "0x800 - MDMA channel 31 interrupt/status register"]
    pub mdma_c31isr: MDMA_C31ISR,
    #[doc = "0x804 - MDMA channel 31 interrupt flag clear register"]
    pub mdma_c31ifcr: MDMA_C31IFCR,
    #[doc = "0x808 - MDMA channel 31 error status register"]
    pub mdma_c31esr: MDMA_C31ESR,
    #[doc = "0x80c - This register is used to control the concerned channel."]
    pub mdma_c31cr: MDMA_C31CR,
    #[doc = "0x810 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    pub mdma_c31tcr: MDMA_C31TCR,
    #[doc = "0x814 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
    pub mdma_c31bndtr: MDMA_C31BNDTR,
    #[doc = "0x818 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
    pub mdma_c31sar: MDMA_C31SAR,
    #[doc = "0x81c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
    pub mdma_c31dar: MDMA_C31DAR,
    #[doc = "0x820 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
    pub mdma_c31brur: MDMA_C31BRUR,
    #[doc = "0x824 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
    pub mdma_c31lar: MDMA_C31LAR,
    #[doc = "0x828 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
    pub mdma_c31tbr: MDMA_C31TBR,
    _reserved416: [u8; 0x04],
    #[doc = "0x830 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
    pub mdma_c31mar: MDMA_C31MAR,
    #[doc = "0x834 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
    pub mdma_c31mdr: MDMA_C31MDR,
}
#[doc = "MDMA_GISR0 (r) register accessor: an alias for `Reg<MDMA_GISR0_SPEC>`"]
pub type MDMA_GISR0 = crate::Reg<mdma_gisr0::MDMA_GISR0_SPEC>;
#[doc = "MDMA global interrupt/status register"]
pub mod mdma_gisr0;
#[doc = "MDMA_SGISR0 (r) register accessor: an alias for `Reg<MDMA_SGISR0_SPEC>`"]
pub type MDMA_SGISR0 = crate::Reg<mdma_sgisr0::MDMA_SGISR0_SPEC>;
#[doc = "MDMA secure global interrupt/status register"]
pub mod mdma_sgisr0;
#[doc = "MDMA_C0ISR (r) register accessor: an alias for `Reg<MDMA_C0ISR_SPEC>`"]
pub type MDMA_C0ISR = crate::Reg<mdma_c0isr::MDMA_C0ISR_SPEC>;
#[doc = "MDMA channel 0 interrupt/status register"]
pub mod mdma_c0isr;
#[doc = "MDMA_C0IFCR (w) register accessor: an alias for `Reg<MDMA_C0IFCR_SPEC>`"]
pub type MDMA_C0IFCR = crate::Reg<mdma_c0ifcr::MDMA_C0IFCR_SPEC>;
#[doc = "MDMA channel 0 interrupt flag clear register"]
pub mod mdma_c0ifcr;
#[doc = "MDMA_C0ESR (r) register accessor: an alias for `Reg<MDMA_C0ESR_SPEC>`"]
pub type MDMA_C0ESR = crate::Reg<mdma_c0esr::MDMA_C0ESR_SPEC>;
#[doc = "MDMA channel 0 error status register"]
pub mod mdma_c0esr;
#[doc = "MDMA_C0CR (rw) register accessor: an alias for `Reg<MDMA_C0CR_SPEC>`"]
pub type MDMA_C0CR = crate::Reg<mdma_c0cr::MDMA_C0CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c0cr;
#[doc = "MDMA_C0TCR (rw) register accessor: an alias for `Reg<MDMA_C0TCR_SPEC>`"]
pub type MDMA_C0TCR = crate::Reg<mdma_c0tcr::MDMA_C0TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c0tcr;
#[doc = "MDMA_C0BNDTR (rw) register accessor: an alias for `Reg<MDMA_C0BNDTR_SPEC>`"]
pub type MDMA_C0BNDTR = crate::Reg<mdma_c0bndtr::MDMA_C0BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c0bndtr;
#[doc = "MDMA_C0SAR (rw) register accessor: an alias for `Reg<MDMA_C0SAR_SPEC>`"]
pub type MDMA_C0SAR = crate::Reg<mdma_c0sar::MDMA_C0SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c0sar;
#[doc = "MDMA_C0DAR (rw) register accessor: an alias for `Reg<MDMA_C0DAR_SPEC>`"]
pub type MDMA_C0DAR = crate::Reg<mdma_c0dar::MDMA_C0DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c0dar;
#[doc = "MDMA_C0BRUR (rw) register accessor: an alias for `Reg<MDMA_C0BRUR_SPEC>`"]
pub type MDMA_C0BRUR = crate::Reg<mdma_c0brur::MDMA_C0BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c0brur;
#[doc = "MDMA_C0LAR (rw) register accessor: an alias for `Reg<MDMA_C0LAR_SPEC>`"]
pub type MDMA_C0LAR = crate::Reg<mdma_c0lar::MDMA_C0LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c0lar;
#[doc = "MDMA_C0TBR (rw) register accessor: an alias for `Reg<MDMA_C0TBR_SPEC>`"]
pub type MDMA_C0TBR = crate::Reg<mdma_c0tbr::MDMA_C0TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c0tbr;
#[doc = "MDMA_C0MAR (rw) register accessor: an alias for `Reg<MDMA_C0MAR_SPEC>`"]
pub type MDMA_C0MAR = crate::Reg<mdma_c0mar::MDMA_C0MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c0mar;
#[doc = "MDMA_C0MDR (rw) register accessor: an alias for `Reg<MDMA_C0MDR_SPEC>`"]
pub type MDMA_C0MDR = crate::Reg<mdma_c0mdr::MDMA_C0MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c0mdr;
#[doc = "MDMA_C1ISR (r) register accessor: an alias for `Reg<MDMA_C1ISR_SPEC>`"]
pub type MDMA_C1ISR = crate::Reg<mdma_c1isr::MDMA_C1ISR_SPEC>;
#[doc = "MDMA channel 1 interrupt/status register"]
pub mod mdma_c1isr;
#[doc = "MDMA_C1IFCR (w) register accessor: an alias for `Reg<MDMA_C1IFCR_SPEC>`"]
pub type MDMA_C1IFCR = crate::Reg<mdma_c1ifcr::MDMA_C1IFCR_SPEC>;
#[doc = "MDMA channel 1 interrupt flag clear register"]
pub mod mdma_c1ifcr;
#[doc = "MDMA_C1ESR (r) register accessor: an alias for `Reg<MDMA_C1ESR_SPEC>`"]
pub type MDMA_C1ESR = crate::Reg<mdma_c1esr::MDMA_C1ESR_SPEC>;
#[doc = "MDMA channel 1 error status register"]
pub mod mdma_c1esr;
#[doc = "MDMA_C1CR (rw) register accessor: an alias for `Reg<MDMA_C1CR_SPEC>`"]
pub type MDMA_C1CR = crate::Reg<mdma_c1cr::MDMA_C1CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c1cr;
#[doc = "MDMA_C1TCR (rw) register accessor: an alias for `Reg<MDMA_C1TCR_SPEC>`"]
pub type MDMA_C1TCR = crate::Reg<mdma_c1tcr::MDMA_C1TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c1tcr;
#[doc = "MDMA_C1BNDTR (rw) register accessor: an alias for `Reg<MDMA_C1BNDTR_SPEC>`"]
pub type MDMA_C1BNDTR = crate::Reg<mdma_c1bndtr::MDMA_C1BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c1bndtr;
#[doc = "MDMA_C1SAR (rw) register accessor: an alias for `Reg<MDMA_C1SAR_SPEC>`"]
pub type MDMA_C1SAR = crate::Reg<mdma_c1sar::MDMA_C1SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c1sar;
#[doc = "MDMA_C1DAR (rw) register accessor: an alias for `Reg<MDMA_C1DAR_SPEC>`"]
pub type MDMA_C1DAR = crate::Reg<mdma_c1dar::MDMA_C1DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c1dar;
#[doc = "MDMA_C1BRUR (rw) register accessor: an alias for `Reg<MDMA_C1BRUR_SPEC>`"]
pub type MDMA_C1BRUR = crate::Reg<mdma_c1brur::MDMA_C1BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c1brur;
#[doc = "MDMA_C1LAR (rw) register accessor: an alias for `Reg<MDMA_C1LAR_SPEC>`"]
pub type MDMA_C1LAR = crate::Reg<mdma_c1lar::MDMA_C1LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c1lar;
#[doc = "MDMA_C1TBR (rw) register accessor: an alias for `Reg<MDMA_C1TBR_SPEC>`"]
pub type MDMA_C1TBR = crate::Reg<mdma_c1tbr::MDMA_C1TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c1tbr;
#[doc = "MDMA_C1MAR (rw) register accessor: an alias for `Reg<MDMA_C1MAR_SPEC>`"]
pub type MDMA_C1MAR = crate::Reg<mdma_c1mar::MDMA_C1MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c1mar;
#[doc = "MDMA_C1MDR (rw) register accessor: an alias for `Reg<MDMA_C1MDR_SPEC>`"]
pub type MDMA_C1MDR = crate::Reg<mdma_c1mdr::MDMA_C1MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c1mdr;
#[doc = "MDMA_C2ISR (r) register accessor: an alias for `Reg<MDMA_C2ISR_SPEC>`"]
pub type MDMA_C2ISR = crate::Reg<mdma_c2isr::MDMA_C2ISR_SPEC>;
#[doc = "MDMA channel 2 interrupt/status register"]
pub mod mdma_c2isr;
#[doc = "MDMA_C2IFCR (w) register accessor: an alias for `Reg<MDMA_C2IFCR_SPEC>`"]
pub type MDMA_C2IFCR = crate::Reg<mdma_c2ifcr::MDMA_C2IFCR_SPEC>;
#[doc = "MDMA channel 2 interrupt flag clear register"]
pub mod mdma_c2ifcr;
#[doc = "MDMA_C2ESR (r) register accessor: an alias for `Reg<MDMA_C2ESR_SPEC>`"]
pub type MDMA_C2ESR = crate::Reg<mdma_c2esr::MDMA_C2ESR_SPEC>;
#[doc = "MDMA channel 2 error status register"]
pub mod mdma_c2esr;
#[doc = "MDMA_C2CR (rw) register accessor: an alias for `Reg<MDMA_C2CR_SPEC>`"]
pub type MDMA_C2CR = crate::Reg<mdma_c2cr::MDMA_C2CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c2cr;
#[doc = "MDMA_C2TCR (rw) register accessor: an alias for `Reg<MDMA_C2TCR_SPEC>`"]
pub type MDMA_C2TCR = crate::Reg<mdma_c2tcr::MDMA_C2TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c2tcr;
#[doc = "MDMA_C2BNDTR (rw) register accessor: an alias for `Reg<MDMA_C2BNDTR_SPEC>`"]
pub type MDMA_C2BNDTR = crate::Reg<mdma_c2bndtr::MDMA_C2BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c2bndtr;
#[doc = "MDMA_C2SAR (rw) register accessor: an alias for `Reg<MDMA_C2SAR_SPEC>`"]
pub type MDMA_C2SAR = crate::Reg<mdma_c2sar::MDMA_C2SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c2sar;
#[doc = "MDMA_C2DAR (rw) register accessor: an alias for `Reg<MDMA_C2DAR_SPEC>`"]
pub type MDMA_C2DAR = crate::Reg<mdma_c2dar::MDMA_C2DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c2dar;
#[doc = "MDMA_C2BRUR (rw) register accessor: an alias for `Reg<MDMA_C2BRUR_SPEC>`"]
pub type MDMA_C2BRUR = crate::Reg<mdma_c2brur::MDMA_C2BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c2brur;
#[doc = "MDMA_C2LAR (rw) register accessor: an alias for `Reg<MDMA_C2LAR_SPEC>`"]
pub type MDMA_C2LAR = crate::Reg<mdma_c2lar::MDMA_C2LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c2lar;
#[doc = "MDMA_C2TBR (rw) register accessor: an alias for `Reg<MDMA_C2TBR_SPEC>`"]
pub type MDMA_C2TBR = crate::Reg<mdma_c2tbr::MDMA_C2TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c2tbr;
#[doc = "MDMA_C2MAR (rw) register accessor: an alias for `Reg<MDMA_C2MAR_SPEC>`"]
pub type MDMA_C2MAR = crate::Reg<mdma_c2mar::MDMA_C2MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c2mar;
#[doc = "MDMA_C2MDR (rw) register accessor: an alias for `Reg<MDMA_C2MDR_SPEC>`"]
pub type MDMA_C2MDR = crate::Reg<mdma_c2mdr::MDMA_C2MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c2mdr;
#[doc = "MDMA_C3ISR (r) register accessor: an alias for `Reg<MDMA_C3ISR_SPEC>`"]
pub type MDMA_C3ISR = crate::Reg<mdma_c3isr::MDMA_C3ISR_SPEC>;
#[doc = "MDMA channel 3 interrupt/status register"]
pub mod mdma_c3isr;
#[doc = "MDMA_C3IFCR (w) register accessor: an alias for `Reg<MDMA_C3IFCR_SPEC>`"]
pub type MDMA_C3IFCR = crate::Reg<mdma_c3ifcr::MDMA_C3IFCR_SPEC>;
#[doc = "MDMA channel 3 interrupt flag clear register"]
pub mod mdma_c3ifcr;
#[doc = "MDMA_C3ESR (r) register accessor: an alias for `Reg<MDMA_C3ESR_SPEC>`"]
pub type MDMA_C3ESR = crate::Reg<mdma_c3esr::MDMA_C3ESR_SPEC>;
#[doc = "MDMA channel 3 error status register"]
pub mod mdma_c3esr;
#[doc = "MDMA_C3CR (rw) register accessor: an alias for `Reg<MDMA_C3CR_SPEC>`"]
pub type MDMA_C3CR = crate::Reg<mdma_c3cr::MDMA_C3CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c3cr;
#[doc = "MDMA_C3TCR (rw) register accessor: an alias for `Reg<MDMA_C3TCR_SPEC>`"]
pub type MDMA_C3TCR = crate::Reg<mdma_c3tcr::MDMA_C3TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c3tcr;
#[doc = "MDMA_C3BNDTR (rw) register accessor: an alias for `Reg<MDMA_C3BNDTR_SPEC>`"]
pub type MDMA_C3BNDTR = crate::Reg<mdma_c3bndtr::MDMA_C3BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c3bndtr;
#[doc = "MDMA_C3SAR (rw) register accessor: an alias for `Reg<MDMA_C3SAR_SPEC>`"]
pub type MDMA_C3SAR = crate::Reg<mdma_c3sar::MDMA_C3SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c3sar;
#[doc = "MDMA_C3DAR (rw) register accessor: an alias for `Reg<MDMA_C3DAR_SPEC>`"]
pub type MDMA_C3DAR = crate::Reg<mdma_c3dar::MDMA_C3DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c3dar;
#[doc = "MDMA_C3BRUR (rw) register accessor: an alias for `Reg<MDMA_C3BRUR_SPEC>`"]
pub type MDMA_C3BRUR = crate::Reg<mdma_c3brur::MDMA_C3BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c3brur;
#[doc = "MDMA_C3LAR (rw) register accessor: an alias for `Reg<MDMA_C3LAR_SPEC>`"]
pub type MDMA_C3LAR = crate::Reg<mdma_c3lar::MDMA_C3LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c3lar;
#[doc = "MDMA_C3TBR (rw) register accessor: an alias for `Reg<MDMA_C3TBR_SPEC>`"]
pub type MDMA_C3TBR = crate::Reg<mdma_c3tbr::MDMA_C3TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c3tbr;
#[doc = "MDMA_C3MAR (rw) register accessor: an alias for `Reg<MDMA_C3MAR_SPEC>`"]
pub type MDMA_C3MAR = crate::Reg<mdma_c3mar::MDMA_C3MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c3mar;
#[doc = "MDMA_C3MDR (rw) register accessor: an alias for `Reg<MDMA_C3MDR_SPEC>`"]
pub type MDMA_C3MDR = crate::Reg<mdma_c3mdr::MDMA_C3MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c3mdr;
#[doc = "MDMA_C4ISR (r) register accessor: an alias for `Reg<MDMA_C4ISR_SPEC>`"]
pub type MDMA_C4ISR = crate::Reg<mdma_c4isr::MDMA_C4ISR_SPEC>;
#[doc = "MDMA channel 4 interrupt/status register"]
pub mod mdma_c4isr;
#[doc = "MDMA_C4IFCR (w) register accessor: an alias for `Reg<MDMA_C4IFCR_SPEC>`"]
pub type MDMA_C4IFCR = crate::Reg<mdma_c4ifcr::MDMA_C4IFCR_SPEC>;
#[doc = "MDMA channel 4 interrupt flag clear register"]
pub mod mdma_c4ifcr;
#[doc = "MDMA_C4ESR (r) register accessor: an alias for `Reg<MDMA_C4ESR_SPEC>`"]
pub type MDMA_C4ESR = crate::Reg<mdma_c4esr::MDMA_C4ESR_SPEC>;
#[doc = "MDMA channel 4 error status register"]
pub mod mdma_c4esr;
#[doc = "MDMA_C4CR (rw) register accessor: an alias for `Reg<MDMA_C4CR_SPEC>`"]
pub type MDMA_C4CR = crate::Reg<mdma_c4cr::MDMA_C4CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c4cr;
#[doc = "MDMA_C4TCR (rw) register accessor: an alias for `Reg<MDMA_C4TCR_SPEC>`"]
pub type MDMA_C4TCR = crate::Reg<mdma_c4tcr::MDMA_C4TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c4tcr;
#[doc = "MDMA_C4BNDTR (rw) register accessor: an alias for `Reg<MDMA_C4BNDTR_SPEC>`"]
pub type MDMA_C4BNDTR = crate::Reg<mdma_c4bndtr::MDMA_C4BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c4bndtr;
#[doc = "MDMA_C4SAR (rw) register accessor: an alias for `Reg<MDMA_C4SAR_SPEC>`"]
pub type MDMA_C4SAR = crate::Reg<mdma_c4sar::MDMA_C4SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c4sar;
#[doc = "MDMA_C4DAR (rw) register accessor: an alias for `Reg<MDMA_C4DAR_SPEC>`"]
pub type MDMA_C4DAR = crate::Reg<mdma_c4dar::MDMA_C4DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c4dar;
#[doc = "MDMA_C4BRUR (rw) register accessor: an alias for `Reg<MDMA_C4BRUR_SPEC>`"]
pub type MDMA_C4BRUR = crate::Reg<mdma_c4brur::MDMA_C4BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c4brur;
#[doc = "MDMA_C4LAR (rw) register accessor: an alias for `Reg<MDMA_C4LAR_SPEC>`"]
pub type MDMA_C4LAR = crate::Reg<mdma_c4lar::MDMA_C4LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c4lar;
#[doc = "MDMA_C4TBR (rw) register accessor: an alias for `Reg<MDMA_C4TBR_SPEC>`"]
pub type MDMA_C4TBR = crate::Reg<mdma_c4tbr::MDMA_C4TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c4tbr;
#[doc = "MDMA_C4MAR (rw) register accessor: an alias for `Reg<MDMA_C4MAR_SPEC>`"]
pub type MDMA_C4MAR = crate::Reg<mdma_c4mar::MDMA_C4MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c4mar;
#[doc = "MDMA_C4MDR (rw) register accessor: an alias for `Reg<MDMA_C4MDR_SPEC>`"]
pub type MDMA_C4MDR = crate::Reg<mdma_c4mdr::MDMA_C4MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c4mdr;
#[doc = "MDMA_C5ISR (r) register accessor: an alias for `Reg<MDMA_C5ISR_SPEC>`"]
pub type MDMA_C5ISR = crate::Reg<mdma_c5isr::MDMA_C5ISR_SPEC>;
#[doc = "MDMA channel 5 interrupt/status register"]
pub mod mdma_c5isr;
#[doc = "MDMA_C5IFCR (w) register accessor: an alias for `Reg<MDMA_C5IFCR_SPEC>`"]
pub type MDMA_C5IFCR = crate::Reg<mdma_c5ifcr::MDMA_C5IFCR_SPEC>;
#[doc = "MDMA channel 5 interrupt flag clear register"]
pub mod mdma_c5ifcr;
#[doc = "MDMA_C5ESR (r) register accessor: an alias for `Reg<MDMA_C5ESR_SPEC>`"]
pub type MDMA_C5ESR = crate::Reg<mdma_c5esr::MDMA_C5ESR_SPEC>;
#[doc = "MDMA channel 5 error status register"]
pub mod mdma_c5esr;
#[doc = "MDMA_C5CR (rw) register accessor: an alias for `Reg<MDMA_C5CR_SPEC>`"]
pub type MDMA_C5CR = crate::Reg<mdma_c5cr::MDMA_C5CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c5cr;
#[doc = "MDMA_C5TCR (rw) register accessor: an alias for `Reg<MDMA_C5TCR_SPEC>`"]
pub type MDMA_C5TCR = crate::Reg<mdma_c5tcr::MDMA_C5TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c5tcr;
#[doc = "MDMA_C5BNDTR (rw) register accessor: an alias for `Reg<MDMA_C5BNDTR_SPEC>`"]
pub type MDMA_C5BNDTR = crate::Reg<mdma_c5bndtr::MDMA_C5BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c5bndtr;
#[doc = "MDMA_C5SAR (rw) register accessor: an alias for `Reg<MDMA_C5SAR_SPEC>`"]
pub type MDMA_C5SAR = crate::Reg<mdma_c5sar::MDMA_C5SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c5sar;
#[doc = "MDMA_C5DAR (rw) register accessor: an alias for `Reg<MDMA_C5DAR_SPEC>`"]
pub type MDMA_C5DAR = crate::Reg<mdma_c5dar::MDMA_C5DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c5dar;
#[doc = "MDMA_C5BRUR (rw) register accessor: an alias for `Reg<MDMA_C5BRUR_SPEC>`"]
pub type MDMA_C5BRUR = crate::Reg<mdma_c5brur::MDMA_C5BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c5brur;
#[doc = "MDMA_C5LAR (rw) register accessor: an alias for `Reg<MDMA_C5LAR_SPEC>`"]
pub type MDMA_C5LAR = crate::Reg<mdma_c5lar::MDMA_C5LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c5lar;
#[doc = "MDMA_C5TBR (rw) register accessor: an alias for `Reg<MDMA_C5TBR_SPEC>`"]
pub type MDMA_C5TBR = crate::Reg<mdma_c5tbr::MDMA_C5TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c5tbr;
#[doc = "MDMA_C5MAR (rw) register accessor: an alias for `Reg<MDMA_C5MAR_SPEC>`"]
pub type MDMA_C5MAR = crate::Reg<mdma_c5mar::MDMA_C5MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c5mar;
#[doc = "MDMA_C5MDR (rw) register accessor: an alias for `Reg<MDMA_C5MDR_SPEC>`"]
pub type MDMA_C5MDR = crate::Reg<mdma_c5mdr::MDMA_C5MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c5mdr;
#[doc = "MDMA_C6ISR (r) register accessor: an alias for `Reg<MDMA_C6ISR_SPEC>`"]
pub type MDMA_C6ISR = crate::Reg<mdma_c6isr::MDMA_C6ISR_SPEC>;
#[doc = "MDMA channel 6 interrupt/status register"]
pub mod mdma_c6isr;
#[doc = "MDMA_C6IFCR (w) register accessor: an alias for `Reg<MDMA_C6IFCR_SPEC>`"]
pub type MDMA_C6IFCR = crate::Reg<mdma_c6ifcr::MDMA_C6IFCR_SPEC>;
#[doc = "MDMA channel 6 interrupt flag clear register"]
pub mod mdma_c6ifcr;
#[doc = "MDMA_C6ESR (r) register accessor: an alias for `Reg<MDMA_C6ESR_SPEC>`"]
pub type MDMA_C6ESR = crate::Reg<mdma_c6esr::MDMA_C6ESR_SPEC>;
#[doc = "MDMA channel 6 error status register"]
pub mod mdma_c6esr;
#[doc = "MDMA_C6CR (rw) register accessor: an alias for `Reg<MDMA_C6CR_SPEC>`"]
pub type MDMA_C6CR = crate::Reg<mdma_c6cr::MDMA_C6CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c6cr;
#[doc = "MDMA_C6TCR (rw) register accessor: an alias for `Reg<MDMA_C6TCR_SPEC>`"]
pub type MDMA_C6TCR = crate::Reg<mdma_c6tcr::MDMA_C6TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c6tcr;
#[doc = "MDMA_C6BNDTR (rw) register accessor: an alias for `Reg<MDMA_C6BNDTR_SPEC>`"]
pub type MDMA_C6BNDTR = crate::Reg<mdma_c6bndtr::MDMA_C6BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c6bndtr;
#[doc = "MDMA_C6SAR (rw) register accessor: an alias for `Reg<MDMA_C6SAR_SPEC>`"]
pub type MDMA_C6SAR = crate::Reg<mdma_c6sar::MDMA_C6SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c6sar;
#[doc = "MDMA_C6DAR (rw) register accessor: an alias for `Reg<MDMA_C6DAR_SPEC>`"]
pub type MDMA_C6DAR = crate::Reg<mdma_c6dar::MDMA_C6DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c6dar;
#[doc = "MDMA_C6BRUR (rw) register accessor: an alias for `Reg<MDMA_C6BRUR_SPEC>`"]
pub type MDMA_C6BRUR = crate::Reg<mdma_c6brur::MDMA_C6BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c6brur;
#[doc = "MDMA_C6LAR (rw) register accessor: an alias for `Reg<MDMA_C6LAR_SPEC>`"]
pub type MDMA_C6LAR = crate::Reg<mdma_c6lar::MDMA_C6LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c6lar;
#[doc = "MDMA_C6TBR (rw) register accessor: an alias for `Reg<MDMA_C6TBR_SPEC>`"]
pub type MDMA_C6TBR = crate::Reg<mdma_c6tbr::MDMA_C6TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c6tbr;
#[doc = "MDMA_C6MAR (rw) register accessor: an alias for `Reg<MDMA_C6MAR_SPEC>`"]
pub type MDMA_C6MAR = crate::Reg<mdma_c6mar::MDMA_C6MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c6mar;
#[doc = "MDMA_C6MDR (rw) register accessor: an alias for `Reg<MDMA_C6MDR_SPEC>`"]
pub type MDMA_C6MDR = crate::Reg<mdma_c6mdr::MDMA_C6MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c6mdr;
#[doc = "MDMA_C7ISR (r) register accessor: an alias for `Reg<MDMA_C7ISR_SPEC>`"]
pub type MDMA_C7ISR = crate::Reg<mdma_c7isr::MDMA_C7ISR_SPEC>;
#[doc = "MDMA channel 7 interrupt/status register"]
pub mod mdma_c7isr;
#[doc = "MDMA_C7IFCR (w) register accessor: an alias for `Reg<MDMA_C7IFCR_SPEC>`"]
pub type MDMA_C7IFCR = crate::Reg<mdma_c7ifcr::MDMA_C7IFCR_SPEC>;
#[doc = "MDMA channel 7 interrupt flag clear register"]
pub mod mdma_c7ifcr;
#[doc = "MDMA_C7ESR (r) register accessor: an alias for `Reg<MDMA_C7ESR_SPEC>`"]
pub type MDMA_C7ESR = crate::Reg<mdma_c7esr::MDMA_C7ESR_SPEC>;
#[doc = "MDMA channel 7 error status register"]
pub mod mdma_c7esr;
#[doc = "MDMA_C7CR (rw) register accessor: an alias for `Reg<MDMA_C7CR_SPEC>`"]
pub type MDMA_C7CR = crate::Reg<mdma_c7cr::MDMA_C7CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c7cr;
#[doc = "MDMA_C7TCR (rw) register accessor: an alias for `Reg<MDMA_C7TCR_SPEC>`"]
pub type MDMA_C7TCR = crate::Reg<mdma_c7tcr::MDMA_C7TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c7tcr;
#[doc = "MDMA_C7BNDTR (rw) register accessor: an alias for `Reg<MDMA_C7BNDTR_SPEC>`"]
pub type MDMA_C7BNDTR = crate::Reg<mdma_c7bndtr::MDMA_C7BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c7bndtr;
#[doc = "MDMA_C7SAR (rw) register accessor: an alias for `Reg<MDMA_C7SAR_SPEC>`"]
pub type MDMA_C7SAR = crate::Reg<mdma_c7sar::MDMA_C7SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c7sar;
#[doc = "MDMA_C7DAR (rw) register accessor: an alias for `Reg<MDMA_C7DAR_SPEC>`"]
pub type MDMA_C7DAR = crate::Reg<mdma_c7dar::MDMA_C7DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c7dar;
#[doc = "MDMA_C7BRUR (rw) register accessor: an alias for `Reg<MDMA_C7BRUR_SPEC>`"]
pub type MDMA_C7BRUR = crate::Reg<mdma_c7brur::MDMA_C7BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c7brur;
#[doc = "MDMA_C7LAR (rw) register accessor: an alias for `Reg<MDMA_C7LAR_SPEC>`"]
pub type MDMA_C7LAR = crate::Reg<mdma_c7lar::MDMA_C7LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c7lar;
#[doc = "MDMA_C7TBR (rw) register accessor: an alias for `Reg<MDMA_C7TBR_SPEC>`"]
pub type MDMA_C7TBR = crate::Reg<mdma_c7tbr::MDMA_C7TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c7tbr;
#[doc = "MDMA_C7MAR (rw) register accessor: an alias for `Reg<MDMA_C7MAR_SPEC>`"]
pub type MDMA_C7MAR = crate::Reg<mdma_c7mar::MDMA_C7MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c7mar;
#[doc = "MDMA_C7MDR (rw) register accessor: an alias for `Reg<MDMA_C7MDR_SPEC>`"]
pub type MDMA_C7MDR = crate::Reg<mdma_c7mdr::MDMA_C7MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c7mdr;
#[doc = "MDMA_C8ISR (r) register accessor: an alias for `Reg<MDMA_C8ISR_SPEC>`"]
pub type MDMA_C8ISR = crate::Reg<mdma_c8isr::MDMA_C8ISR_SPEC>;
#[doc = "MDMA channel 8 interrupt/status register"]
pub mod mdma_c8isr;
#[doc = "MDMA_C8IFCR (w) register accessor: an alias for `Reg<MDMA_C8IFCR_SPEC>`"]
pub type MDMA_C8IFCR = crate::Reg<mdma_c8ifcr::MDMA_C8IFCR_SPEC>;
#[doc = "MDMA channel 8 interrupt flag clear register"]
pub mod mdma_c8ifcr;
#[doc = "MDMA_C8ESR (r) register accessor: an alias for `Reg<MDMA_C8ESR_SPEC>`"]
pub type MDMA_C8ESR = crate::Reg<mdma_c8esr::MDMA_C8ESR_SPEC>;
#[doc = "MDMA channel 8 error status register"]
pub mod mdma_c8esr;
#[doc = "MDMA_C8CR (rw) register accessor: an alias for `Reg<MDMA_C8CR_SPEC>`"]
pub type MDMA_C8CR = crate::Reg<mdma_c8cr::MDMA_C8CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c8cr;
#[doc = "MDMA_C8TCR (rw) register accessor: an alias for `Reg<MDMA_C8TCR_SPEC>`"]
pub type MDMA_C8TCR = crate::Reg<mdma_c8tcr::MDMA_C8TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c8tcr;
#[doc = "MDMA_C8BNDTR (rw) register accessor: an alias for `Reg<MDMA_C8BNDTR_SPEC>`"]
pub type MDMA_C8BNDTR = crate::Reg<mdma_c8bndtr::MDMA_C8BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c8bndtr;
#[doc = "MDMA_C8SAR (rw) register accessor: an alias for `Reg<MDMA_C8SAR_SPEC>`"]
pub type MDMA_C8SAR = crate::Reg<mdma_c8sar::MDMA_C8SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c8sar;
#[doc = "MDMA_C8DAR (rw) register accessor: an alias for `Reg<MDMA_C8DAR_SPEC>`"]
pub type MDMA_C8DAR = crate::Reg<mdma_c8dar::MDMA_C8DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c8dar;
#[doc = "MDMA_C8BRUR (rw) register accessor: an alias for `Reg<MDMA_C8BRUR_SPEC>`"]
pub type MDMA_C8BRUR = crate::Reg<mdma_c8brur::MDMA_C8BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c8brur;
#[doc = "MDMA_C8LAR (rw) register accessor: an alias for `Reg<MDMA_C8LAR_SPEC>`"]
pub type MDMA_C8LAR = crate::Reg<mdma_c8lar::MDMA_C8LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c8lar;
#[doc = "MDMA_C8TBR (rw) register accessor: an alias for `Reg<MDMA_C8TBR_SPEC>`"]
pub type MDMA_C8TBR = crate::Reg<mdma_c8tbr::MDMA_C8TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c8tbr;
#[doc = "MDMA_C8MAR (rw) register accessor: an alias for `Reg<MDMA_C8MAR_SPEC>`"]
pub type MDMA_C8MAR = crate::Reg<mdma_c8mar::MDMA_C8MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c8mar;
#[doc = "MDMA_C8MDR (rw) register accessor: an alias for `Reg<MDMA_C8MDR_SPEC>`"]
pub type MDMA_C8MDR = crate::Reg<mdma_c8mdr::MDMA_C8MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c8mdr;
#[doc = "MDMA_C9ISR (r) register accessor: an alias for `Reg<MDMA_C9ISR_SPEC>`"]
pub type MDMA_C9ISR = crate::Reg<mdma_c9isr::MDMA_C9ISR_SPEC>;
#[doc = "MDMA channel 9 interrupt/status register"]
pub mod mdma_c9isr;
#[doc = "MDMA_C9IFCR (w) register accessor: an alias for `Reg<MDMA_C9IFCR_SPEC>`"]
pub type MDMA_C9IFCR = crate::Reg<mdma_c9ifcr::MDMA_C9IFCR_SPEC>;
#[doc = "MDMA channel 9 interrupt flag clear register"]
pub mod mdma_c9ifcr;
#[doc = "MDMA_C9ESR (r) register accessor: an alias for `Reg<MDMA_C9ESR_SPEC>`"]
pub type MDMA_C9ESR = crate::Reg<mdma_c9esr::MDMA_C9ESR_SPEC>;
#[doc = "MDMA channel 9 error status register"]
pub mod mdma_c9esr;
#[doc = "MDMA_C9CR (rw) register accessor: an alias for `Reg<MDMA_C9CR_SPEC>`"]
pub type MDMA_C9CR = crate::Reg<mdma_c9cr::MDMA_C9CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c9cr;
#[doc = "MDMA_C9TCR (rw) register accessor: an alias for `Reg<MDMA_C9TCR_SPEC>`"]
pub type MDMA_C9TCR = crate::Reg<mdma_c9tcr::MDMA_C9TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c9tcr;
#[doc = "MDMA_C9BNDTR (rw) register accessor: an alias for `Reg<MDMA_C9BNDTR_SPEC>`"]
pub type MDMA_C9BNDTR = crate::Reg<mdma_c9bndtr::MDMA_C9BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c9bndtr;
#[doc = "MDMA_C9SAR (rw) register accessor: an alias for `Reg<MDMA_C9SAR_SPEC>`"]
pub type MDMA_C9SAR = crate::Reg<mdma_c9sar::MDMA_C9SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c9sar;
#[doc = "MDMA_C9DAR (rw) register accessor: an alias for `Reg<MDMA_C9DAR_SPEC>`"]
pub type MDMA_C9DAR = crate::Reg<mdma_c9dar::MDMA_C9DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c9dar;
#[doc = "MDMA_C9BRUR (rw) register accessor: an alias for `Reg<MDMA_C9BRUR_SPEC>`"]
pub type MDMA_C9BRUR = crate::Reg<mdma_c9brur::MDMA_C9BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c9brur;
#[doc = "MDMA_C9LAR (rw) register accessor: an alias for `Reg<MDMA_C9LAR_SPEC>`"]
pub type MDMA_C9LAR = crate::Reg<mdma_c9lar::MDMA_C9LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c9lar;
#[doc = "MDMA_C9TBR (rw) register accessor: an alias for `Reg<MDMA_C9TBR_SPEC>`"]
pub type MDMA_C9TBR = crate::Reg<mdma_c9tbr::MDMA_C9TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c9tbr;
#[doc = "MDMA_C9MAR (rw) register accessor: an alias for `Reg<MDMA_C9MAR_SPEC>`"]
pub type MDMA_C9MAR = crate::Reg<mdma_c9mar::MDMA_C9MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c9mar;
#[doc = "MDMA_C9MDR (rw) register accessor: an alias for `Reg<MDMA_C9MDR_SPEC>`"]
pub type MDMA_C9MDR = crate::Reg<mdma_c9mdr::MDMA_C9MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c9mdr;
#[doc = "MDMA_C10ISR (r) register accessor: an alias for `Reg<MDMA_C10ISR_SPEC>`"]
pub type MDMA_C10ISR = crate::Reg<mdma_c10isr::MDMA_C10ISR_SPEC>;
#[doc = "MDMA channel 10 interrupt/status register"]
pub mod mdma_c10isr;
#[doc = "MDMA_C10IFCR (w) register accessor: an alias for `Reg<MDMA_C10IFCR_SPEC>`"]
pub type MDMA_C10IFCR = crate::Reg<mdma_c10ifcr::MDMA_C10IFCR_SPEC>;
#[doc = "MDMA channel 10 interrupt flag clear register"]
pub mod mdma_c10ifcr;
#[doc = "MDMA_C10ESR (r) register accessor: an alias for `Reg<MDMA_C10ESR_SPEC>`"]
pub type MDMA_C10ESR = crate::Reg<mdma_c10esr::MDMA_C10ESR_SPEC>;
#[doc = "MDMA channel 10 error status register"]
pub mod mdma_c10esr;
#[doc = "MDMA_C10CR (rw) register accessor: an alias for `Reg<MDMA_C10CR_SPEC>`"]
pub type MDMA_C10CR = crate::Reg<mdma_c10cr::MDMA_C10CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c10cr;
#[doc = "MDMA_C10TCR (rw) register accessor: an alias for `Reg<MDMA_C10TCR_SPEC>`"]
pub type MDMA_C10TCR = crate::Reg<mdma_c10tcr::MDMA_C10TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c10tcr;
#[doc = "MDMA_C10BNDTR (rw) register accessor: an alias for `Reg<MDMA_C10BNDTR_SPEC>`"]
pub type MDMA_C10BNDTR = crate::Reg<mdma_c10bndtr::MDMA_C10BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c10bndtr;
#[doc = "MDMA_C10SAR (rw) register accessor: an alias for `Reg<MDMA_C10SAR_SPEC>`"]
pub type MDMA_C10SAR = crate::Reg<mdma_c10sar::MDMA_C10SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c10sar;
#[doc = "MDMA_C10DAR (rw) register accessor: an alias for `Reg<MDMA_C10DAR_SPEC>`"]
pub type MDMA_C10DAR = crate::Reg<mdma_c10dar::MDMA_C10DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c10dar;
#[doc = "MDMA_C10BRUR (rw) register accessor: an alias for `Reg<MDMA_C10BRUR_SPEC>`"]
pub type MDMA_C10BRUR = crate::Reg<mdma_c10brur::MDMA_C10BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c10brur;
#[doc = "MDMA_C10LAR (rw) register accessor: an alias for `Reg<MDMA_C10LAR_SPEC>`"]
pub type MDMA_C10LAR = crate::Reg<mdma_c10lar::MDMA_C10LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c10lar;
#[doc = "MDMA_C10TBR (rw) register accessor: an alias for `Reg<MDMA_C10TBR_SPEC>`"]
pub type MDMA_C10TBR = crate::Reg<mdma_c10tbr::MDMA_C10TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c10tbr;
#[doc = "MDMA_C10MAR (rw) register accessor: an alias for `Reg<MDMA_C10MAR_SPEC>`"]
pub type MDMA_C10MAR = crate::Reg<mdma_c10mar::MDMA_C10MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c10mar;
#[doc = "MDMA_C10MDR (rw) register accessor: an alias for `Reg<MDMA_C10MDR_SPEC>`"]
pub type MDMA_C10MDR = crate::Reg<mdma_c10mdr::MDMA_C10MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c10mdr;
#[doc = "MDMA_C11ISR (r) register accessor: an alias for `Reg<MDMA_C11ISR_SPEC>`"]
pub type MDMA_C11ISR = crate::Reg<mdma_c11isr::MDMA_C11ISR_SPEC>;
#[doc = "MDMA channel 11 interrupt/status register"]
pub mod mdma_c11isr;
#[doc = "MDMA_C11IFCR (w) register accessor: an alias for `Reg<MDMA_C11IFCR_SPEC>`"]
pub type MDMA_C11IFCR = crate::Reg<mdma_c11ifcr::MDMA_C11IFCR_SPEC>;
#[doc = "MDMA channel 11 interrupt flag clear register"]
pub mod mdma_c11ifcr;
#[doc = "MDMA_C11ESR (r) register accessor: an alias for `Reg<MDMA_C11ESR_SPEC>`"]
pub type MDMA_C11ESR = crate::Reg<mdma_c11esr::MDMA_C11ESR_SPEC>;
#[doc = "MDMA channel 11 error status register"]
pub mod mdma_c11esr;
#[doc = "MDMA_C11CR (rw) register accessor: an alias for `Reg<MDMA_C11CR_SPEC>`"]
pub type MDMA_C11CR = crate::Reg<mdma_c11cr::MDMA_C11CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c11cr;
#[doc = "MDMA_C11TCR (rw) register accessor: an alias for `Reg<MDMA_C11TCR_SPEC>`"]
pub type MDMA_C11TCR = crate::Reg<mdma_c11tcr::MDMA_C11TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c11tcr;
#[doc = "MDMA_C11BNDTR (rw) register accessor: an alias for `Reg<MDMA_C11BNDTR_SPEC>`"]
pub type MDMA_C11BNDTR = crate::Reg<mdma_c11bndtr::MDMA_C11BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c11bndtr;
#[doc = "MDMA_C11SAR (rw) register accessor: an alias for `Reg<MDMA_C11SAR_SPEC>`"]
pub type MDMA_C11SAR = crate::Reg<mdma_c11sar::MDMA_C11SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c11sar;
#[doc = "MDMA_C11DAR (rw) register accessor: an alias for `Reg<MDMA_C11DAR_SPEC>`"]
pub type MDMA_C11DAR = crate::Reg<mdma_c11dar::MDMA_C11DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c11dar;
#[doc = "MDMA_C11BRUR (rw) register accessor: an alias for `Reg<MDMA_C11BRUR_SPEC>`"]
pub type MDMA_C11BRUR = crate::Reg<mdma_c11brur::MDMA_C11BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c11brur;
#[doc = "MDMA_C11LAR (rw) register accessor: an alias for `Reg<MDMA_C11LAR_SPEC>`"]
pub type MDMA_C11LAR = crate::Reg<mdma_c11lar::MDMA_C11LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c11lar;
#[doc = "MDMA_C11TBR (rw) register accessor: an alias for `Reg<MDMA_C11TBR_SPEC>`"]
pub type MDMA_C11TBR = crate::Reg<mdma_c11tbr::MDMA_C11TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c11tbr;
#[doc = "MDMA_C11MAR (rw) register accessor: an alias for `Reg<MDMA_C11MAR_SPEC>`"]
pub type MDMA_C11MAR = crate::Reg<mdma_c11mar::MDMA_C11MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c11mar;
#[doc = "MDMA_C11MDR (rw) register accessor: an alias for `Reg<MDMA_C11MDR_SPEC>`"]
pub type MDMA_C11MDR = crate::Reg<mdma_c11mdr::MDMA_C11MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c11mdr;
#[doc = "MDMA_C12ISR (r) register accessor: an alias for `Reg<MDMA_C12ISR_SPEC>`"]
pub type MDMA_C12ISR = crate::Reg<mdma_c12isr::MDMA_C12ISR_SPEC>;
#[doc = "MDMA channel 12 interrupt/status register"]
pub mod mdma_c12isr;
#[doc = "MDMA_C12IFCR (w) register accessor: an alias for `Reg<MDMA_C12IFCR_SPEC>`"]
pub type MDMA_C12IFCR = crate::Reg<mdma_c12ifcr::MDMA_C12IFCR_SPEC>;
#[doc = "MDMA channel 12 interrupt flag clear register"]
pub mod mdma_c12ifcr;
#[doc = "MDMA_C12ESR (r) register accessor: an alias for `Reg<MDMA_C12ESR_SPEC>`"]
pub type MDMA_C12ESR = crate::Reg<mdma_c12esr::MDMA_C12ESR_SPEC>;
#[doc = "MDMA channel 12 error status register"]
pub mod mdma_c12esr;
#[doc = "MDMA_C12CR (rw) register accessor: an alias for `Reg<MDMA_C12CR_SPEC>`"]
pub type MDMA_C12CR = crate::Reg<mdma_c12cr::MDMA_C12CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c12cr;
#[doc = "MDMA_C12TCR (rw) register accessor: an alias for `Reg<MDMA_C12TCR_SPEC>`"]
pub type MDMA_C12TCR = crate::Reg<mdma_c12tcr::MDMA_C12TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c12tcr;
#[doc = "MDMA_C12BNDTR (rw) register accessor: an alias for `Reg<MDMA_C12BNDTR_SPEC>`"]
pub type MDMA_C12BNDTR = crate::Reg<mdma_c12bndtr::MDMA_C12BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c12bndtr;
#[doc = "MDMA_C12SAR (rw) register accessor: an alias for `Reg<MDMA_C12SAR_SPEC>`"]
pub type MDMA_C12SAR = crate::Reg<mdma_c12sar::MDMA_C12SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c12sar;
#[doc = "MDMA_C12DAR (rw) register accessor: an alias for `Reg<MDMA_C12DAR_SPEC>`"]
pub type MDMA_C12DAR = crate::Reg<mdma_c12dar::MDMA_C12DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c12dar;
#[doc = "MDMA_C12BRUR (rw) register accessor: an alias for `Reg<MDMA_C12BRUR_SPEC>`"]
pub type MDMA_C12BRUR = crate::Reg<mdma_c12brur::MDMA_C12BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c12brur;
#[doc = "MDMA_C12LAR (rw) register accessor: an alias for `Reg<MDMA_C12LAR_SPEC>`"]
pub type MDMA_C12LAR = crate::Reg<mdma_c12lar::MDMA_C12LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c12lar;
#[doc = "MDMA_C12TBR (rw) register accessor: an alias for `Reg<MDMA_C12TBR_SPEC>`"]
pub type MDMA_C12TBR = crate::Reg<mdma_c12tbr::MDMA_C12TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c12tbr;
#[doc = "MDMA_C12MAR (rw) register accessor: an alias for `Reg<MDMA_C12MAR_SPEC>`"]
pub type MDMA_C12MAR = crate::Reg<mdma_c12mar::MDMA_C12MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c12mar;
#[doc = "MDMA_C12MDR (rw) register accessor: an alias for `Reg<MDMA_C12MDR_SPEC>`"]
pub type MDMA_C12MDR = crate::Reg<mdma_c12mdr::MDMA_C12MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c12mdr;
#[doc = "MDMA_C13ISR (r) register accessor: an alias for `Reg<MDMA_C13ISR_SPEC>`"]
pub type MDMA_C13ISR = crate::Reg<mdma_c13isr::MDMA_C13ISR_SPEC>;
#[doc = "MDMA channel 13 interrupt/status register"]
pub mod mdma_c13isr;
#[doc = "MDMA_C13IFCR (w) register accessor: an alias for `Reg<MDMA_C13IFCR_SPEC>`"]
pub type MDMA_C13IFCR = crate::Reg<mdma_c13ifcr::MDMA_C13IFCR_SPEC>;
#[doc = "MDMA channel 13 interrupt flag clear register"]
pub mod mdma_c13ifcr;
#[doc = "MDMA_C13ESR (r) register accessor: an alias for `Reg<MDMA_C13ESR_SPEC>`"]
pub type MDMA_C13ESR = crate::Reg<mdma_c13esr::MDMA_C13ESR_SPEC>;
#[doc = "MDMA channel 13 error status register"]
pub mod mdma_c13esr;
#[doc = "MDMA_C13CR (rw) register accessor: an alias for `Reg<MDMA_C13CR_SPEC>`"]
pub type MDMA_C13CR = crate::Reg<mdma_c13cr::MDMA_C13CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c13cr;
#[doc = "MDMA_C13TCR (rw) register accessor: an alias for `Reg<MDMA_C13TCR_SPEC>`"]
pub type MDMA_C13TCR = crate::Reg<mdma_c13tcr::MDMA_C13TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c13tcr;
#[doc = "MDMA_C13BNDTR (rw) register accessor: an alias for `Reg<MDMA_C13BNDTR_SPEC>`"]
pub type MDMA_C13BNDTR = crate::Reg<mdma_c13bndtr::MDMA_C13BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c13bndtr;
#[doc = "MDMA_C13SAR (rw) register accessor: an alias for `Reg<MDMA_C13SAR_SPEC>`"]
pub type MDMA_C13SAR = crate::Reg<mdma_c13sar::MDMA_C13SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c13sar;
#[doc = "MDMA_C13DAR (rw) register accessor: an alias for `Reg<MDMA_C13DAR_SPEC>`"]
pub type MDMA_C13DAR = crate::Reg<mdma_c13dar::MDMA_C13DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c13dar;
#[doc = "MDMA_C13BRUR (rw) register accessor: an alias for `Reg<MDMA_C13BRUR_SPEC>`"]
pub type MDMA_C13BRUR = crate::Reg<mdma_c13brur::MDMA_C13BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c13brur;
#[doc = "MDMA_C13LAR (rw) register accessor: an alias for `Reg<MDMA_C13LAR_SPEC>`"]
pub type MDMA_C13LAR = crate::Reg<mdma_c13lar::MDMA_C13LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c13lar;
#[doc = "MDMA_C13TBR (rw) register accessor: an alias for `Reg<MDMA_C13TBR_SPEC>`"]
pub type MDMA_C13TBR = crate::Reg<mdma_c13tbr::MDMA_C13TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c13tbr;
#[doc = "MDMA_C13MAR (rw) register accessor: an alias for `Reg<MDMA_C13MAR_SPEC>`"]
pub type MDMA_C13MAR = crate::Reg<mdma_c13mar::MDMA_C13MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c13mar;
#[doc = "MDMA_C13MDR (rw) register accessor: an alias for `Reg<MDMA_C13MDR_SPEC>`"]
pub type MDMA_C13MDR = crate::Reg<mdma_c13mdr::MDMA_C13MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c13mdr;
#[doc = "MDMA_C14ISR (r) register accessor: an alias for `Reg<MDMA_C14ISR_SPEC>`"]
pub type MDMA_C14ISR = crate::Reg<mdma_c14isr::MDMA_C14ISR_SPEC>;
#[doc = "MDMA channel 14 interrupt/status register"]
pub mod mdma_c14isr;
#[doc = "MDMA_C14IFCR (w) register accessor: an alias for `Reg<MDMA_C14IFCR_SPEC>`"]
pub type MDMA_C14IFCR = crate::Reg<mdma_c14ifcr::MDMA_C14IFCR_SPEC>;
#[doc = "MDMA channel 14 interrupt flag clear register"]
pub mod mdma_c14ifcr;
#[doc = "MDMA_C14ESR (r) register accessor: an alias for `Reg<MDMA_C14ESR_SPEC>`"]
pub type MDMA_C14ESR = crate::Reg<mdma_c14esr::MDMA_C14ESR_SPEC>;
#[doc = "MDMA channel 14 error status register"]
pub mod mdma_c14esr;
#[doc = "MDMA_C14CR (rw) register accessor: an alias for `Reg<MDMA_C14CR_SPEC>`"]
pub type MDMA_C14CR = crate::Reg<mdma_c14cr::MDMA_C14CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c14cr;
#[doc = "MDMA_C14TCR (rw) register accessor: an alias for `Reg<MDMA_C14TCR_SPEC>`"]
pub type MDMA_C14TCR = crate::Reg<mdma_c14tcr::MDMA_C14TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c14tcr;
#[doc = "MDMA_C14BNDTR (rw) register accessor: an alias for `Reg<MDMA_C14BNDTR_SPEC>`"]
pub type MDMA_C14BNDTR = crate::Reg<mdma_c14bndtr::MDMA_C14BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c14bndtr;
#[doc = "MDMA_C14SAR (rw) register accessor: an alias for `Reg<MDMA_C14SAR_SPEC>`"]
pub type MDMA_C14SAR = crate::Reg<mdma_c14sar::MDMA_C14SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c14sar;
#[doc = "MDMA_C14DAR (rw) register accessor: an alias for `Reg<MDMA_C14DAR_SPEC>`"]
pub type MDMA_C14DAR = crate::Reg<mdma_c14dar::MDMA_C14DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c14dar;
#[doc = "MDMA_C14BRUR (rw) register accessor: an alias for `Reg<MDMA_C14BRUR_SPEC>`"]
pub type MDMA_C14BRUR = crate::Reg<mdma_c14brur::MDMA_C14BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c14brur;
#[doc = "MDMA_C14LAR (rw) register accessor: an alias for `Reg<MDMA_C14LAR_SPEC>`"]
pub type MDMA_C14LAR = crate::Reg<mdma_c14lar::MDMA_C14LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c14lar;
#[doc = "MDMA_C14TBR (rw) register accessor: an alias for `Reg<MDMA_C14TBR_SPEC>`"]
pub type MDMA_C14TBR = crate::Reg<mdma_c14tbr::MDMA_C14TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c14tbr;
#[doc = "MDMA_C14MAR (rw) register accessor: an alias for `Reg<MDMA_C14MAR_SPEC>`"]
pub type MDMA_C14MAR = crate::Reg<mdma_c14mar::MDMA_C14MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c14mar;
#[doc = "MDMA_C14MDR (rw) register accessor: an alias for `Reg<MDMA_C14MDR_SPEC>`"]
pub type MDMA_C14MDR = crate::Reg<mdma_c14mdr::MDMA_C14MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c14mdr;
#[doc = "MDMA_C15ISR (r) register accessor: an alias for `Reg<MDMA_C15ISR_SPEC>`"]
pub type MDMA_C15ISR = crate::Reg<mdma_c15isr::MDMA_C15ISR_SPEC>;
#[doc = "MDMA channel 15 interrupt/status register"]
pub mod mdma_c15isr;
#[doc = "MDMA_C15IFCR (w) register accessor: an alias for `Reg<MDMA_C15IFCR_SPEC>`"]
pub type MDMA_C15IFCR = crate::Reg<mdma_c15ifcr::MDMA_C15IFCR_SPEC>;
#[doc = "MDMA channel 15 interrupt flag clear register"]
pub mod mdma_c15ifcr;
#[doc = "MDMA_C15ESR (r) register accessor: an alias for `Reg<MDMA_C15ESR_SPEC>`"]
pub type MDMA_C15ESR = crate::Reg<mdma_c15esr::MDMA_C15ESR_SPEC>;
#[doc = "MDMA channel 15 error status register"]
pub mod mdma_c15esr;
#[doc = "MDMA_C15CR (rw) register accessor: an alias for `Reg<MDMA_C15CR_SPEC>`"]
pub type MDMA_C15CR = crate::Reg<mdma_c15cr::MDMA_C15CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c15cr;
#[doc = "MDMA_C15TCR (rw) register accessor: an alias for `Reg<MDMA_C15TCR_SPEC>`"]
pub type MDMA_C15TCR = crate::Reg<mdma_c15tcr::MDMA_C15TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c15tcr;
#[doc = "MDMA_C15BNDTR (rw) register accessor: an alias for `Reg<MDMA_C15BNDTR_SPEC>`"]
pub type MDMA_C15BNDTR = crate::Reg<mdma_c15bndtr::MDMA_C15BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c15bndtr;
#[doc = "MDMA_C15SAR (rw) register accessor: an alias for `Reg<MDMA_C15SAR_SPEC>`"]
pub type MDMA_C15SAR = crate::Reg<mdma_c15sar::MDMA_C15SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c15sar;
#[doc = "MDMA_C15DAR (rw) register accessor: an alias for `Reg<MDMA_C15DAR_SPEC>`"]
pub type MDMA_C15DAR = crate::Reg<mdma_c15dar::MDMA_C15DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c15dar;
#[doc = "MDMA_C15BRUR (rw) register accessor: an alias for `Reg<MDMA_C15BRUR_SPEC>`"]
pub type MDMA_C15BRUR = crate::Reg<mdma_c15brur::MDMA_C15BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c15brur;
#[doc = "MDMA_C15LAR (rw) register accessor: an alias for `Reg<MDMA_C15LAR_SPEC>`"]
pub type MDMA_C15LAR = crate::Reg<mdma_c15lar::MDMA_C15LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c15lar;
#[doc = "MDMA_C15TBR (rw) register accessor: an alias for `Reg<MDMA_C15TBR_SPEC>`"]
pub type MDMA_C15TBR = crate::Reg<mdma_c15tbr::MDMA_C15TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c15tbr;
#[doc = "MDMA_C15MAR (rw) register accessor: an alias for `Reg<MDMA_C15MAR_SPEC>`"]
pub type MDMA_C15MAR = crate::Reg<mdma_c15mar::MDMA_C15MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c15mar;
#[doc = "MDMA_C15MDR (rw) register accessor: an alias for `Reg<MDMA_C15MDR_SPEC>`"]
pub type MDMA_C15MDR = crate::Reg<mdma_c15mdr::MDMA_C15MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c15mdr;
#[doc = "MDMA_C16ISR (r) register accessor: an alias for `Reg<MDMA_C16ISR_SPEC>`"]
pub type MDMA_C16ISR = crate::Reg<mdma_c16isr::MDMA_C16ISR_SPEC>;
#[doc = "MDMA channel 16 interrupt/status register"]
pub mod mdma_c16isr;
#[doc = "MDMA_C16IFCR (w) register accessor: an alias for `Reg<MDMA_C16IFCR_SPEC>`"]
pub type MDMA_C16IFCR = crate::Reg<mdma_c16ifcr::MDMA_C16IFCR_SPEC>;
#[doc = "MDMA channel 16 interrupt flag clear register"]
pub mod mdma_c16ifcr;
#[doc = "MDMA_C16ESR (r) register accessor: an alias for `Reg<MDMA_C16ESR_SPEC>`"]
pub type MDMA_C16ESR = crate::Reg<mdma_c16esr::MDMA_C16ESR_SPEC>;
#[doc = "MDMA channel 16 error status register"]
pub mod mdma_c16esr;
#[doc = "MDMA_C16CR (rw) register accessor: an alias for `Reg<MDMA_C16CR_SPEC>`"]
pub type MDMA_C16CR = crate::Reg<mdma_c16cr::MDMA_C16CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c16cr;
#[doc = "MDMA_C16TCR (rw) register accessor: an alias for `Reg<MDMA_C16TCR_SPEC>`"]
pub type MDMA_C16TCR = crate::Reg<mdma_c16tcr::MDMA_C16TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c16tcr;
#[doc = "MDMA_C16BNDTR (rw) register accessor: an alias for `Reg<MDMA_C16BNDTR_SPEC>`"]
pub type MDMA_C16BNDTR = crate::Reg<mdma_c16bndtr::MDMA_C16BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c16bndtr;
#[doc = "MDMA_C16SAR (rw) register accessor: an alias for `Reg<MDMA_C16SAR_SPEC>`"]
pub type MDMA_C16SAR = crate::Reg<mdma_c16sar::MDMA_C16SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c16sar;
#[doc = "MDMA_C16DAR (rw) register accessor: an alias for `Reg<MDMA_C16DAR_SPEC>`"]
pub type MDMA_C16DAR = crate::Reg<mdma_c16dar::MDMA_C16DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c16dar;
#[doc = "MDMA_C16BRUR (rw) register accessor: an alias for `Reg<MDMA_C16BRUR_SPEC>`"]
pub type MDMA_C16BRUR = crate::Reg<mdma_c16brur::MDMA_C16BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c16brur;
#[doc = "MDMA_C16LAR (rw) register accessor: an alias for `Reg<MDMA_C16LAR_SPEC>`"]
pub type MDMA_C16LAR = crate::Reg<mdma_c16lar::MDMA_C16LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c16lar;
#[doc = "MDMA_C16TBR (rw) register accessor: an alias for `Reg<MDMA_C16TBR_SPEC>`"]
pub type MDMA_C16TBR = crate::Reg<mdma_c16tbr::MDMA_C16TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c16tbr;
#[doc = "MDMA_C16MAR (rw) register accessor: an alias for `Reg<MDMA_C16MAR_SPEC>`"]
pub type MDMA_C16MAR = crate::Reg<mdma_c16mar::MDMA_C16MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c16mar;
#[doc = "MDMA_C16MDR (rw) register accessor: an alias for `Reg<MDMA_C16MDR_SPEC>`"]
pub type MDMA_C16MDR = crate::Reg<mdma_c16mdr::MDMA_C16MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c16mdr;
#[doc = "MDMA_C17ISR (r) register accessor: an alias for `Reg<MDMA_C17ISR_SPEC>`"]
pub type MDMA_C17ISR = crate::Reg<mdma_c17isr::MDMA_C17ISR_SPEC>;
#[doc = "MDMA channel 17 interrupt/status register"]
pub mod mdma_c17isr;
#[doc = "MDMA_C17IFCR (w) register accessor: an alias for `Reg<MDMA_C17IFCR_SPEC>`"]
pub type MDMA_C17IFCR = crate::Reg<mdma_c17ifcr::MDMA_C17IFCR_SPEC>;
#[doc = "MDMA channel 17 interrupt flag clear register"]
pub mod mdma_c17ifcr;
#[doc = "MDMA_C17ESR (r) register accessor: an alias for `Reg<MDMA_C17ESR_SPEC>`"]
pub type MDMA_C17ESR = crate::Reg<mdma_c17esr::MDMA_C17ESR_SPEC>;
#[doc = "MDMA channel 17 error status register"]
pub mod mdma_c17esr;
#[doc = "MDMA_C17CR (rw) register accessor: an alias for `Reg<MDMA_C17CR_SPEC>`"]
pub type MDMA_C17CR = crate::Reg<mdma_c17cr::MDMA_C17CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c17cr;
#[doc = "MDMA_C17TCR (rw) register accessor: an alias for `Reg<MDMA_C17TCR_SPEC>`"]
pub type MDMA_C17TCR = crate::Reg<mdma_c17tcr::MDMA_C17TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c17tcr;
#[doc = "MDMA_C17BNDTR (rw) register accessor: an alias for `Reg<MDMA_C17BNDTR_SPEC>`"]
pub type MDMA_C17BNDTR = crate::Reg<mdma_c17bndtr::MDMA_C17BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c17bndtr;
#[doc = "MDMA_C17SAR (rw) register accessor: an alias for `Reg<MDMA_C17SAR_SPEC>`"]
pub type MDMA_C17SAR = crate::Reg<mdma_c17sar::MDMA_C17SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c17sar;
#[doc = "MDMA_C17DAR (rw) register accessor: an alias for `Reg<MDMA_C17DAR_SPEC>`"]
pub type MDMA_C17DAR = crate::Reg<mdma_c17dar::MDMA_C17DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c17dar;
#[doc = "MDMA_C17BRUR (rw) register accessor: an alias for `Reg<MDMA_C17BRUR_SPEC>`"]
pub type MDMA_C17BRUR = crate::Reg<mdma_c17brur::MDMA_C17BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c17brur;
#[doc = "MDMA_C17LAR (rw) register accessor: an alias for `Reg<MDMA_C17LAR_SPEC>`"]
pub type MDMA_C17LAR = crate::Reg<mdma_c17lar::MDMA_C17LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c17lar;
#[doc = "MDMA_C17TBR (rw) register accessor: an alias for `Reg<MDMA_C17TBR_SPEC>`"]
pub type MDMA_C17TBR = crate::Reg<mdma_c17tbr::MDMA_C17TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c17tbr;
#[doc = "MDMA_C17MAR (rw) register accessor: an alias for `Reg<MDMA_C17MAR_SPEC>`"]
pub type MDMA_C17MAR = crate::Reg<mdma_c17mar::MDMA_C17MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c17mar;
#[doc = "MDMA_C17MDR (rw) register accessor: an alias for `Reg<MDMA_C17MDR_SPEC>`"]
pub type MDMA_C17MDR = crate::Reg<mdma_c17mdr::MDMA_C17MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c17mdr;
#[doc = "MDMA_C18ISR (r) register accessor: an alias for `Reg<MDMA_C18ISR_SPEC>`"]
pub type MDMA_C18ISR = crate::Reg<mdma_c18isr::MDMA_C18ISR_SPEC>;
#[doc = "MDMA channel 18 interrupt/status register"]
pub mod mdma_c18isr;
#[doc = "MDMA_C18IFCR (w) register accessor: an alias for `Reg<MDMA_C18IFCR_SPEC>`"]
pub type MDMA_C18IFCR = crate::Reg<mdma_c18ifcr::MDMA_C18IFCR_SPEC>;
#[doc = "MDMA channel 18 interrupt flag clear register"]
pub mod mdma_c18ifcr;
#[doc = "MDMA_C18ESR (r) register accessor: an alias for `Reg<MDMA_C18ESR_SPEC>`"]
pub type MDMA_C18ESR = crate::Reg<mdma_c18esr::MDMA_C18ESR_SPEC>;
#[doc = "MDMA channel 18 error status register"]
pub mod mdma_c18esr;
#[doc = "MDMA_C18CR (rw) register accessor: an alias for `Reg<MDMA_C18CR_SPEC>`"]
pub type MDMA_C18CR = crate::Reg<mdma_c18cr::MDMA_C18CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c18cr;
#[doc = "MDMA_C18TCR (rw) register accessor: an alias for `Reg<MDMA_C18TCR_SPEC>`"]
pub type MDMA_C18TCR = crate::Reg<mdma_c18tcr::MDMA_C18TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c18tcr;
#[doc = "MDMA_C18BNDTR (rw) register accessor: an alias for `Reg<MDMA_C18BNDTR_SPEC>`"]
pub type MDMA_C18BNDTR = crate::Reg<mdma_c18bndtr::MDMA_C18BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c18bndtr;
#[doc = "MDMA_C18SAR (rw) register accessor: an alias for `Reg<MDMA_C18SAR_SPEC>`"]
pub type MDMA_C18SAR = crate::Reg<mdma_c18sar::MDMA_C18SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c18sar;
#[doc = "MDMA_C18DAR (rw) register accessor: an alias for `Reg<MDMA_C18DAR_SPEC>`"]
pub type MDMA_C18DAR = crate::Reg<mdma_c18dar::MDMA_C18DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c18dar;
#[doc = "MDMA_C18BRUR (rw) register accessor: an alias for `Reg<MDMA_C18BRUR_SPEC>`"]
pub type MDMA_C18BRUR = crate::Reg<mdma_c18brur::MDMA_C18BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c18brur;
#[doc = "MDMA_C18LAR (rw) register accessor: an alias for `Reg<MDMA_C18LAR_SPEC>`"]
pub type MDMA_C18LAR = crate::Reg<mdma_c18lar::MDMA_C18LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c18lar;
#[doc = "MDMA_C18TBR (rw) register accessor: an alias for `Reg<MDMA_C18TBR_SPEC>`"]
pub type MDMA_C18TBR = crate::Reg<mdma_c18tbr::MDMA_C18TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c18tbr;
#[doc = "MDMA_C18MAR (rw) register accessor: an alias for `Reg<MDMA_C18MAR_SPEC>`"]
pub type MDMA_C18MAR = crate::Reg<mdma_c18mar::MDMA_C18MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c18mar;
#[doc = "MDMA_C18MDR (rw) register accessor: an alias for `Reg<MDMA_C18MDR_SPEC>`"]
pub type MDMA_C18MDR = crate::Reg<mdma_c18mdr::MDMA_C18MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c18mdr;
#[doc = "MDMA_C19ISR (r) register accessor: an alias for `Reg<MDMA_C19ISR_SPEC>`"]
pub type MDMA_C19ISR = crate::Reg<mdma_c19isr::MDMA_C19ISR_SPEC>;
#[doc = "MDMA channel 19 interrupt/status register"]
pub mod mdma_c19isr;
#[doc = "MDMA_C19IFCR (w) register accessor: an alias for `Reg<MDMA_C19IFCR_SPEC>`"]
pub type MDMA_C19IFCR = crate::Reg<mdma_c19ifcr::MDMA_C19IFCR_SPEC>;
#[doc = "MDMA channel 19 interrupt flag clear register"]
pub mod mdma_c19ifcr;
#[doc = "MDMA_C19ESR (r) register accessor: an alias for `Reg<MDMA_C19ESR_SPEC>`"]
pub type MDMA_C19ESR = crate::Reg<mdma_c19esr::MDMA_C19ESR_SPEC>;
#[doc = "MDMA channel 19 error status register"]
pub mod mdma_c19esr;
#[doc = "MDMA_C19CR (rw) register accessor: an alias for `Reg<MDMA_C19CR_SPEC>`"]
pub type MDMA_C19CR = crate::Reg<mdma_c19cr::MDMA_C19CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c19cr;
#[doc = "MDMA_C19TCR (rw) register accessor: an alias for `Reg<MDMA_C19TCR_SPEC>`"]
pub type MDMA_C19TCR = crate::Reg<mdma_c19tcr::MDMA_C19TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c19tcr;
#[doc = "MDMA_C19BNDTR (rw) register accessor: an alias for `Reg<MDMA_C19BNDTR_SPEC>`"]
pub type MDMA_C19BNDTR = crate::Reg<mdma_c19bndtr::MDMA_C19BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c19bndtr;
#[doc = "MDMA_C19SAR (rw) register accessor: an alias for `Reg<MDMA_C19SAR_SPEC>`"]
pub type MDMA_C19SAR = crate::Reg<mdma_c19sar::MDMA_C19SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c19sar;
#[doc = "MDMA_C19DAR (rw) register accessor: an alias for `Reg<MDMA_C19DAR_SPEC>`"]
pub type MDMA_C19DAR = crate::Reg<mdma_c19dar::MDMA_C19DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c19dar;
#[doc = "MDMA_C19BRUR (rw) register accessor: an alias for `Reg<MDMA_C19BRUR_SPEC>`"]
pub type MDMA_C19BRUR = crate::Reg<mdma_c19brur::MDMA_C19BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c19brur;
#[doc = "MDMA_C19LAR (rw) register accessor: an alias for `Reg<MDMA_C19LAR_SPEC>`"]
pub type MDMA_C19LAR = crate::Reg<mdma_c19lar::MDMA_C19LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c19lar;
#[doc = "MDMA_C19TBR (rw) register accessor: an alias for `Reg<MDMA_C19TBR_SPEC>`"]
pub type MDMA_C19TBR = crate::Reg<mdma_c19tbr::MDMA_C19TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c19tbr;
#[doc = "MDMA_C19MAR (rw) register accessor: an alias for `Reg<MDMA_C19MAR_SPEC>`"]
pub type MDMA_C19MAR = crate::Reg<mdma_c19mar::MDMA_C19MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c19mar;
#[doc = "MDMA_C19MDR (rw) register accessor: an alias for `Reg<MDMA_C19MDR_SPEC>`"]
pub type MDMA_C19MDR = crate::Reg<mdma_c19mdr::MDMA_C19MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c19mdr;
#[doc = "MDMA_C20ISR (r) register accessor: an alias for `Reg<MDMA_C20ISR_SPEC>`"]
pub type MDMA_C20ISR = crate::Reg<mdma_c20isr::MDMA_C20ISR_SPEC>;
#[doc = "MDMA channel 20 interrupt/status register"]
pub mod mdma_c20isr;
#[doc = "MDMA_C20IFCR (w) register accessor: an alias for `Reg<MDMA_C20IFCR_SPEC>`"]
pub type MDMA_C20IFCR = crate::Reg<mdma_c20ifcr::MDMA_C20IFCR_SPEC>;
#[doc = "MDMA channel 20 interrupt flag clear register"]
pub mod mdma_c20ifcr;
#[doc = "MDMA_C20ESR (r) register accessor: an alias for `Reg<MDMA_C20ESR_SPEC>`"]
pub type MDMA_C20ESR = crate::Reg<mdma_c20esr::MDMA_C20ESR_SPEC>;
#[doc = "MDMA channel 20 error status register"]
pub mod mdma_c20esr;
#[doc = "MDMA_C20CR (rw) register accessor: an alias for `Reg<MDMA_C20CR_SPEC>`"]
pub type MDMA_C20CR = crate::Reg<mdma_c20cr::MDMA_C20CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c20cr;
#[doc = "MDMA_C20TCR (rw) register accessor: an alias for `Reg<MDMA_C20TCR_SPEC>`"]
pub type MDMA_C20TCR = crate::Reg<mdma_c20tcr::MDMA_C20TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c20tcr;
#[doc = "MDMA_C20BNDTR (rw) register accessor: an alias for `Reg<MDMA_C20BNDTR_SPEC>`"]
pub type MDMA_C20BNDTR = crate::Reg<mdma_c20bndtr::MDMA_C20BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c20bndtr;
#[doc = "MDMA_C20SAR (rw) register accessor: an alias for `Reg<MDMA_C20SAR_SPEC>`"]
pub type MDMA_C20SAR = crate::Reg<mdma_c20sar::MDMA_C20SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c20sar;
#[doc = "MDMA_C20DAR (rw) register accessor: an alias for `Reg<MDMA_C20DAR_SPEC>`"]
pub type MDMA_C20DAR = crate::Reg<mdma_c20dar::MDMA_C20DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c20dar;
#[doc = "MDMA_C20BRUR (rw) register accessor: an alias for `Reg<MDMA_C20BRUR_SPEC>`"]
pub type MDMA_C20BRUR = crate::Reg<mdma_c20brur::MDMA_C20BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c20brur;
#[doc = "MDMA_C20LAR (rw) register accessor: an alias for `Reg<MDMA_C20LAR_SPEC>`"]
pub type MDMA_C20LAR = crate::Reg<mdma_c20lar::MDMA_C20LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c20lar;
#[doc = "MDMA_C20TBR (rw) register accessor: an alias for `Reg<MDMA_C20TBR_SPEC>`"]
pub type MDMA_C20TBR = crate::Reg<mdma_c20tbr::MDMA_C20TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c20tbr;
#[doc = "MDMA_C20MAR (rw) register accessor: an alias for `Reg<MDMA_C20MAR_SPEC>`"]
pub type MDMA_C20MAR = crate::Reg<mdma_c20mar::MDMA_C20MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c20mar;
#[doc = "MDMA_C20MDR (rw) register accessor: an alias for `Reg<MDMA_C20MDR_SPEC>`"]
pub type MDMA_C20MDR = crate::Reg<mdma_c20mdr::MDMA_C20MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c20mdr;
#[doc = "MDMA_C21ISR (r) register accessor: an alias for `Reg<MDMA_C21ISR_SPEC>`"]
pub type MDMA_C21ISR = crate::Reg<mdma_c21isr::MDMA_C21ISR_SPEC>;
#[doc = "MDMA channel 21 interrupt/status register"]
pub mod mdma_c21isr;
#[doc = "MDMA_C21IFCR (w) register accessor: an alias for `Reg<MDMA_C21IFCR_SPEC>`"]
pub type MDMA_C21IFCR = crate::Reg<mdma_c21ifcr::MDMA_C21IFCR_SPEC>;
#[doc = "MDMA channel 21 interrupt flag clear register"]
pub mod mdma_c21ifcr;
#[doc = "MDMA_C21ESR (r) register accessor: an alias for `Reg<MDMA_C21ESR_SPEC>`"]
pub type MDMA_C21ESR = crate::Reg<mdma_c21esr::MDMA_C21ESR_SPEC>;
#[doc = "MDMA channel 21 error status register"]
pub mod mdma_c21esr;
#[doc = "MDMA_C21CR (rw) register accessor: an alias for `Reg<MDMA_C21CR_SPEC>`"]
pub type MDMA_C21CR = crate::Reg<mdma_c21cr::MDMA_C21CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c21cr;
#[doc = "MDMA_C21TCR (rw) register accessor: an alias for `Reg<MDMA_C21TCR_SPEC>`"]
pub type MDMA_C21TCR = crate::Reg<mdma_c21tcr::MDMA_C21TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c21tcr;
#[doc = "MDMA_C21BNDTR (rw) register accessor: an alias for `Reg<MDMA_C21BNDTR_SPEC>`"]
pub type MDMA_C21BNDTR = crate::Reg<mdma_c21bndtr::MDMA_C21BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c21bndtr;
#[doc = "MDMA_C21SAR (rw) register accessor: an alias for `Reg<MDMA_C21SAR_SPEC>`"]
pub type MDMA_C21SAR = crate::Reg<mdma_c21sar::MDMA_C21SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c21sar;
#[doc = "MDMA_C21DAR (rw) register accessor: an alias for `Reg<MDMA_C21DAR_SPEC>`"]
pub type MDMA_C21DAR = crate::Reg<mdma_c21dar::MDMA_C21DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c21dar;
#[doc = "MDMA_C21BRUR (rw) register accessor: an alias for `Reg<MDMA_C21BRUR_SPEC>`"]
pub type MDMA_C21BRUR = crate::Reg<mdma_c21brur::MDMA_C21BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c21brur;
#[doc = "MDMA_C21LAR (rw) register accessor: an alias for `Reg<MDMA_C21LAR_SPEC>`"]
pub type MDMA_C21LAR = crate::Reg<mdma_c21lar::MDMA_C21LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c21lar;
#[doc = "MDMA_C21TBR (rw) register accessor: an alias for `Reg<MDMA_C21TBR_SPEC>`"]
pub type MDMA_C21TBR = crate::Reg<mdma_c21tbr::MDMA_C21TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c21tbr;
#[doc = "MDMA_C21MAR (rw) register accessor: an alias for `Reg<MDMA_C21MAR_SPEC>`"]
pub type MDMA_C21MAR = crate::Reg<mdma_c21mar::MDMA_C21MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c21mar;
#[doc = "MDMA_C21MDR (rw) register accessor: an alias for `Reg<MDMA_C21MDR_SPEC>`"]
pub type MDMA_C21MDR = crate::Reg<mdma_c21mdr::MDMA_C21MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c21mdr;
#[doc = "MDMA_C22ISR (r) register accessor: an alias for `Reg<MDMA_C22ISR_SPEC>`"]
pub type MDMA_C22ISR = crate::Reg<mdma_c22isr::MDMA_C22ISR_SPEC>;
#[doc = "MDMA channel 22 interrupt/status register"]
pub mod mdma_c22isr;
#[doc = "MDMA_C22IFCR (w) register accessor: an alias for `Reg<MDMA_C22IFCR_SPEC>`"]
pub type MDMA_C22IFCR = crate::Reg<mdma_c22ifcr::MDMA_C22IFCR_SPEC>;
#[doc = "MDMA channel 22 interrupt flag clear register"]
pub mod mdma_c22ifcr;
#[doc = "MDMA_C22ESR (r) register accessor: an alias for `Reg<MDMA_C22ESR_SPEC>`"]
pub type MDMA_C22ESR = crate::Reg<mdma_c22esr::MDMA_C22ESR_SPEC>;
#[doc = "MDMA channel 22 error status register"]
pub mod mdma_c22esr;
#[doc = "MDMA_C22CR (rw) register accessor: an alias for `Reg<MDMA_C22CR_SPEC>`"]
pub type MDMA_C22CR = crate::Reg<mdma_c22cr::MDMA_C22CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c22cr;
#[doc = "MDMA_C22TCR (rw) register accessor: an alias for `Reg<MDMA_C22TCR_SPEC>`"]
pub type MDMA_C22TCR = crate::Reg<mdma_c22tcr::MDMA_C22TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c22tcr;
#[doc = "MDMA_C22BNDTR (rw) register accessor: an alias for `Reg<MDMA_C22BNDTR_SPEC>`"]
pub type MDMA_C22BNDTR = crate::Reg<mdma_c22bndtr::MDMA_C22BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c22bndtr;
#[doc = "MDMA_C22SAR (rw) register accessor: an alias for `Reg<MDMA_C22SAR_SPEC>`"]
pub type MDMA_C22SAR = crate::Reg<mdma_c22sar::MDMA_C22SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c22sar;
#[doc = "MDMA_C22DAR (rw) register accessor: an alias for `Reg<MDMA_C22DAR_SPEC>`"]
pub type MDMA_C22DAR = crate::Reg<mdma_c22dar::MDMA_C22DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c22dar;
#[doc = "MDMA_C22BRUR (rw) register accessor: an alias for `Reg<MDMA_C22BRUR_SPEC>`"]
pub type MDMA_C22BRUR = crate::Reg<mdma_c22brur::MDMA_C22BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c22brur;
#[doc = "MDMA_C22LAR (rw) register accessor: an alias for `Reg<MDMA_C22LAR_SPEC>`"]
pub type MDMA_C22LAR = crate::Reg<mdma_c22lar::MDMA_C22LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c22lar;
#[doc = "MDMA_C22TBR (rw) register accessor: an alias for `Reg<MDMA_C22TBR_SPEC>`"]
pub type MDMA_C22TBR = crate::Reg<mdma_c22tbr::MDMA_C22TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c22tbr;
#[doc = "MDMA_C22MAR (rw) register accessor: an alias for `Reg<MDMA_C22MAR_SPEC>`"]
pub type MDMA_C22MAR = crate::Reg<mdma_c22mar::MDMA_C22MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c22mar;
#[doc = "MDMA_C22MDR (rw) register accessor: an alias for `Reg<MDMA_C22MDR_SPEC>`"]
pub type MDMA_C22MDR = crate::Reg<mdma_c22mdr::MDMA_C22MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c22mdr;
#[doc = "MDMA_C23ISR (r) register accessor: an alias for `Reg<MDMA_C23ISR_SPEC>`"]
pub type MDMA_C23ISR = crate::Reg<mdma_c23isr::MDMA_C23ISR_SPEC>;
#[doc = "MDMA channel 23 interrupt/status register"]
pub mod mdma_c23isr;
#[doc = "MDMA_C23IFCR (w) register accessor: an alias for `Reg<MDMA_C23IFCR_SPEC>`"]
pub type MDMA_C23IFCR = crate::Reg<mdma_c23ifcr::MDMA_C23IFCR_SPEC>;
#[doc = "MDMA channel 23 interrupt flag clear register"]
pub mod mdma_c23ifcr;
#[doc = "MDMA_C23ESR (r) register accessor: an alias for `Reg<MDMA_C23ESR_SPEC>`"]
pub type MDMA_C23ESR = crate::Reg<mdma_c23esr::MDMA_C23ESR_SPEC>;
#[doc = "MDMA channel 23 error status register"]
pub mod mdma_c23esr;
#[doc = "MDMA_C23CR (rw) register accessor: an alias for `Reg<MDMA_C23CR_SPEC>`"]
pub type MDMA_C23CR = crate::Reg<mdma_c23cr::MDMA_C23CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c23cr;
#[doc = "MDMA_C23TCR (rw) register accessor: an alias for `Reg<MDMA_C23TCR_SPEC>`"]
pub type MDMA_C23TCR = crate::Reg<mdma_c23tcr::MDMA_C23TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c23tcr;
#[doc = "MDMA_C23BNDTR (rw) register accessor: an alias for `Reg<MDMA_C23BNDTR_SPEC>`"]
pub type MDMA_C23BNDTR = crate::Reg<mdma_c23bndtr::MDMA_C23BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c23bndtr;
#[doc = "MDMA_C23SAR (rw) register accessor: an alias for `Reg<MDMA_C23SAR_SPEC>`"]
pub type MDMA_C23SAR = crate::Reg<mdma_c23sar::MDMA_C23SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c23sar;
#[doc = "MDMA_C23DAR (rw) register accessor: an alias for `Reg<MDMA_C23DAR_SPEC>`"]
pub type MDMA_C23DAR = crate::Reg<mdma_c23dar::MDMA_C23DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c23dar;
#[doc = "MDMA_C23BRUR (rw) register accessor: an alias for `Reg<MDMA_C23BRUR_SPEC>`"]
pub type MDMA_C23BRUR = crate::Reg<mdma_c23brur::MDMA_C23BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c23brur;
#[doc = "MDMA_C23LAR (rw) register accessor: an alias for `Reg<MDMA_C23LAR_SPEC>`"]
pub type MDMA_C23LAR = crate::Reg<mdma_c23lar::MDMA_C23LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c23lar;
#[doc = "MDMA_C23TBR (rw) register accessor: an alias for `Reg<MDMA_C23TBR_SPEC>`"]
pub type MDMA_C23TBR = crate::Reg<mdma_c23tbr::MDMA_C23TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c23tbr;
#[doc = "MDMA_C23MAR (rw) register accessor: an alias for `Reg<MDMA_C23MAR_SPEC>`"]
pub type MDMA_C23MAR = crate::Reg<mdma_c23mar::MDMA_C23MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c23mar;
#[doc = "MDMA_C23MDR (rw) register accessor: an alias for `Reg<MDMA_C23MDR_SPEC>`"]
pub type MDMA_C23MDR = crate::Reg<mdma_c23mdr::MDMA_C23MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c23mdr;
#[doc = "MDMA_C24ISR (r) register accessor: an alias for `Reg<MDMA_C24ISR_SPEC>`"]
pub type MDMA_C24ISR = crate::Reg<mdma_c24isr::MDMA_C24ISR_SPEC>;
#[doc = "MDMA channel 24 interrupt/status register"]
pub mod mdma_c24isr;
#[doc = "MDMA_C24IFCR (w) register accessor: an alias for `Reg<MDMA_C24IFCR_SPEC>`"]
pub type MDMA_C24IFCR = crate::Reg<mdma_c24ifcr::MDMA_C24IFCR_SPEC>;
#[doc = "MDMA channel 24 interrupt flag clear register"]
pub mod mdma_c24ifcr;
#[doc = "MDMA_C24ESR (r) register accessor: an alias for `Reg<MDMA_C24ESR_SPEC>`"]
pub type MDMA_C24ESR = crate::Reg<mdma_c24esr::MDMA_C24ESR_SPEC>;
#[doc = "MDMA channel 24 error status register"]
pub mod mdma_c24esr;
#[doc = "MDMA_C24CR (rw) register accessor: an alias for `Reg<MDMA_C24CR_SPEC>`"]
pub type MDMA_C24CR = crate::Reg<mdma_c24cr::MDMA_C24CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c24cr;
#[doc = "MDMA_C24TCR (rw) register accessor: an alias for `Reg<MDMA_C24TCR_SPEC>`"]
pub type MDMA_C24TCR = crate::Reg<mdma_c24tcr::MDMA_C24TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c24tcr;
#[doc = "MDMA_C24BNDTR (rw) register accessor: an alias for `Reg<MDMA_C24BNDTR_SPEC>`"]
pub type MDMA_C24BNDTR = crate::Reg<mdma_c24bndtr::MDMA_C24BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c24bndtr;
#[doc = "MDMA_C24SAR (rw) register accessor: an alias for `Reg<MDMA_C24SAR_SPEC>`"]
pub type MDMA_C24SAR = crate::Reg<mdma_c24sar::MDMA_C24SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c24sar;
#[doc = "MDMA_C24DAR (rw) register accessor: an alias for `Reg<MDMA_C24DAR_SPEC>`"]
pub type MDMA_C24DAR = crate::Reg<mdma_c24dar::MDMA_C24DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c24dar;
#[doc = "MDMA_C24BRUR (rw) register accessor: an alias for `Reg<MDMA_C24BRUR_SPEC>`"]
pub type MDMA_C24BRUR = crate::Reg<mdma_c24brur::MDMA_C24BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c24brur;
#[doc = "MDMA_C24LAR (rw) register accessor: an alias for `Reg<MDMA_C24LAR_SPEC>`"]
pub type MDMA_C24LAR = crate::Reg<mdma_c24lar::MDMA_C24LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c24lar;
#[doc = "MDMA_C24TBR (rw) register accessor: an alias for `Reg<MDMA_C24TBR_SPEC>`"]
pub type MDMA_C24TBR = crate::Reg<mdma_c24tbr::MDMA_C24TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c24tbr;
#[doc = "MDMA_C24MAR (rw) register accessor: an alias for `Reg<MDMA_C24MAR_SPEC>`"]
pub type MDMA_C24MAR = crate::Reg<mdma_c24mar::MDMA_C24MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c24mar;
#[doc = "MDMA_C24MDR (rw) register accessor: an alias for `Reg<MDMA_C24MDR_SPEC>`"]
pub type MDMA_C24MDR = crate::Reg<mdma_c24mdr::MDMA_C24MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c24mdr;
#[doc = "MDMA_C25ISR (r) register accessor: an alias for `Reg<MDMA_C25ISR_SPEC>`"]
pub type MDMA_C25ISR = crate::Reg<mdma_c25isr::MDMA_C25ISR_SPEC>;
#[doc = "MDMA channel 25 interrupt/status register"]
pub mod mdma_c25isr;
#[doc = "MDMA_C25IFCR (w) register accessor: an alias for `Reg<MDMA_C25IFCR_SPEC>`"]
pub type MDMA_C25IFCR = crate::Reg<mdma_c25ifcr::MDMA_C25IFCR_SPEC>;
#[doc = "MDMA channel 25 interrupt flag clear register"]
pub mod mdma_c25ifcr;
#[doc = "MDMA_C25ESR (r) register accessor: an alias for `Reg<MDMA_C25ESR_SPEC>`"]
pub type MDMA_C25ESR = crate::Reg<mdma_c25esr::MDMA_C25ESR_SPEC>;
#[doc = "MDMA channel 25 error status register"]
pub mod mdma_c25esr;
#[doc = "MDMA_C25CR (rw) register accessor: an alias for `Reg<MDMA_C25CR_SPEC>`"]
pub type MDMA_C25CR = crate::Reg<mdma_c25cr::MDMA_C25CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c25cr;
#[doc = "MDMA_C25TCR (rw) register accessor: an alias for `Reg<MDMA_C25TCR_SPEC>`"]
pub type MDMA_C25TCR = crate::Reg<mdma_c25tcr::MDMA_C25TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c25tcr;
#[doc = "MDMA_C25BNDTR (rw) register accessor: an alias for `Reg<MDMA_C25BNDTR_SPEC>`"]
pub type MDMA_C25BNDTR = crate::Reg<mdma_c25bndtr::MDMA_C25BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c25bndtr;
#[doc = "MDMA_C25SAR (rw) register accessor: an alias for `Reg<MDMA_C25SAR_SPEC>`"]
pub type MDMA_C25SAR = crate::Reg<mdma_c25sar::MDMA_C25SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c25sar;
#[doc = "MDMA_C25DAR (rw) register accessor: an alias for `Reg<MDMA_C25DAR_SPEC>`"]
pub type MDMA_C25DAR = crate::Reg<mdma_c25dar::MDMA_C25DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c25dar;
#[doc = "MDMA_C25BRUR (rw) register accessor: an alias for `Reg<MDMA_C25BRUR_SPEC>`"]
pub type MDMA_C25BRUR = crate::Reg<mdma_c25brur::MDMA_C25BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c25brur;
#[doc = "MDMA_C25LAR (rw) register accessor: an alias for `Reg<MDMA_C25LAR_SPEC>`"]
pub type MDMA_C25LAR = crate::Reg<mdma_c25lar::MDMA_C25LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c25lar;
#[doc = "MDMA_C25TBR (rw) register accessor: an alias for `Reg<MDMA_C25TBR_SPEC>`"]
pub type MDMA_C25TBR = crate::Reg<mdma_c25tbr::MDMA_C25TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c25tbr;
#[doc = "MDMA_C25MAR (rw) register accessor: an alias for `Reg<MDMA_C25MAR_SPEC>`"]
pub type MDMA_C25MAR = crate::Reg<mdma_c25mar::MDMA_C25MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c25mar;
#[doc = "MDMA_C25MDR (rw) register accessor: an alias for `Reg<MDMA_C25MDR_SPEC>`"]
pub type MDMA_C25MDR = crate::Reg<mdma_c25mdr::MDMA_C25MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c25mdr;
#[doc = "MDMA_C26ISR (r) register accessor: an alias for `Reg<MDMA_C26ISR_SPEC>`"]
pub type MDMA_C26ISR = crate::Reg<mdma_c26isr::MDMA_C26ISR_SPEC>;
#[doc = "MDMA channel 26 interrupt/status register"]
pub mod mdma_c26isr;
#[doc = "MDMA_C26IFCR (w) register accessor: an alias for `Reg<MDMA_C26IFCR_SPEC>`"]
pub type MDMA_C26IFCR = crate::Reg<mdma_c26ifcr::MDMA_C26IFCR_SPEC>;
#[doc = "MDMA channel 26 interrupt flag clear register"]
pub mod mdma_c26ifcr;
#[doc = "MDMA_C26ESR (r) register accessor: an alias for `Reg<MDMA_C26ESR_SPEC>`"]
pub type MDMA_C26ESR = crate::Reg<mdma_c26esr::MDMA_C26ESR_SPEC>;
#[doc = "MDMA channel 26 error status register"]
pub mod mdma_c26esr;
#[doc = "MDMA_C26CR (rw) register accessor: an alias for `Reg<MDMA_C26CR_SPEC>`"]
pub type MDMA_C26CR = crate::Reg<mdma_c26cr::MDMA_C26CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c26cr;
#[doc = "MDMA_C26TCR (rw) register accessor: an alias for `Reg<MDMA_C26TCR_SPEC>`"]
pub type MDMA_C26TCR = crate::Reg<mdma_c26tcr::MDMA_C26TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c26tcr;
#[doc = "MDMA_C26BNDTR (rw) register accessor: an alias for `Reg<MDMA_C26BNDTR_SPEC>`"]
pub type MDMA_C26BNDTR = crate::Reg<mdma_c26bndtr::MDMA_C26BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c26bndtr;
#[doc = "MDMA_C26SAR (rw) register accessor: an alias for `Reg<MDMA_C26SAR_SPEC>`"]
pub type MDMA_C26SAR = crate::Reg<mdma_c26sar::MDMA_C26SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c26sar;
#[doc = "MDMA_C26DAR (rw) register accessor: an alias for `Reg<MDMA_C26DAR_SPEC>`"]
pub type MDMA_C26DAR = crate::Reg<mdma_c26dar::MDMA_C26DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c26dar;
#[doc = "MDMA_C26BRUR (rw) register accessor: an alias for `Reg<MDMA_C26BRUR_SPEC>`"]
pub type MDMA_C26BRUR = crate::Reg<mdma_c26brur::MDMA_C26BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c26brur;
#[doc = "MDMA_C26LAR (rw) register accessor: an alias for `Reg<MDMA_C26LAR_SPEC>`"]
pub type MDMA_C26LAR = crate::Reg<mdma_c26lar::MDMA_C26LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c26lar;
#[doc = "MDMA_C26TBR (rw) register accessor: an alias for `Reg<MDMA_C26TBR_SPEC>`"]
pub type MDMA_C26TBR = crate::Reg<mdma_c26tbr::MDMA_C26TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c26tbr;
#[doc = "MDMA_C26MAR (rw) register accessor: an alias for `Reg<MDMA_C26MAR_SPEC>`"]
pub type MDMA_C26MAR = crate::Reg<mdma_c26mar::MDMA_C26MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c26mar;
#[doc = "MDMA_C26MDR (rw) register accessor: an alias for `Reg<MDMA_C26MDR_SPEC>`"]
pub type MDMA_C26MDR = crate::Reg<mdma_c26mdr::MDMA_C26MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c26mdr;
#[doc = "MDMA_C27ISR (r) register accessor: an alias for `Reg<MDMA_C27ISR_SPEC>`"]
pub type MDMA_C27ISR = crate::Reg<mdma_c27isr::MDMA_C27ISR_SPEC>;
#[doc = "MDMA channel 27 interrupt/status register"]
pub mod mdma_c27isr;
#[doc = "MDMA_C27IFCR (w) register accessor: an alias for `Reg<MDMA_C27IFCR_SPEC>`"]
pub type MDMA_C27IFCR = crate::Reg<mdma_c27ifcr::MDMA_C27IFCR_SPEC>;
#[doc = "MDMA channel 27 interrupt flag clear register"]
pub mod mdma_c27ifcr;
#[doc = "MDMA_C27ESR (r) register accessor: an alias for `Reg<MDMA_C27ESR_SPEC>`"]
pub type MDMA_C27ESR = crate::Reg<mdma_c27esr::MDMA_C27ESR_SPEC>;
#[doc = "MDMA channel 27 error status register"]
pub mod mdma_c27esr;
#[doc = "MDMA_C27CR (rw) register accessor: an alias for `Reg<MDMA_C27CR_SPEC>`"]
pub type MDMA_C27CR = crate::Reg<mdma_c27cr::MDMA_C27CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c27cr;
#[doc = "MDMA_C27TCR (rw) register accessor: an alias for `Reg<MDMA_C27TCR_SPEC>`"]
pub type MDMA_C27TCR = crate::Reg<mdma_c27tcr::MDMA_C27TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c27tcr;
#[doc = "MDMA_C27BNDTR (rw) register accessor: an alias for `Reg<MDMA_C27BNDTR_SPEC>`"]
pub type MDMA_C27BNDTR = crate::Reg<mdma_c27bndtr::MDMA_C27BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c27bndtr;
#[doc = "MDMA_C27SAR (rw) register accessor: an alias for `Reg<MDMA_C27SAR_SPEC>`"]
pub type MDMA_C27SAR = crate::Reg<mdma_c27sar::MDMA_C27SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c27sar;
#[doc = "MDMA_C27DAR (rw) register accessor: an alias for `Reg<MDMA_C27DAR_SPEC>`"]
pub type MDMA_C27DAR = crate::Reg<mdma_c27dar::MDMA_C27DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c27dar;
#[doc = "MDMA_C27BRUR (rw) register accessor: an alias for `Reg<MDMA_C27BRUR_SPEC>`"]
pub type MDMA_C27BRUR = crate::Reg<mdma_c27brur::MDMA_C27BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c27brur;
#[doc = "MDMA_C27LAR (rw) register accessor: an alias for `Reg<MDMA_C27LAR_SPEC>`"]
pub type MDMA_C27LAR = crate::Reg<mdma_c27lar::MDMA_C27LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c27lar;
#[doc = "MDMA_C27TBR (rw) register accessor: an alias for `Reg<MDMA_C27TBR_SPEC>`"]
pub type MDMA_C27TBR = crate::Reg<mdma_c27tbr::MDMA_C27TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c27tbr;
#[doc = "MDMA_C27MAR (rw) register accessor: an alias for `Reg<MDMA_C27MAR_SPEC>`"]
pub type MDMA_C27MAR = crate::Reg<mdma_c27mar::MDMA_C27MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c27mar;
#[doc = "MDMA_C27MDR (rw) register accessor: an alias for `Reg<MDMA_C27MDR_SPEC>`"]
pub type MDMA_C27MDR = crate::Reg<mdma_c27mdr::MDMA_C27MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c27mdr;
#[doc = "MDMA_C28ISR (r) register accessor: an alias for `Reg<MDMA_C28ISR_SPEC>`"]
pub type MDMA_C28ISR = crate::Reg<mdma_c28isr::MDMA_C28ISR_SPEC>;
#[doc = "MDMA channel 28 interrupt/status register"]
pub mod mdma_c28isr;
#[doc = "MDMA_C28IFCR (w) register accessor: an alias for `Reg<MDMA_C28IFCR_SPEC>`"]
pub type MDMA_C28IFCR = crate::Reg<mdma_c28ifcr::MDMA_C28IFCR_SPEC>;
#[doc = "MDMA channel 28 interrupt flag clear register"]
pub mod mdma_c28ifcr;
#[doc = "MDMA_C28ESR (r) register accessor: an alias for `Reg<MDMA_C28ESR_SPEC>`"]
pub type MDMA_C28ESR = crate::Reg<mdma_c28esr::MDMA_C28ESR_SPEC>;
#[doc = "MDMA channel 28 error status register"]
pub mod mdma_c28esr;
#[doc = "MDMA_C28CR (rw) register accessor: an alias for `Reg<MDMA_C28CR_SPEC>`"]
pub type MDMA_C28CR = crate::Reg<mdma_c28cr::MDMA_C28CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c28cr;
#[doc = "MDMA_C28TCR (rw) register accessor: an alias for `Reg<MDMA_C28TCR_SPEC>`"]
pub type MDMA_C28TCR = crate::Reg<mdma_c28tcr::MDMA_C28TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c28tcr;
#[doc = "MDMA_C28BNDTR (rw) register accessor: an alias for `Reg<MDMA_C28BNDTR_SPEC>`"]
pub type MDMA_C28BNDTR = crate::Reg<mdma_c28bndtr::MDMA_C28BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c28bndtr;
#[doc = "MDMA_C28SAR (rw) register accessor: an alias for `Reg<MDMA_C28SAR_SPEC>`"]
pub type MDMA_C28SAR = crate::Reg<mdma_c28sar::MDMA_C28SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c28sar;
#[doc = "MDMA_C28DAR (rw) register accessor: an alias for `Reg<MDMA_C28DAR_SPEC>`"]
pub type MDMA_C28DAR = crate::Reg<mdma_c28dar::MDMA_C28DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c28dar;
#[doc = "MDMA_C28BRUR (rw) register accessor: an alias for `Reg<MDMA_C28BRUR_SPEC>`"]
pub type MDMA_C28BRUR = crate::Reg<mdma_c28brur::MDMA_C28BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c28brur;
#[doc = "MDMA_C28LAR (rw) register accessor: an alias for `Reg<MDMA_C28LAR_SPEC>`"]
pub type MDMA_C28LAR = crate::Reg<mdma_c28lar::MDMA_C28LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c28lar;
#[doc = "MDMA_C28TBR (rw) register accessor: an alias for `Reg<MDMA_C28TBR_SPEC>`"]
pub type MDMA_C28TBR = crate::Reg<mdma_c28tbr::MDMA_C28TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c28tbr;
#[doc = "MDMA_C28MAR (rw) register accessor: an alias for `Reg<MDMA_C28MAR_SPEC>`"]
pub type MDMA_C28MAR = crate::Reg<mdma_c28mar::MDMA_C28MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c28mar;
#[doc = "MDMA_C28MDR (rw) register accessor: an alias for `Reg<MDMA_C28MDR_SPEC>`"]
pub type MDMA_C28MDR = crate::Reg<mdma_c28mdr::MDMA_C28MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c28mdr;
#[doc = "MDMA_C29ISR (r) register accessor: an alias for `Reg<MDMA_C29ISR_SPEC>`"]
pub type MDMA_C29ISR = crate::Reg<mdma_c29isr::MDMA_C29ISR_SPEC>;
#[doc = "MDMA channel 29 interrupt/status register"]
pub mod mdma_c29isr;
#[doc = "MDMA_C29IFCR (w) register accessor: an alias for `Reg<MDMA_C29IFCR_SPEC>`"]
pub type MDMA_C29IFCR = crate::Reg<mdma_c29ifcr::MDMA_C29IFCR_SPEC>;
#[doc = "MDMA channel 29 interrupt flag clear register"]
pub mod mdma_c29ifcr;
#[doc = "MDMA_C29ESR (r) register accessor: an alias for `Reg<MDMA_C29ESR_SPEC>`"]
pub type MDMA_C29ESR = crate::Reg<mdma_c29esr::MDMA_C29ESR_SPEC>;
#[doc = "MDMA channel 29 error status register"]
pub mod mdma_c29esr;
#[doc = "MDMA_C29CR (rw) register accessor: an alias for `Reg<MDMA_C29CR_SPEC>`"]
pub type MDMA_C29CR = crate::Reg<mdma_c29cr::MDMA_C29CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c29cr;
#[doc = "MDMA_C29TCR (rw) register accessor: an alias for `Reg<MDMA_C29TCR_SPEC>`"]
pub type MDMA_C29TCR = crate::Reg<mdma_c29tcr::MDMA_C29TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c29tcr;
#[doc = "MDMA_C29BNDTR (rw) register accessor: an alias for `Reg<MDMA_C29BNDTR_SPEC>`"]
pub type MDMA_C29BNDTR = crate::Reg<mdma_c29bndtr::MDMA_C29BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c29bndtr;
#[doc = "MDMA_C29SAR (rw) register accessor: an alias for `Reg<MDMA_C29SAR_SPEC>`"]
pub type MDMA_C29SAR = crate::Reg<mdma_c29sar::MDMA_C29SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c29sar;
#[doc = "MDMA_C29DAR (rw) register accessor: an alias for `Reg<MDMA_C29DAR_SPEC>`"]
pub type MDMA_C29DAR = crate::Reg<mdma_c29dar::MDMA_C29DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c29dar;
#[doc = "MDMA_C29BRUR (rw) register accessor: an alias for `Reg<MDMA_C29BRUR_SPEC>`"]
pub type MDMA_C29BRUR = crate::Reg<mdma_c29brur::MDMA_C29BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c29brur;
#[doc = "MDMA_C29LAR (rw) register accessor: an alias for `Reg<MDMA_C29LAR_SPEC>`"]
pub type MDMA_C29LAR = crate::Reg<mdma_c29lar::MDMA_C29LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c29lar;
#[doc = "MDMA_C29TBR (rw) register accessor: an alias for `Reg<MDMA_C29TBR_SPEC>`"]
pub type MDMA_C29TBR = crate::Reg<mdma_c29tbr::MDMA_C29TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c29tbr;
#[doc = "MDMA_C29MAR (rw) register accessor: an alias for `Reg<MDMA_C29MAR_SPEC>`"]
pub type MDMA_C29MAR = crate::Reg<mdma_c29mar::MDMA_C29MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c29mar;
#[doc = "MDMA_C29MDR (rw) register accessor: an alias for `Reg<MDMA_C29MDR_SPEC>`"]
pub type MDMA_C29MDR = crate::Reg<mdma_c29mdr::MDMA_C29MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c29mdr;
#[doc = "MDMA_C30ISR (r) register accessor: an alias for `Reg<MDMA_C30ISR_SPEC>`"]
pub type MDMA_C30ISR = crate::Reg<mdma_c30isr::MDMA_C30ISR_SPEC>;
#[doc = "MDMA channel 30 interrupt/status register"]
pub mod mdma_c30isr;
#[doc = "MDMA_C30IFCR (w) register accessor: an alias for `Reg<MDMA_C30IFCR_SPEC>`"]
pub type MDMA_C30IFCR = crate::Reg<mdma_c30ifcr::MDMA_C30IFCR_SPEC>;
#[doc = "MDMA channel 30 interrupt flag clear register"]
pub mod mdma_c30ifcr;
#[doc = "MDMA_C30ESR (r) register accessor: an alias for `Reg<MDMA_C30ESR_SPEC>`"]
pub type MDMA_C30ESR = crate::Reg<mdma_c30esr::MDMA_C30ESR_SPEC>;
#[doc = "MDMA channel 30 error status register"]
pub mod mdma_c30esr;
#[doc = "MDMA_C30CR (rw) register accessor: an alias for `Reg<MDMA_C30CR_SPEC>`"]
pub type MDMA_C30CR = crate::Reg<mdma_c30cr::MDMA_C30CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c30cr;
#[doc = "MDMA_C30TCR (rw) register accessor: an alias for `Reg<MDMA_C30TCR_SPEC>`"]
pub type MDMA_C30TCR = crate::Reg<mdma_c30tcr::MDMA_C30TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c30tcr;
#[doc = "MDMA_C30BNDTR (rw) register accessor: an alias for `Reg<MDMA_C30BNDTR_SPEC>`"]
pub type MDMA_C30BNDTR = crate::Reg<mdma_c30bndtr::MDMA_C30BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c30bndtr;
#[doc = "MDMA_C30SAR (rw) register accessor: an alias for `Reg<MDMA_C30SAR_SPEC>`"]
pub type MDMA_C30SAR = crate::Reg<mdma_c30sar::MDMA_C30SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c30sar;
#[doc = "MDMA_C30DAR (rw) register accessor: an alias for `Reg<MDMA_C30DAR_SPEC>`"]
pub type MDMA_C30DAR = crate::Reg<mdma_c30dar::MDMA_C30DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c30dar;
#[doc = "MDMA_C30BRUR (rw) register accessor: an alias for `Reg<MDMA_C30BRUR_SPEC>`"]
pub type MDMA_C30BRUR = crate::Reg<mdma_c30brur::MDMA_C30BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c30brur;
#[doc = "MDMA_C30LAR (rw) register accessor: an alias for `Reg<MDMA_C30LAR_SPEC>`"]
pub type MDMA_C30LAR = crate::Reg<mdma_c30lar::MDMA_C30LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c30lar;
#[doc = "MDMA_C30TBR (rw) register accessor: an alias for `Reg<MDMA_C30TBR_SPEC>`"]
pub type MDMA_C30TBR = crate::Reg<mdma_c30tbr::MDMA_C30TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c30tbr;
#[doc = "MDMA_C30MAR (rw) register accessor: an alias for `Reg<MDMA_C30MAR_SPEC>`"]
pub type MDMA_C30MAR = crate::Reg<mdma_c30mar::MDMA_C30MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c30mar;
#[doc = "MDMA_C30MDR (rw) register accessor: an alias for `Reg<MDMA_C30MDR_SPEC>`"]
pub type MDMA_C30MDR = crate::Reg<mdma_c30mdr::MDMA_C30MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c30mdr;
#[doc = "MDMA_C31ISR (r) register accessor: an alias for `Reg<MDMA_C31ISR_SPEC>`"]
pub type MDMA_C31ISR = crate::Reg<mdma_c31isr::MDMA_C31ISR_SPEC>;
#[doc = "MDMA channel 31 interrupt/status register"]
pub mod mdma_c31isr;
#[doc = "MDMA_C31IFCR (w) register accessor: an alias for `Reg<MDMA_C31IFCR_SPEC>`"]
pub type MDMA_C31IFCR = crate::Reg<mdma_c31ifcr::MDMA_C31IFCR_SPEC>;
#[doc = "MDMA channel 31 interrupt flag clear register"]
pub mod mdma_c31ifcr;
#[doc = "MDMA_C31ESR (r) register accessor: an alias for `Reg<MDMA_C31ESR_SPEC>`"]
pub type MDMA_C31ESR = crate::Reg<mdma_c31esr::MDMA_C31ESR_SPEC>;
#[doc = "MDMA channel 31 error status register"]
pub mod mdma_c31esr;
#[doc = "MDMA_C31CR (rw) register accessor: an alias for `Reg<MDMA_C31CR_SPEC>`"]
pub type MDMA_C31CR = crate::Reg<mdma_c31cr::MDMA_C31CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod mdma_c31cr;
#[doc = "MDMA_C31TCR (rw) register accessor: an alias for `Reg<MDMA_C31TCR_SPEC>`"]
pub type MDMA_C31TCR = crate::Reg<mdma_c31tcr::MDMA_C31TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub mod mdma_c31tcr;
#[doc = "MDMA_C31BNDTR (rw) register accessor: an alias for `Reg<MDMA_C31BNDTR_SPEC>`"]
pub type MDMA_C31BNDTR = crate::Reg<mdma_c31bndtr::MDMA_C31BNDTR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04)."]
pub mod mdma_c31bndtr;
#[doc = "MDMA_C31SAR (rw) register accessor: an alias for `Reg<MDMA_C31SAR_SPEC>`"]
pub type MDMA_C31SAR = crate::Reg<mdma_c31sar::MDMA_C31SAR_SPEC>;
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08)."]
pub mod mdma_c31sar;
#[doc = "MDMA_C31DAR (rw) register accessor: an alias for `Reg<MDMA_C31DAR_SPEC>`"]
pub type MDMA_C31DAR = crate::Reg<mdma_c31dar::MDMA_C31DAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M"]
pub mod mdma_c31dar;
#[doc = "MDMA_C31BRUR (rw) register accessor: an alias for `Reg<MDMA_C31BRUR_SPEC>`"]
pub type MDMA_C31BRUR = crate::Reg<mdma_c31brur::MDMA_C31BRUR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10)."]
pub mod mdma_c31brur;
#[doc = "MDMA_C31LAR (rw) register accessor: an alias for `Reg<MDMA_C31LAR_SPEC>`"]
pub type MDMA_C31LAR = crate::Reg<mdma_c31lar::MDMA_C31LAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block."]
pub mod mdma_c31lar;
#[doc = "MDMA_C31TBR (rw) register accessor: an alias for `Reg<MDMA_C31TBR_SPEC>`"]
pub type MDMA_C31TBR = crate::Reg<mdma_c31tbr::MDMA_C31TBR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18)."]
pub mod mdma_c31tbr;
#[doc = "MDMA_C31MAR (rw) register accessor: an alias for `Reg<MDMA_C31MAR_SPEC>`"]
pub type MDMA_C31MAR = crate::Reg<mdma_c31mar::MDMA_C31MAR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x20)."]
pub mod mdma_c31mar;
#[doc = "MDMA_C31MDR (rw) register accessor: an alias for `Reg<MDMA_C31MDR_SPEC>`"]
pub type MDMA_C31MDR = crate::Reg<mdma_c31mdr::MDMA_C31MDR_SPEC>;
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24)."]
pub mod mdma_c31mdr;
