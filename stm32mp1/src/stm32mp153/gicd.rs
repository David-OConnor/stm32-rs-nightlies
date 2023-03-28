#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICD control register"]
    pub gicd_ctlr: GICD_CTLR,
    #[doc = "0x04 - GICD interrupt controller type register"]
    pub gicd_typer: GICD_TYPER,
    #[doc = "0x08 - GICD implementer identification register"]
    pub gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 0x74],
    #[doc = "0x80 - For interrupts ID"]
    pub gicd_igroupr0: GICD_IGROUPR0,
    #[doc = "0x84 - For interrupts ID"]
    pub gicd_igroupr1: GICD_IGROUPR1,
    #[doc = "0x88 - For interrupts ID"]
    pub gicd_igroupr2: GICD_IGROUPR2,
    #[doc = "0x8c - For interrupts ID = x*32 to ID = x*32+31"]
    pub gicd_igroupr3: GICD_IGROUPR3,
    #[doc = "0x90 - For interrupts ID = x*32 to ID = x*32+31"]
    pub gicd_igroupr4: GICD_IGROUPR4,
    #[doc = "0x94 - For interrupts ID"]
    pub gicd_igroupr5: GICD_IGROUPR5,
    #[doc = "0x98 - For interrupts ID"]
    pub gicd_igroupr6: GICD_IGROUPR6,
    #[doc = "0x9c - For interrupts ID"]
    pub gicd_igroupr7: GICD_IGROUPR7,
    #[doc = "0xa0 - For interrupts ID"]
    pub gicd_igroupr8: GICD_IGROUPR8,
    _reserved12: [u8; 0x5c],
    #[doc = "0x100 - For interrupts ID = 0 to ID = 31"]
    pub gicd_isenabler0: GICD_ISENABLER0,
    #[doc = "0x104 - For interrupts ID"]
    pub gicd_isenabler1: GICD_ISENABLER1,
    #[doc = "0x108 - For interrupts ID"]
    pub gicd_isenabler2: GICD_ISENABLER2,
    #[doc = "0x10c - For interrupts ID"]
    pub gicd_isenabler3: GICD_ISENABLER3,
    #[doc = "0x110 - For interrupts ID"]
    pub gicd_isenabler4: GICD_ISENABLER4,
    #[doc = "0x114 - For interrupts ID"]
    pub gicd_isenabler5: GICD_ISENABLER5,
    #[doc = "0x118 - For interrupts ID"]
    pub gicd_isenabler6: GICD_ISENABLER6,
    #[doc = "0x11c - For interrupts ID"]
    pub gicd_isenabler7: GICD_ISENABLER7,
    #[doc = "0x120 - For interrupts ID"]
    pub gicd_isenabler8: GICD_ISENABLER8,
    _reserved21: [u8; 0x5c],
    #[doc = "0x180 - For interrupts ID = 0 to ID = 31"]
    pub gicd_icenabler0: GICD_ICENABLER0,
    #[doc = "0x184 - For interrupts ID"]
    pub gicd_icenabler1: GICD_ICENABLER1,
    #[doc = "0x188 - For interrupts ID"]
    pub gicd_icenabler2: GICD_ICENABLER2,
    #[doc = "0x18c - For interrupts ID"]
    pub gicd_icenabler3: GICD_ICENABLER3,
    #[doc = "0x190 - For interrupts ID"]
    pub gicd_icenabler4: GICD_ICENABLER4,
    #[doc = "0x194 - For interrupts ID"]
    pub gicd_icenabler5: GICD_ICENABLER5,
    #[doc = "0x198 - For interrupts ID"]
    pub gicd_icenabler6: GICD_ICENABLER6,
    #[doc = "0x19c - For interrupts ID"]
    pub gicd_icenabler7: GICD_ICENABLER7,
    #[doc = "0x1a0 - For interrupts ID"]
    pub gicd_icenabler8: GICD_ICENABLER8,
    _reserved30: [u8; 0x5c],
    #[doc = "0x200 - For interrupts ID"]
    pub gicd_ispendr0: GICD_ISPENDR0,
    #[doc = "0x204 - For interrupts ID"]
    pub gicd_ispendr1: GICD_ISPENDR1,
    #[doc = "0x208 - For interrupts ID"]
    pub gicd_ispendr2: GICD_ISPENDR2,
    #[doc = "0x20c - For interrupts ID"]
    pub gicd_ispendr3: GICD_ISPENDR3,
    #[doc = "0x210 - For interrupts ID"]
    pub gicd_ispendr4: GICD_ISPENDR4,
    #[doc = "0x214 - For interrupts ID"]
    pub gicd_ispendr5: GICD_ISPENDR5,
    #[doc = "0x218 - For interrupts ID"]
    pub gicd_ispendr6: GICD_ISPENDR6,
    #[doc = "0x21c - For interrupts ID"]
    pub gicd_ispendr7: GICD_ISPENDR7,
    #[doc = "0x220 - For interrupts ID"]
    pub gicd_ispendr8: GICD_ISPENDR8,
    _reserved39: [u8; 0x5c],
    #[doc = "0x280 - For interrupts ID"]
    pub gicd_icpendr0: GICD_ICPENDR0,
    #[doc = "0x284 - For interrupts ID"]
    pub gicd_icpendr1: GICD_ICPENDR1,
    #[doc = "0x288 - For interrupts ID"]
    pub gicd_icpendr2: GICD_ICPENDR2,
    #[doc = "0x28c - For interrupts ID"]
    pub gicd_icpendr3: GICD_ICPENDR3,
    #[doc = "0x290 - For interrupts ID"]
    pub gicd_icpendr4: GICD_ICPENDR4,
    #[doc = "0x294 - For interrupts ID"]
    pub gicd_icpendr5: GICD_ICPENDR5,
    #[doc = "0x298 - For interrupts ID"]
    pub gicd_icpendr6: GICD_ICPENDR6,
    #[doc = "0x29c - For interrupts ID"]
    pub gicd_icpendr7: GICD_ICPENDR7,
    #[doc = "0x2a0 - For interrupts ID"]
    pub gicd_icpendr8: GICD_ICPENDR8,
    _reserved48: [u8; 0x5c],
    #[doc = "0x300 - For interrupts ID"]
    pub gicd_isactiver0: GICD_ISACTIVER0,
    #[doc = "0x304 - For interrupts ID"]
    pub gicd_isactiver1: GICD_ISACTIVER1,
    #[doc = "0x308 - For interrupts ID"]
    pub gicd_isactiver2: GICD_ISACTIVER2,
    #[doc = "0x30c - For interrupts ID"]
    pub gicd_isactiver3: GICD_ISACTIVER3,
    #[doc = "0x310 - For interrupts ID"]
    pub gicd_isactiver4: GICD_ISACTIVER4,
    #[doc = "0x314 - For interrupts ID"]
    pub gicd_isactiver5: GICD_ISACTIVER5,
    #[doc = "0x318 - For interrupts ID"]
    pub gicd_isactiver6: GICD_ISACTIVER6,
    #[doc = "0x31c - For interrupts ID"]
    pub gicd_isactiver7: GICD_ISACTIVER7,
    #[doc = "0x320 - For interrupts ID"]
    pub gicd_isactiver8: GICD_ISACTIVER8,
    _reserved57: [u8; 0x5c],
    #[doc = "0x380 - For interrupts ID"]
    pub gicd_icactiver0: GICD_ICACTIVER0,
    #[doc = "0x384 - For interrupts ID"]
    pub gicd_icactiver1: GICD_ICACTIVER1,
    #[doc = "0x388 - For interrupts ID"]
    pub gicd_icactiver2: GICD_ICACTIVER2,
    #[doc = "0x38c - For interrupts ID"]
    pub gicd_icactiver3: GICD_ICACTIVER3,
    #[doc = "0x390 - For interrupts ID"]
    pub gicd_icactiver4: GICD_ICACTIVER4,
    #[doc = "0x394 - For interrupts ID"]
    pub gicd_icactiver5: GICD_ICACTIVER5,
    #[doc = "0x398 - For interrupts ID"]
    pub gicd_icactiver6: GICD_ICACTIVER6,
    #[doc = "0x39c - For interrupts ID"]
    pub gicd_icactiver7: GICD_ICACTIVER7,
    #[doc = "0x3a0 - For interrupts ID"]
    pub gicd_icactiver8: GICD_ICACTIVER8,
    _reserved66: [u8; 0x5c],
    #[doc = "0x400 - GICD interrupt priority register 0"]
    pub gicd_ipriorityr0: GICD_IPRIORITYR0,
    #[doc = "0x404 - GICD interrupt priority register 1"]
    pub gicd_ipriorityr1: GICD_IPRIORITYR1,
    #[doc = "0x408 - GICD interrupt priority register 2"]
    pub gicd_ipriorityr2: GICD_IPRIORITYR2,
    #[doc = "0x40c - GICD interrupt priority register 3"]
    pub gicd_ipriorityr3: GICD_IPRIORITYR3,
    #[doc = "0x410 - GICD interrupt priority register 4"]
    pub gicd_ipriorityr4: GICD_IPRIORITYR4,
    #[doc = "0x414 - GICD interrupt priority register 5"]
    pub gicd_ipriorityr5: GICD_IPRIORITYR5,
    #[doc = "0x418 - GICD interrupt priority register 6"]
    pub gicd_ipriorityr6: GICD_IPRIORITYR6,
    #[doc = "0x41c - GICD interrupt priority register 7"]
    pub gicd_ipriorityr7: GICD_IPRIORITYR7,
    #[doc = "0x420 - GICD interrupt priority register 8"]
    pub gicd_ipriorityr8: GICD_IPRIORITYR8,
    #[doc = "0x424 - GICD interrupt priority register 9"]
    pub gicd_ipriorityr9: GICD_IPRIORITYR9,
    #[doc = "0x428 - GICD interrupt priority register 10"]
    pub gicd_ipriorityr10: GICD_IPRIORITYR10,
    #[doc = "0x42c - GICD interrupt priority register 11"]
    pub gicd_ipriorityr11: GICD_IPRIORITYR11,
    #[doc = "0x430 - GICD interrupt priority register 12"]
    pub gicd_ipriorityr12: GICD_IPRIORITYR12,
    #[doc = "0x434 - GICD interrupt priority register 13"]
    pub gicd_ipriorityr13: GICD_IPRIORITYR13,
    #[doc = "0x438 - GICD interrupt priority register 14"]
    pub gicd_ipriorityr14: GICD_IPRIORITYR14,
    #[doc = "0x43c - GICD interrupt priority register 15"]
    pub gicd_ipriorityr15: GICD_IPRIORITYR15,
    #[doc = "0x440 - GICD interrupt priority register 16"]
    pub gicd_ipriorityr16: GICD_IPRIORITYR16,
    #[doc = "0x444 - GICD interrupt priority register 17"]
    pub gicd_ipriorityr17: GICD_IPRIORITYR17,
    #[doc = "0x448 - GICD interrupt priority register 18"]
    pub gicd_ipriorityr18: GICD_IPRIORITYR18,
    #[doc = "0x44c - GICD interrupt priority register 19"]
    pub gicd_ipriorityr19: GICD_IPRIORITYR19,
    #[doc = "0x450 - GICD interrupt priority register 20"]
    pub gicd_ipriorityr20: GICD_IPRIORITYR20,
    #[doc = "0x454 - GICD interrupt priority register 21"]
    pub gicd_ipriorityr21: GICD_IPRIORITYR21,
    #[doc = "0x458 - GICD interrupt priority register 22"]
    pub gicd_ipriorityr22: GICD_IPRIORITYR22,
    #[doc = "0x45c - GICD interrupt priority register 23"]
    pub gicd_ipriorityr23: GICD_IPRIORITYR23,
    #[doc = "0x460 - GICD interrupt priority register 24"]
    pub gicd_ipriorityr24: GICD_IPRIORITYR24,
    #[doc = "0x464 - GICD interrupt priority register 25"]
    pub gicd_ipriorityr25: GICD_IPRIORITYR25,
    #[doc = "0x468 - GICD interrupt priority register 26"]
    pub gicd_ipriorityr26: GICD_IPRIORITYR26,
    #[doc = "0x46c - GICD interrupt priority register 27"]
    pub gicd_ipriorityr27: GICD_IPRIORITYR27,
    #[doc = "0x470 - GICD interrupt priority register 28"]
    pub gicd_ipriorityr28: GICD_IPRIORITYR28,
    #[doc = "0x474 - GICD interrupt priority register 29"]
    pub gicd_ipriorityr29: GICD_IPRIORITYR29,
    #[doc = "0x478 - GICD interrupt priority register 30"]
    pub gicd_ipriorityr30: GICD_IPRIORITYR30,
    #[doc = "0x47c - GICD interrupt priority register 31"]
    pub gicd_ipriorityr31: GICD_IPRIORITYR31,
    #[doc = "0x480 - GICD interrupt priority register 32"]
    pub gicd_ipriorityr32: GICD_IPRIORITYR32,
    #[doc = "0x484 - GICD interrupt priority register 33"]
    pub gicd_ipriorityr33: GICD_IPRIORITYR33,
    #[doc = "0x488 - GICD interrupt priority register 34"]
    pub gicd_ipriorityr34: GICD_IPRIORITYR34,
    #[doc = "0x48c - GICD interrupt priority register 35"]
    pub gicd_ipriorityr35: GICD_IPRIORITYR35,
    #[doc = "0x490 - GICD interrupt priority register 36"]
    pub gicd_ipriorityr36: GICD_IPRIORITYR36,
    #[doc = "0x494 - GICD interrupt priority register 37"]
    pub gicd_ipriorityr37: GICD_IPRIORITYR37,
    #[doc = "0x498 - GICD interrupt priority register 38"]
    pub gicd_ipriorityr38: GICD_IPRIORITYR38,
    #[doc = "0x49c - GICD interrupt priority register 39"]
    pub gicd_ipriorityr39: GICD_IPRIORITYR39,
    #[doc = "0x4a0 - GICD interrupt priority register 40"]
    pub gicd_ipriorityr40: GICD_IPRIORITYR40,
    #[doc = "0x4a4 - GICD interrupt priority register 41"]
    pub gicd_ipriorityr41: GICD_IPRIORITYR41,
    #[doc = "0x4a8 - GICD interrupt priority register 42"]
    pub gicd_ipriorityr42: GICD_IPRIORITYR42,
    #[doc = "0x4ac - GICD interrupt priority register 43"]
    pub gicd_ipriorityr43: GICD_IPRIORITYR43,
    #[doc = "0x4b0 - GICD interrupt priority register 44"]
    pub gicd_ipriorityr44: GICD_IPRIORITYR44,
    #[doc = "0x4b4 - GICD interrupt priority register 45"]
    pub gicd_ipriorityr45: GICD_IPRIORITYR45,
    #[doc = "0x4b8 - GICD interrupt priority register 46"]
    pub gicd_ipriorityr46: GICD_IPRIORITYR46,
    #[doc = "0x4bc - GICD interrupt priority register 47"]
    pub gicd_ipriorityr47: GICD_IPRIORITYR47,
    #[doc = "0x4c0 - GICD interrupt priority register 48"]
    pub gicd_ipriorityr48: GICD_IPRIORITYR48,
    #[doc = "0x4c4 - GICD interrupt priority register 49"]
    pub gicd_ipriorityr49: GICD_IPRIORITYR49,
    #[doc = "0x4c8 - GICD interrupt priority register 50"]
    pub gicd_ipriorityr50: GICD_IPRIORITYR50,
    #[doc = "0x4cc - GICD interrupt priority register 51"]
    pub gicd_ipriorityr51: GICD_IPRIORITYR51,
    #[doc = "0x4d0 - GICD interrupt priority register 52"]
    pub gicd_ipriorityr52: GICD_IPRIORITYR52,
    #[doc = "0x4d4 - GICD interrupt priority register 53"]
    pub gicd_ipriorityr53: GICD_IPRIORITYR53,
    #[doc = "0x4d8 - GICD interrupt priority register 54"]
    pub gicd_ipriorityr54: GICD_IPRIORITYR54,
    #[doc = "0x4dc - GICD interrupt priority register 55"]
    pub gicd_ipriorityr55: GICD_IPRIORITYR55,
    #[doc = "0x4e0 - GICD interrupt priority register 56"]
    pub gicd_ipriorityr56: GICD_IPRIORITYR56,
    #[doc = "0x4e4 - GICD interrupt priority register 57"]
    pub gicd_ipriorityr57: GICD_IPRIORITYR57,
    #[doc = "0x4e8 - GICD interrupt priority register 58"]
    pub gicd_ipriorityr58: GICD_IPRIORITYR58,
    #[doc = "0x4ec - GICD interrupt priority register 59"]
    pub gicd_ipriorityr59: GICD_IPRIORITYR59,
    #[doc = "0x4f0 - GICD interrupt priority register 60"]
    pub gicd_ipriorityr60: GICD_IPRIORITYR60,
    #[doc = "0x4f4 - GICD interrupt priority register 61"]
    pub gicd_ipriorityr61: GICD_IPRIORITYR61,
    #[doc = "0x4f8 - GICD interrupt priority register 62"]
    pub gicd_ipriorityr62: GICD_IPRIORITYR62,
    #[doc = "0x4fc - GICD interrupt priority register 63"]
    pub gicd_ipriorityr63: GICD_IPRIORITYR63,
    #[doc = "0x500 - GICD interrupt priority register 64"]
    pub gicd_ipriorityr64: GICD_IPRIORITYR64,
    #[doc = "0x504 - GICD interrupt priority register 65"]
    pub gicd_ipriorityr65: GICD_IPRIORITYR65,
    #[doc = "0x508 - GICD interrupt priority register 66"]
    pub gicd_ipriorityr66: GICD_IPRIORITYR66,
    #[doc = "0x50c - GICD interrupt priority register 67"]
    pub gicd_ipriorityr67: GICD_IPRIORITYR67,
    #[doc = "0x510 - GICD interrupt priority register 68"]
    pub gicd_ipriorityr68: GICD_IPRIORITYR68,
    #[doc = "0x514 - GICD interrupt priority register 69"]
    pub gicd_ipriorityr69: GICD_IPRIORITYR69,
    #[doc = "0x518 - GICD interrupt priority register 70"]
    pub gicd_ipriorityr70: GICD_IPRIORITYR70,
    #[doc = "0x51c - GICD interrupt priority register 71"]
    pub gicd_ipriorityr71: GICD_IPRIORITYR71,
    _reserved138: [u8; 0x02e0],
    #[doc = "0x800 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr0: GICD_ITARGETSR0,
    #[doc = "0x804 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr1: GICD_ITARGETSR1,
    #[doc = "0x808 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr2: GICD_ITARGETSR2,
    #[doc = "0x80c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr3: GICD_ITARGETSR3,
    #[doc = "0x810 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr4: GICD_ITARGETSR4,
    #[doc = "0x814 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr5: GICD_ITARGETSR5,
    #[doc = "0x818 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr6: GICD_ITARGETSR6,
    #[doc = "0x81c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr7: GICD_ITARGETSR7,
    #[doc = "0x820 - GICD interrupt processor target register 8"]
    pub gicd_itargetsr8: GICD_ITARGETSR8,
    #[doc = "0x824 - GICD interrupt processor target register 9"]
    pub gicd_itargetsr9: GICD_ITARGETSR9,
    #[doc = "0x828 - GICD interrupt processor target register 10"]
    pub gicd_itargetsr10: GICD_ITARGETSR10,
    #[doc = "0x82c - GICD interrupt processor target register 11"]
    pub gicd_itargetsr11: GICD_ITARGETSR11,
    #[doc = "0x830 - GICD interrupt processor target register 12"]
    pub gicd_itargetsr12: GICD_ITARGETSR12,
    #[doc = "0x834 - GICD interrupt processor target register 13"]
    pub gicd_itargetsr13: GICD_ITARGETSR13,
    #[doc = "0x838 - GICD interrupt processor target register 14"]
    pub gicd_itargetsr14: GICD_ITARGETSR14,
    #[doc = "0x83c - GICD interrupt processor target register 15"]
    pub gicd_itargetsr15: GICD_ITARGETSR15,
    #[doc = "0x840 - GICD interrupt processor target register 16"]
    pub gicd_itargetsr16: GICD_ITARGETSR16,
    #[doc = "0x844 - GICD interrupt processor target register 17"]
    pub gicd_itargetsr17: GICD_ITARGETSR17,
    #[doc = "0x848 - GICD interrupt processor target register 18"]
    pub gicd_itargetsr18: GICD_ITARGETSR18,
    #[doc = "0x84c - GICD interrupt processor target register 19"]
    pub gicd_itargetsr19: GICD_ITARGETSR19,
    #[doc = "0x850 - GICD interrupt processor target register 20"]
    pub gicd_itargetsr20: GICD_ITARGETSR20,
    #[doc = "0x854 - GICD interrupt processor target register 21"]
    pub gicd_itargetsr21: GICD_ITARGETSR21,
    #[doc = "0x858 - GICD interrupt processor target register 22"]
    pub gicd_itargetsr22: GICD_ITARGETSR22,
    #[doc = "0x85c - GICD interrupt processor target register 23"]
    pub gicd_itargetsr23: GICD_ITARGETSR23,
    #[doc = "0x860 - GICD interrupt processor target register 24"]
    pub gicd_itargetsr24: GICD_ITARGETSR24,
    #[doc = "0x864 - GICD interrupt processor target register 25"]
    pub gicd_itargetsr25: GICD_ITARGETSR25,
    #[doc = "0x868 - GICD interrupt processor target register 26"]
    pub gicd_itargetsr26: GICD_ITARGETSR26,
    #[doc = "0x86c - GICD interrupt processor target register 27"]
    pub gicd_itargetsr27: GICD_ITARGETSR27,
    #[doc = "0x870 - GICD interrupt processor target register 28"]
    pub gicd_itargetsr28: GICD_ITARGETSR28,
    #[doc = "0x874 - GICD interrupt processor target register 29"]
    pub gicd_itargetsr29: GICD_ITARGETSR29,
    #[doc = "0x878 - GICD interrupt processor target register 30"]
    pub gicd_itargetsr30: GICD_ITARGETSR30,
    #[doc = "0x87c - GICD interrupt processor target register 31"]
    pub gicd_itargetsr31: GICD_ITARGETSR31,
    #[doc = "0x880 - GICD interrupt processor target register 32"]
    pub gicd_itargetsr32: GICD_ITARGETSR32,
    #[doc = "0x884 - GICD interrupt processor target register 33"]
    pub gicd_itargetsr33: GICD_ITARGETSR33,
    #[doc = "0x888 - GICD interrupt processor target register 34"]
    pub gicd_itargetsr34: GICD_ITARGETSR34,
    #[doc = "0x88c - GICD interrupt processor target register 35"]
    pub gicd_itargetsr35: GICD_ITARGETSR35,
    #[doc = "0x890 - GICD interrupt processor target register 36"]
    pub gicd_itargetsr36: GICD_ITARGETSR36,
    #[doc = "0x894 - GICD interrupt processor target register 37"]
    pub gicd_itargetsr37: GICD_ITARGETSR37,
    #[doc = "0x898 - GICD interrupt processor target register 38"]
    pub gicd_itargetsr38: GICD_ITARGETSR38,
    #[doc = "0x89c - GICD interrupt processor target register 39"]
    pub gicd_itargetsr39: GICD_ITARGETSR39,
    #[doc = "0x8a0 - GICD interrupt processor target register 40"]
    pub gicd_itargetsr40: GICD_ITARGETSR40,
    #[doc = "0x8a4 - GICD interrupt processor target register 41"]
    pub gicd_itargetsr41: GICD_ITARGETSR41,
    #[doc = "0x8a8 - GICD interrupt processor target register 42"]
    pub gicd_itargetsr42: GICD_ITARGETSR42,
    #[doc = "0x8ac - GICD interrupt processor target register 43"]
    pub gicd_itargetsr43: GICD_ITARGETSR43,
    #[doc = "0x8b0 - GICD interrupt processor target register 44"]
    pub gicd_itargetsr44: GICD_ITARGETSR44,
    #[doc = "0x8b4 - GICD interrupt processor target register 45"]
    pub gicd_itargetsr45: GICD_ITARGETSR45,
    #[doc = "0x8b8 - GICD interrupt processor target register 46"]
    pub gicd_itargetsr46: GICD_ITARGETSR46,
    #[doc = "0x8bc - GICD interrupt processor target register 47"]
    pub gicd_itargetsr47: GICD_ITARGETSR47,
    #[doc = "0x8c0 - GICD interrupt processor target register 48"]
    pub gicd_itargetsr48: GICD_ITARGETSR48,
    #[doc = "0x8c4 - GICD interrupt processor target register 49"]
    pub gicd_itargetsr49: GICD_ITARGETSR49,
    #[doc = "0x8c8 - GICD interrupt processor target register 50"]
    pub gicd_itargetsr50: GICD_ITARGETSR50,
    #[doc = "0x8cc - GICD interrupt processor target register 51"]
    pub gicd_itargetsr51: GICD_ITARGETSR51,
    #[doc = "0x8d0 - GICD interrupt processor target register 52"]
    pub gicd_itargetsr52: GICD_ITARGETSR52,
    #[doc = "0x8d4 - GICD interrupt processor target register 53"]
    pub gicd_itargetsr53: GICD_ITARGETSR53,
    #[doc = "0x8d8 - GICD interrupt processor target register 54"]
    pub gicd_itargetsr54: GICD_ITARGETSR54,
    #[doc = "0x8dc - GICD interrupt processor target register 55"]
    pub gicd_itargetsr55: GICD_ITARGETSR55,
    #[doc = "0x8e0 - GICD interrupt processor target register 56"]
    pub gicd_itargetsr56: GICD_ITARGETSR56,
    #[doc = "0x8e4 - GICD interrupt processor target register 57"]
    pub gicd_itargetsr57: GICD_ITARGETSR57,
    #[doc = "0x8e8 - GICD interrupt processor target register 58"]
    pub gicd_itargetsr58: GICD_ITARGETSR58,
    #[doc = "0x8ec - GICD interrupt processor target register 59"]
    pub gicd_itargetsr59: GICD_ITARGETSR59,
    #[doc = "0x8f0 - GICD interrupt processor target register 60"]
    pub gicd_itargetsr60: GICD_ITARGETSR60,
    #[doc = "0x8f4 - GICD interrupt processor target register 61"]
    pub gicd_itargetsr61: GICD_ITARGETSR61,
    #[doc = "0x8f8 - GICD interrupt processor target register 62"]
    pub gicd_itargetsr62: GICD_ITARGETSR62,
    #[doc = "0x8fc - GICD interrupt processor target register 63"]
    pub gicd_itargetsr63: GICD_ITARGETSR63,
    #[doc = "0x900 - GICD interrupt processor target register 64"]
    pub gicd_itargetsr64: GICD_ITARGETSR64,
    #[doc = "0x904 - GICD interrupt processor target register 65"]
    pub gicd_itargetsr65: GICD_ITARGETSR65,
    #[doc = "0x908 - GICD interrupt processor target register 66"]
    pub gicd_itargetsr66: GICD_ITARGETSR66,
    #[doc = "0x90c - GICD interrupt processor target register 67"]
    pub gicd_itargetsr67: GICD_ITARGETSR67,
    #[doc = "0x910 - GICD interrupt processor target register 68"]
    pub gicd_itargetsr68: GICD_ITARGETSR68,
    #[doc = "0x914 - GICD interrupt processor target register 69"]
    pub gicd_itargetsr69: GICD_ITARGETSR69,
    #[doc = "0x918 - GICD interrupt processor target register 70"]
    pub gicd_itargetsr70: GICD_ITARGETSR70,
    #[doc = "0x91c - GICD interrupt processor target register 71"]
    pub gicd_itargetsr71: GICD_ITARGETSR71,
    _reserved210: [u8; 0x02e0],
    #[doc = "0xc00 - GICD interrupt configuration register"]
    pub gicd_icfgr0: GICD_ICFGR0,
    #[doc = "0xc04 - GICD interrupt configuration register"]
    pub gicd_icfgr1: GICD_ICFGR1,
    #[doc = "0xc08 - GICD interrupt configuration register 2"]
    pub gicd_icfgr2: GICD_ICFGR2,
    #[doc = "0xc0c - GICD interrupt configuration register 3"]
    pub gicd_icfgr3: GICD_ICFGR3,
    #[doc = "0xc10 - GICD interrupt configuration register 4"]
    pub gicd_icfgr4: GICD_ICFGR4,
    #[doc = "0xc14 - GICD interrupt configuration register 5"]
    pub gicd_icfgr5: GICD_ICFGR5,
    #[doc = "0xc18 - GICD interrupt configuration register 6"]
    pub gicd_icfgr6: GICD_ICFGR6,
    #[doc = "0xc1c - GICD interrupt configuration register 7"]
    pub gicd_icfgr7: GICD_ICFGR7,
    #[doc = "0xc20 - GICD interrupt configuration register 8"]
    pub gicd_icfgr8: GICD_ICFGR8,
    #[doc = "0xc24 - GICD interrupt configuration register 9"]
    pub gicd_icfgr9: GICD_ICFGR9,
    #[doc = "0xc28 - GICD interrupt configuration register 10"]
    pub gicd_icfgr10: GICD_ICFGR10,
    #[doc = "0xc2c - GICD interrupt configuration register 11"]
    pub gicd_icfgr11: GICD_ICFGR11,
    #[doc = "0xc30 - GICD interrupt configuration register 12"]
    pub gicd_icfgr12: GICD_ICFGR12,
    #[doc = "0xc34 - GICD interrupt configuration register 13"]
    pub gicd_icfgr13: GICD_ICFGR13,
    #[doc = "0xc38 - GICD interrupt configuration register 14"]
    pub gicd_icfgr14: GICD_ICFGR14,
    #[doc = "0xc3c - GICD interrupt configuration register 15"]
    pub gicd_icfgr15: GICD_ICFGR15,
    #[doc = "0xc40 - GICD interrupt configuration register 16"]
    pub gicd_icfgr16: GICD_ICFGR16,
    #[doc = "0xc44 - GICD interrupt configuration register 17"]
    pub gicd_icfgr17: GICD_ICFGR17,
    _reserved228: [u8; 0xb8],
    #[doc = "0xd00 - GICD private peripheral interrupt status register"]
    pub gicd_ppisr: GICD_PPISR,
    _reserved229: [u8; 0x04],
    #[doc = "0xd08 - For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
    pub gicd_spisr1: GICD_SPISR1,
    #[doc = "0xd0c - For interrupts ID"]
    pub gicd_spisr2: GICD_SPISR2,
    #[doc = "0xd10 - For interrupts ID"]
    pub gicd_spisr3: GICD_SPISR3,
    #[doc = "0xd14 - For interrupts ID"]
    pub gicd_spisr4: GICD_SPISR4,
    #[doc = "0xd18 - For interrupts ID"]
    pub gicd_spisr5: GICD_SPISR5,
    #[doc = "0xd1c - For interrupts ID"]
    pub gicd_spisr6: GICD_SPISR6,
    #[doc = "0xd20 - For interrupts ID"]
    pub gicd_spisr7: GICD_SPISR7,
    _reserved236: [u8; 0x01dc],
    #[doc = "0xf00 - GICD software generated interrupt register"]
    pub gicd_sgir: GICD_SGIR,
    _reserved237: [u8; 0x0c],
    #[doc = "0xf10 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir0: GICD_CPENDSGIR0,
    #[doc = "0xf14 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir1: GICD_CPENDSGIR1,
    #[doc = "0xf18 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir2: GICD_CPENDSGIR2,
    #[doc = "0xf1c - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir3: GICD_CPENDSGIR3,
    #[doc = "0xf20 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir0: GICD_SPENDSGIR0,
    #[doc = "0xf24 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir1: GICD_SPENDSGIR1,
    #[doc = "0xf28 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir2: GICD_SPENDSGIR2,
    #[doc = "0xf2c - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir3: GICD_SPENDSGIR3,
    _reserved245: [u8; 0xa0],
    #[doc = "0xfd0 - GICD peripheral ID4 register"]
    pub gicd_pidr4: GICD_PIDR4,
    #[doc = "0xfd4 - GICD peripheral ID5 to ID7 register 5"]
    pub gicd_pidr5: GICD_PIDR5,
    #[doc = "0xfd8 - GICD peripheral ID5 to ID7 register 6"]
    pub gicd_pidr6: GICD_PIDR6,
    #[doc = "0xfdc - GICD peripheral ID5 to ID7 register 7"]
    pub gicd_pidr7: GICD_PIDR7,
    #[doc = "0xfe0 - GICD peripheral ID0 register"]
    pub gicd_pidr0: GICD_PIDR0,
    #[doc = "0xfe4 - GICD peripheral ID1 register"]
    pub gicd_pidr1: GICD_PIDR1,
    #[doc = "0xfe8 - GICD peripheral ID2 register"]
    pub gicd_pidr2: GICD_PIDR2,
    #[doc = "0xfec - GICD peripheral ID3 register"]
    pub gicd_pidr3: GICD_PIDR3,
    #[doc = "0xff0 - GICD component ID0 register"]
    pub gicd_cidr0: GICD_CIDR0,
    #[doc = "0xff4 - GICD component ID1 register"]
    pub gicd_cidr1: GICD_CIDR1,
    #[doc = "0xff8 - GICD component ID2 register"]
    pub gicd_cidr2: GICD_CIDR2,
    #[doc = "0xffc - GICD component ID3 register"]
    pub gicd_cidr3: GICD_CIDR3,
}
#[doc = "GICD_CTLR (rw) register accessor: an alias for `Reg<GICD_CTLR_SPEC>`"]
pub type GICD_CTLR = crate::Reg<gicd_ctlr::GICD_CTLR_SPEC>;
#[doc = "GICD control register"]
pub mod gicd_ctlr;
#[doc = "GICD_TYPER (r) register accessor: an alias for `Reg<GICD_TYPER_SPEC>`"]
pub type GICD_TYPER = crate::Reg<gicd_typer::GICD_TYPER_SPEC>;
#[doc = "GICD interrupt controller type register"]
pub mod gicd_typer;
#[doc = "GICD_IIDR (r) register accessor: an alias for `Reg<GICD_IIDR_SPEC>`"]
pub type GICD_IIDR = crate::Reg<gicd_iidr::GICD_IIDR_SPEC>;
#[doc = "GICD implementer identification register"]
pub mod gicd_iidr;
#[doc = "GICD_IGROUPR0 (rw) register accessor: an alias for `Reg<GICD_IGROUPR0_SPEC>`"]
pub type GICD_IGROUPR0 = crate::Reg<gicd_igroupr0::GICD_IGROUPR0_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr0;
#[doc = "GICD_IGROUPR1 (rw) register accessor: an alias for `Reg<GICD_IGROUPR1_SPEC>`"]
pub type GICD_IGROUPR1 = crate::Reg<gicd_igroupr1::GICD_IGROUPR1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr1;
#[doc = "GICD_IGROUPR2 (rw) register accessor: an alias for `Reg<GICD_IGROUPR2_SPEC>`"]
pub type GICD_IGROUPR2 = crate::Reg<gicd_igroupr2::GICD_IGROUPR2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr2;
#[doc = "GICD_IGROUPR3 (rw) register accessor: an alias for `Reg<GICD_IGROUPR3_SPEC>`"]
pub type GICD_IGROUPR3 = crate::Reg<gicd_igroupr3::GICD_IGROUPR3_SPEC>;
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr3;
#[doc = "GICD_IGROUPR4 (rw) register accessor: an alias for `Reg<GICD_IGROUPR4_SPEC>`"]
pub type GICD_IGROUPR4 = crate::Reg<gicd_igroupr4::GICD_IGROUPR4_SPEC>;
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr4;
#[doc = "GICD_IGROUPR5 (rw) register accessor: an alias for `Reg<GICD_IGROUPR5_SPEC>`"]
pub type GICD_IGROUPR5 = crate::Reg<gicd_igroupr5::GICD_IGROUPR5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr5;
#[doc = "GICD_IGROUPR6 (rw) register accessor: an alias for `Reg<GICD_IGROUPR6_SPEC>`"]
pub type GICD_IGROUPR6 = crate::Reg<gicd_igroupr6::GICD_IGROUPR6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr6;
#[doc = "GICD_IGROUPR7 (rw) register accessor: an alias for `Reg<GICD_IGROUPR7_SPEC>`"]
pub type GICD_IGROUPR7 = crate::Reg<gicd_igroupr7::GICD_IGROUPR7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr7;
#[doc = "GICD_IGROUPR8 (rw) register accessor: an alias for `Reg<GICD_IGROUPR8_SPEC>`"]
pub type GICD_IGROUPR8 = crate::Reg<gicd_igroupr8::GICD_IGROUPR8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr8;
#[doc = "GICD_ISENABLER0 (rw) register accessor: an alias for `Reg<GICD_ISENABLER0_SPEC>`"]
pub type GICD_ISENABLER0 = crate::Reg<gicd_isenabler0::GICD_ISENABLER0_SPEC>;
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_isenabler0;
#[doc = "GICD_ISENABLER1 (rw) register accessor: an alias for `Reg<GICD_ISENABLER1_SPEC>`"]
pub type GICD_ISENABLER1 = crate::Reg<gicd_isenabler1::GICD_ISENABLER1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler1;
#[doc = "GICD_ISENABLER2 (rw) register accessor: an alias for `Reg<GICD_ISENABLER2_SPEC>`"]
pub type GICD_ISENABLER2 = crate::Reg<gicd_isenabler2::GICD_ISENABLER2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler2;
#[doc = "GICD_ISENABLER3 (rw) register accessor: an alias for `Reg<GICD_ISENABLER3_SPEC>`"]
pub type GICD_ISENABLER3 = crate::Reg<gicd_isenabler3::GICD_ISENABLER3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler3;
#[doc = "GICD_ISENABLER4 (rw) register accessor: an alias for `Reg<GICD_ISENABLER4_SPEC>`"]
pub type GICD_ISENABLER4 = crate::Reg<gicd_isenabler4::GICD_ISENABLER4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler4;
#[doc = "GICD_ISENABLER5 (rw) register accessor: an alias for `Reg<GICD_ISENABLER5_SPEC>`"]
pub type GICD_ISENABLER5 = crate::Reg<gicd_isenabler5::GICD_ISENABLER5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler5;
#[doc = "GICD_ISENABLER6 (rw) register accessor: an alias for `Reg<GICD_ISENABLER6_SPEC>`"]
pub type GICD_ISENABLER6 = crate::Reg<gicd_isenabler6::GICD_ISENABLER6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler6;
#[doc = "GICD_ISENABLER7 (rw) register accessor: an alias for `Reg<GICD_ISENABLER7_SPEC>`"]
pub type GICD_ISENABLER7 = crate::Reg<gicd_isenabler7::GICD_ISENABLER7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler7;
#[doc = "GICD_ISENABLER8 (rw) register accessor: an alias for `Reg<GICD_ISENABLER8_SPEC>`"]
pub type GICD_ISENABLER8 = crate::Reg<gicd_isenabler8::GICD_ISENABLER8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler8;
#[doc = "GICD_ICENABLER0 (rw) register accessor: an alias for `Reg<GICD_ICENABLER0_SPEC>`"]
pub type GICD_ICENABLER0 = crate::Reg<gicd_icenabler0::GICD_ICENABLER0_SPEC>;
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_icenabler0;
#[doc = "GICD_ICENABLER1 (rw) register accessor: an alias for `Reg<GICD_ICENABLER1_SPEC>`"]
pub type GICD_ICENABLER1 = crate::Reg<gicd_icenabler1::GICD_ICENABLER1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler1;
#[doc = "GICD_ICENABLER2 (rw) register accessor: an alias for `Reg<GICD_ICENABLER2_SPEC>`"]
pub type GICD_ICENABLER2 = crate::Reg<gicd_icenabler2::GICD_ICENABLER2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler2;
#[doc = "GICD_ICENABLER3 (rw) register accessor: an alias for `Reg<GICD_ICENABLER3_SPEC>`"]
pub type GICD_ICENABLER3 = crate::Reg<gicd_icenabler3::GICD_ICENABLER3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler3;
#[doc = "GICD_ICENABLER4 (rw) register accessor: an alias for `Reg<GICD_ICENABLER4_SPEC>`"]
pub type GICD_ICENABLER4 = crate::Reg<gicd_icenabler4::GICD_ICENABLER4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler4;
#[doc = "GICD_ICENABLER5 (rw) register accessor: an alias for `Reg<GICD_ICENABLER5_SPEC>`"]
pub type GICD_ICENABLER5 = crate::Reg<gicd_icenabler5::GICD_ICENABLER5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler5;
#[doc = "GICD_ICENABLER6 (rw) register accessor: an alias for `Reg<GICD_ICENABLER6_SPEC>`"]
pub type GICD_ICENABLER6 = crate::Reg<gicd_icenabler6::GICD_ICENABLER6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler6;
#[doc = "GICD_ICENABLER7 (rw) register accessor: an alias for `Reg<GICD_ICENABLER7_SPEC>`"]
pub type GICD_ICENABLER7 = crate::Reg<gicd_icenabler7::GICD_ICENABLER7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler7;
#[doc = "GICD_ICENABLER8 (rw) register accessor: an alias for `Reg<GICD_ICENABLER8_SPEC>`"]
pub type GICD_ICENABLER8 = crate::Reg<gicd_icenabler8::GICD_ICENABLER8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler8;
#[doc = "GICD_ISPENDR0 (rw) register accessor: an alias for `Reg<GICD_ISPENDR0_SPEC>`"]
pub type GICD_ISPENDR0 = crate::Reg<gicd_ispendr0::GICD_ISPENDR0_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr0;
#[doc = "GICD_ISPENDR1 (rw) register accessor: an alias for `Reg<GICD_ISPENDR1_SPEC>`"]
pub type GICD_ISPENDR1 = crate::Reg<gicd_ispendr1::GICD_ISPENDR1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr1;
#[doc = "GICD_ISPENDR2 (rw) register accessor: an alias for `Reg<GICD_ISPENDR2_SPEC>`"]
pub type GICD_ISPENDR2 = crate::Reg<gicd_ispendr2::GICD_ISPENDR2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr2;
#[doc = "GICD_ISPENDR3 (rw) register accessor: an alias for `Reg<GICD_ISPENDR3_SPEC>`"]
pub type GICD_ISPENDR3 = crate::Reg<gicd_ispendr3::GICD_ISPENDR3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr3;
#[doc = "GICD_ISPENDR4 (rw) register accessor: an alias for `Reg<GICD_ISPENDR4_SPEC>`"]
pub type GICD_ISPENDR4 = crate::Reg<gicd_ispendr4::GICD_ISPENDR4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr4;
#[doc = "GICD_ISPENDR5 (rw) register accessor: an alias for `Reg<GICD_ISPENDR5_SPEC>`"]
pub type GICD_ISPENDR5 = crate::Reg<gicd_ispendr5::GICD_ISPENDR5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr5;
#[doc = "GICD_ISPENDR6 (rw) register accessor: an alias for `Reg<GICD_ISPENDR6_SPEC>`"]
pub type GICD_ISPENDR6 = crate::Reg<gicd_ispendr6::GICD_ISPENDR6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr6;
#[doc = "GICD_ISPENDR7 (rw) register accessor: an alias for `Reg<GICD_ISPENDR7_SPEC>`"]
pub type GICD_ISPENDR7 = crate::Reg<gicd_ispendr7::GICD_ISPENDR7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr7;
#[doc = "GICD_ISPENDR8 (rw) register accessor: an alias for `Reg<GICD_ISPENDR8_SPEC>`"]
pub type GICD_ISPENDR8 = crate::Reg<gicd_ispendr8::GICD_ISPENDR8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr8;
#[doc = "GICD_ICPENDR0 (rw) register accessor: an alias for `Reg<GICD_ICPENDR0_SPEC>`"]
pub type GICD_ICPENDR0 = crate::Reg<gicd_icpendr0::GICD_ICPENDR0_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr0;
#[doc = "GICD_ICPENDR1 (rw) register accessor: an alias for `Reg<GICD_ICPENDR1_SPEC>`"]
pub type GICD_ICPENDR1 = crate::Reg<gicd_icpendr1::GICD_ICPENDR1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr1;
#[doc = "GICD_ICPENDR2 (rw) register accessor: an alias for `Reg<GICD_ICPENDR2_SPEC>`"]
pub type GICD_ICPENDR2 = crate::Reg<gicd_icpendr2::GICD_ICPENDR2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr2;
#[doc = "GICD_ICPENDR3 (rw) register accessor: an alias for `Reg<GICD_ICPENDR3_SPEC>`"]
pub type GICD_ICPENDR3 = crate::Reg<gicd_icpendr3::GICD_ICPENDR3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr3;
#[doc = "GICD_ICPENDR4 (rw) register accessor: an alias for `Reg<GICD_ICPENDR4_SPEC>`"]
pub type GICD_ICPENDR4 = crate::Reg<gicd_icpendr4::GICD_ICPENDR4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr4;
#[doc = "GICD_ICPENDR5 (rw) register accessor: an alias for `Reg<GICD_ICPENDR5_SPEC>`"]
pub type GICD_ICPENDR5 = crate::Reg<gicd_icpendr5::GICD_ICPENDR5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr5;
#[doc = "GICD_ICPENDR6 (rw) register accessor: an alias for `Reg<GICD_ICPENDR6_SPEC>`"]
pub type GICD_ICPENDR6 = crate::Reg<gicd_icpendr6::GICD_ICPENDR6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr6;
#[doc = "GICD_ICPENDR7 (rw) register accessor: an alias for `Reg<GICD_ICPENDR7_SPEC>`"]
pub type GICD_ICPENDR7 = crate::Reg<gicd_icpendr7::GICD_ICPENDR7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr7;
#[doc = "GICD_ICPENDR8 (rw) register accessor: an alias for `Reg<GICD_ICPENDR8_SPEC>`"]
pub type GICD_ICPENDR8 = crate::Reg<gicd_icpendr8::GICD_ICPENDR8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr8;
#[doc = "GICD_ISACTIVER0 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER0_SPEC>`"]
pub type GICD_ISACTIVER0 = crate::Reg<gicd_isactiver0::GICD_ISACTIVER0_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver0;
#[doc = "GICD_ISACTIVER1 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER1_SPEC>`"]
pub type GICD_ISACTIVER1 = crate::Reg<gicd_isactiver1::GICD_ISACTIVER1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver1;
#[doc = "GICD_ISACTIVER2 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER2_SPEC>`"]
pub type GICD_ISACTIVER2 = crate::Reg<gicd_isactiver2::GICD_ISACTIVER2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver2;
#[doc = "GICD_ISACTIVER3 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER3_SPEC>`"]
pub type GICD_ISACTIVER3 = crate::Reg<gicd_isactiver3::GICD_ISACTIVER3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver3;
#[doc = "GICD_ISACTIVER4 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER4_SPEC>`"]
pub type GICD_ISACTIVER4 = crate::Reg<gicd_isactiver4::GICD_ISACTIVER4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver4;
#[doc = "GICD_ISACTIVER5 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER5_SPEC>`"]
pub type GICD_ISACTIVER5 = crate::Reg<gicd_isactiver5::GICD_ISACTIVER5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver5;
#[doc = "GICD_ISACTIVER6 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER6_SPEC>`"]
pub type GICD_ISACTIVER6 = crate::Reg<gicd_isactiver6::GICD_ISACTIVER6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver6;
#[doc = "GICD_ISACTIVER7 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER7_SPEC>`"]
pub type GICD_ISACTIVER7 = crate::Reg<gicd_isactiver7::GICD_ISACTIVER7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver7;
#[doc = "GICD_ISACTIVER8 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER8_SPEC>`"]
pub type GICD_ISACTIVER8 = crate::Reg<gicd_isactiver8::GICD_ISACTIVER8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver8;
#[doc = "GICD_ICACTIVER0 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER0_SPEC>`"]
pub type GICD_ICACTIVER0 = crate::Reg<gicd_icactiver0::GICD_ICACTIVER0_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver0;
#[doc = "GICD_ICACTIVER1 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER1_SPEC>`"]
pub type GICD_ICACTIVER1 = crate::Reg<gicd_icactiver1::GICD_ICACTIVER1_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver1;
#[doc = "GICD_ICACTIVER2 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER2_SPEC>`"]
pub type GICD_ICACTIVER2 = crate::Reg<gicd_icactiver2::GICD_ICACTIVER2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver2;
#[doc = "GICD_ICACTIVER3 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER3_SPEC>`"]
pub type GICD_ICACTIVER3 = crate::Reg<gicd_icactiver3::GICD_ICACTIVER3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver3;
#[doc = "GICD_ICACTIVER4 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER4_SPEC>`"]
pub type GICD_ICACTIVER4 = crate::Reg<gicd_icactiver4::GICD_ICACTIVER4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver4;
#[doc = "GICD_ICACTIVER5 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER5_SPEC>`"]
pub type GICD_ICACTIVER5 = crate::Reg<gicd_icactiver5::GICD_ICACTIVER5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver5;
#[doc = "GICD_ICACTIVER6 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER6_SPEC>`"]
pub type GICD_ICACTIVER6 = crate::Reg<gicd_icactiver6::GICD_ICACTIVER6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver6;
#[doc = "GICD_ICACTIVER7 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER7_SPEC>`"]
pub type GICD_ICACTIVER7 = crate::Reg<gicd_icactiver7::GICD_ICACTIVER7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver7;
#[doc = "GICD_ICACTIVER8 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER8_SPEC>`"]
pub type GICD_ICACTIVER8 = crate::Reg<gicd_icactiver8::GICD_ICACTIVER8_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver8;
#[doc = "GICD_IPRIORITYR0 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR0_SPEC>`"]
pub type GICD_IPRIORITYR0 = crate::Reg<gicd_ipriorityr0::GICD_IPRIORITYR0_SPEC>;
#[doc = "GICD interrupt priority register 0"]
pub mod gicd_ipriorityr0;
#[doc = "GICD_IPRIORITYR1 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR1_SPEC>`"]
pub type GICD_IPRIORITYR1 = crate::Reg<gicd_ipriorityr1::GICD_IPRIORITYR1_SPEC>;
#[doc = "GICD interrupt priority register 1"]
pub mod gicd_ipriorityr1;
#[doc = "GICD_IPRIORITYR2 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR2_SPEC>`"]
pub type GICD_IPRIORITYR2 = crate::Reg<gicd_ipriorityr2::GICD_IPRIORITYR2_SPEC>;
#[doc = "GICD interrupt priority register 2"]
pub mod gicd_ipriorityr2;
#[doc = "GICD_IPRIORITYR3 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR3_SPEC>`"]
pub type GICD_IPRIORITYR3 = crate::Reg<gicd_ipriorityr3::GICD_IPRIORITYR3_SPEC>;
#[doc = "GICD interrupt priority register 3"]
pub mod gicd_ipriorityr3;
#[doc = "GICD_IPRIORITYR4 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR4_SPEC>`"]
pub type GICD_IPRIORITYR4 = crate::Reg<gicd_ipriorityr4::GICD_IPRIORITYR4_SPEC>;
#[doc = "GICD interrupt priority register 4"]
pub mod gicd_ipriorityr4;
#[doc = "GICD_IPRIORITYR5 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR5_SPEC>`"]
pub type GICD_IPRIORITYR5 = crate::Reg<gicd_ipriorityr5::GICD_IPRIORITYR5_SPEC>;
#[doc = "GICD interrupt priority register 5"]
pub mod gicd_ipriorityr5;
#[doc = "GICD_IPRIORITYR6 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR6_SPEC>`"]
pub type GICD_IPRIORITYR6 = crate::Reg<gicd_ipriorityr6::GICD_IPRIORITYR6_SPEC>;
#[doc = "GICD interrupt priority register 6"]
pub mod gicd_ipriorityr6;
#[doc = "GICD_IPRIORITYR7 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR7_SPEC>`"]
pub type GICD_IPRIORITYR7 = crate::Reg<gicd_ipriorityr7::GICD_IPRIORITYR7_SPEC>;
#[doc = "GICD interrupt priority register 7"]
pub mod gicd_ipriorityr7;
#[doc = "GICD_IPRIORITYR8 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR8_SPEC>`"]
pub type GICD_IPRIORITYR8 = crate::Reg<gicd_ipriorityr8::GICD_IPRIORITYR8_SPEC>;
#[doc = "GICD interrupt priority register 8"]
pub mod gicd_ipriorityr8;
#[doc = "GICD_IPRIORITYR9 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR9_SPEC>`"]
pub type GICD_IPRIORITYR9 = crate::Reg<gicd_ipriorityr9::GICD_IPRIORITYR9_SPEC>;
#[doc = "GICD interrupt priority register 9"]
pub mod gicd_ipriorityr9;
#[doc = "GICD_IPRIORITYR10 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR10_SPEC>`"]
pub type GICD_IPRIORITYR10 = crate::Reg<gicd_ipriorityr10::GICD_IPRIORITYR10_SPEC>;
#[doc = "GICD interrupt priority register 10"]
pub mod gicd_ipriorityr10;
#[doc = "GICD_IPRIORITYR11 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR11_SPEC>`"]
pub type GICD_IPRIORITYR11 = crate::Reg<gicd_ipriorityr11::GICD_IPRIORITYR11_SPEC>;
#[doc = "GICD interrupt priority register 11"]
pub mod gicd_ipriorityr11;
#[doc = "GICD_IPRIORITYR12 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR12_SPEC>`"]
pub type GICD_IPRIORITYR12 = crate::Reg<gicd_ipriorityr12::GICD_IPRIORITYR12_SPEC>;
#[doc = "GICD interrupt priority register 12"]
pub mod gicd_ipriorityr12;
#[doc = "GICD_IPRIORITYR13 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR13_SPEC>`"]
pub type GICD_IPRIORITYR13 = crate::Reg<gicd_ipriorityr13::GICD_IPRIORITYR13_SPEC>;
#[doc = "GICD interrupt priority register 13"]
pub mod gicd_ipriorityr13;
#[doc = "GICD_IPRIORITYR14 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR14_SPEC>`"]
pub type GICD_IPRIORITYR14 = crate::Reg<gicd_ipriorityr14::GICD_IPRIORITYR14_SPEC>;
#[doc = "GICD interrupt priority register 14"]
pub mod gicd_ipriorityr14;
#[doc = "GICD_IPRIORITYR15 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR15_SPEC>`"]
pub type GICD_IPRIORITYR15 = crate::Reg<gicd_ipriorityr15::GICD_IPRIORITYR15_SPEC>;
#[doc = "GICD interrupt priority register 15"]
pub mod gicd_ipriorityr15;
#[doc = "GICD_IPRIORITYR16 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR16_SPEC>`"]
pub type GICD_IPRIORITYR16 = crate::Reg<gicd_ipriorityr16::GICD_IPRIORITYR16_SPEC>;
#[doc = "GICD interrupt priority register 16"]
pub mod gicd_ipriorityr16;
#[doc = "GICD_IPRIORITYR17 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR17_SPEC>`"]
pub type GICD_IPRIORITYR17 = crate::Reg<gicd_ipriorityr17::GICD_IPRIORITYR17_SPEC>;
#[doc = "GICD interrupt priority register 17"]
pub mod gicd_ipriorityr17;
#[doc = "GICD_IPRIORITYR18 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR18_SPEC>`"]
pub type GICD_IPRIORITYR18 = crate::Reg<gicd_ipriorityr18::GICD_IPRIORITYR18_SPEC>;
#[doc = "GICD interrupt priority register 18"]
pub mod gicd_ipriorityr18;
#[doc = "GICD_IPRIORITYR19 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR19_SPEC>`"]
pub type GICD_IPRIORITYR19 = crate::Reg<gicd_ipriorityr19::GICD_IPRIORITYR19_SPEC>;
#[doc = "GICD interrupt priority register 19"]
pub mod gicd_ipriorityr19;
#[doc = "GICD_IPRIORITYR20 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR20_SPEC>`"]
pub type GICD_IPRIORITYR20 = crate::Reg<gicd_ipriorityr20::GICD_IPRIORITYR20_SPEC>;
#[doc = "GICD interrupt priority register 20"]
pub mod gicd_ipriorityr20;
#[doc = "GICD_IPRIORITYR21 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR21_SPEC>`"]
pub type GICD_IPRIORITYR21 = crate::Reg<gicd_ipriorityr21::GICD_IPRIORITYR21_SPEC>;
#[doc = "GICD interrupt priority register 21"]
pub mod gicd_ipriorityr21;
#[doc = "GICD_IPRIORITYR22 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR22_SPEC>`"]
pub type GICD_IPRIORITYR22 = crate::Reg<gicd_ipriorityr22::GICD_IPRIORITYR22_SPEC>;
#[doc = "GICD interrupt priority register 22"]
pub mod gicd_ipriorityr22;
#[doc = "GICD_IPRIORITYR23 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR23_SPEC>`"]
pub type GICD_IPRIORITYR23 = crate::Reg<gicd_ipriorityr23::GICD_IPRIORITYR23_SPEC>;
#[doc = "GICD interrupt priority register 23"]
pub mod gicd_ipriorityr23;
#[doc = "GICD_IPRIORITYR24 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR24_SPEC>`"]
pub type GICD_IPRIORITYR24 = crate::Reg<gicd_ipriorityr24::GICD_IPRIORITYR24_SPEC>;
#[doc = "GICD interrupt priority register 24"]
pub mod gicd_ipriorityr24;
#[doc = "GICD_IPRIORITYR25 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR25_SPEC>`"]
pub type GICD_IPRIORITYR25 = crate::Reg<gicd_ipriorityr25::GICD_IPRIORITYR25_SPEC>;
#[doc = "GICD interrupt priority register 25"]
pub mod gicd_ipriorityr25;
#[doc = "GICD_IPRIORITYR26 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR26_SPEC>`"]
pub type GICD_IPRIORITYR26 = crate::Reg<gicd_ipriorityr26::GICD_IPRIORITYR26_SPEC>;
#[doc = "GICD interrupt priority register 26"]
pub mod gicd_ipriorityr26;
#[doc = "GICD_IPRIORITYR27 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR27_SPEC>`"]
pub type GICD_IPRIORITYR27 = crate::Reg<gicd_ipriorityr27::GICD_IPRIORITYR27_SPEC>;
#[doc = "GICD interrupt priority register 27"]
pub mod gicd_ipriorityr27;
#[doc = "GICD_IPRIORITYR28 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR28_SPEC>`"]
pub type GICD_IPRIORITYR28 = crate::Reg<gicd_ipriorityr28::GICD_IPRIORITYR28_SPEC>;
#[doc = "GICD interrupt priority register 28"]
pub mod gicd_ipriorityr28;
#[doc = "GICD_IPRIORITYR29 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR29_SPEC>`"]
pub type GICD_IPRIORITYR29 = crate::Reg<gicd_ipriorityr29::GICD_IPRIORITYR29_SPEC>;
#[doc = "GICD interrupt priority register 29"]
pub mod gicd_ipriorityr29;
#[doc = "GICD_IPRIORITYR30 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR30_SPEC>`"]
pub type GICD_IPRIORITYR30 = crate::Reg<gicd_ipriorityr30::GICD_IPRIORITYR30_SPEC>;
#[doc = "GICD interrupt priority register 30"]
pub mod gicd_ipriorityr30;
#[doc = "GICD_IPRIORITYR31 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR31_SPEC>`"]
pub type GICD_IPRIORITYR31 = crate::Reg<gicd_ipriorityr31::GICD_IPRIORITYR31_SPEC>;
#[doc = "GICD interrupt priority register 31"]
pub mod gicd_ipriorityr31;
#[doc = "GICD_IPRIORITYR32 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR32_SPEC>`"]
pub type GICD_IPRIORITYR32 = crate::Reg<gicd_ipriorityr32::GICD_IPRIORITYR32_SPEC>;
#[doc = "GICD interrupt priority register 32"]
pub mod gicd_ipriorityr32;
#[doc = "GICD_IPRIORITYR33 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR33_SPEC>`"]
pub type GICD_IPRIORITYR33 = crate::Reg<gicd_ipriorityr33::GICD_IPRIORITYR33_SPEC>;
#[doc = "GICD interrupt priority register 33"]
pub mod gicd_ipriorityr33;
#[doc = "GICD_IPRIORITYR34 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR34_SPEC>`"]
pub type GICD_IPRIORITYR34 = crate::Reg<gicd_ipriorityr34::GICD_IPRIORITYR34_SPEC>;
#[doc = "GICD interrupt priority register 34"]
pub mod gicd_ipriorityr34;
#[doc = "GICD_IPRIORITYR35 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR35_SPEC>`"]
pub type GICD_IPRIORITYR35 = crate::Reg<gicd_ipriorityr35::GICD_IPRIORITYR35_SPEC>;
#[doc = "GICD interrupt priority register 35"]
pub mod gicd_ipriorityr35;
#[doc = "GICD_IPRIORITYR36 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR36_SPEC>`"]
pub type GICD_IPRIORITYR36 = crate::Reg<gicd_ipriorityr36::GICD_IPRIORITYR36_SPEC>;
#[doc = "GICD interrupt priority register 36"]
pub mod gicd_ipriorityr36;
#[doc = "GICD_IPRIORITYR37 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR37_SPEC>`"]
pub type GICD_IPRIORITYR37 = crate::Reg<gicd_ipriorityr37::GICD_IPRIORITYR37_SPEC>;
#[doc = "GICD interrupt priority register 37"]
pub mod gicd_ipriorityr37;
#[doc = "GICD_IPRIORITYR38 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR38_SPEC>`"]
pub type GICD_IPRIORITYR38 = crate::Reg<gicd_ipriorityr38::GICD_IPRIORITYR38_SPEC>;
#[doc = "GICD interrupt priority register 38"]
pub mod gicd_ipriorityr38;
#[doc = "GICD_IPRIORITYR39 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR39_SPEC>`"]
pub type GICD_IPRIORITYR39 = crate::Reg<gicd_ipriorityr39::GICD_IPRIORITYR39_SPEC>;
#[doc = "GICD interrupt priority register 39"]
pub mod gicd_ipriorityr39;
#[doc = "GICD_IPRIORITYR40 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR40_SPEC>`"]
pub type GICD_IPRIORITYR40 = crate::Reg<gicd_ipriorityr40::GICD_IPRIORITYR40_SPEC>;
#[doc = "GICD interrupt priority register 40"]
pub mod gicd_ipriorityr40;
#[doc = "GICD_IPRIORITYR41 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR41_SPEC>`"]
pub type GICD_IPRIORITYR41 = crate::Reg<gicd_ipriorityr41::GICD_IPRIORITYR41_SPEC>;
#[doc = "GICD interrupt priority register 41"]
pub mod gicd_ipriorityr41;
#[doc = "GICD_IPRIORITYR42 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR42_SPEC>`"]
pub type GICD_IPRIORITYR42 = crate::Reg<gicd_ipriorityr42::GICD_IPRIORITYR42_SPEC>;
#[doc = "GICD interrupt priority register 42"]
pub mod gicd_ipriorityr42;
#[doc = "GICD_IPRIORITYR43 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR43_SPEC>`"]
pub type GICD_IPRIORITYR43 = crate::Reg<gicd_ipriorityr43::GICD_IPRIORITYR43_SPEC>;
#[doc = "GICD interrupt priority register 43"]
pub mod gicd_ipriorityr43;
#[doc = "GICD_IPRIORITYR44 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR44_SPEC>`"]
pub type GICD_IPRIORITYR44 = crate::Reg<gicd_ipriorityr44::GICD_IPRIORITYR44_SPEC>;
#[doc = "GICD interrupt priority register 44"]
pub mod gicd_ipriorityr44;
#[doc = "GICD_IPRIORITYR45 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR45_SPEC>`"]
pub type GICD_IPRIORITYR45 = crate::Reg<gicd_ipriorityr45::GICD_IPRIORITYR45_SPEC>;
#[doc = "GICD interrupt priority register 45"]
pub mod gicd_ipriorityr45;
#[doc = "GICD_IPRIORITYR46 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR46_SPEC>`"]
pub type GICD_IPRIORITYR46 = crate::Reg<gicd_ipriorityr46::GICD_IPRIORITYR46_SPEC>;
#[doc = "GICD interrupt priority register 46"]
pub mod gicd_ipriorityr46;
#[doc = "GICD_IPRIORITYR47 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR47_SPEC>`"]
pub type GICD_IPRIORITYR47 = crate::Reg<gicd_ipriorityr47::GICD_IPRIORITYR47_SPEC>;
#[doc = "GICD interrupt priority register 47"]
pub mod gicd_ipriorityr47;
#[doc = "GICD_IPRIORITYR48 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR48_SPEC>`"]
pub type GICD_IPRIORITYR48 = crate::Reg<gicd_ipriorityr48::GICD_IPRIORITYR48_SPEC>;
#[doc = "GICD interrupt priority register 48"]
pub mod gicd_ipriorityr48;
#[doc = "GICD_IPRIORITYR49 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR49_SPEC>`"]
pub type GICD_IPRIORITYR49 = crate::Reg<gicd_ipriorityr49::GICD_IPRIORITYR49_SPEC>;
#[doc = "GICD interrupt priority register 49"]
pub mod gicd_ipriorityr49;
#[doc = "GICD_IPRIORITYR50 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR50_SPEC>`"]
pub type GICD_IPRIORITYR50 = crate::Reg<gicd_ipriorityr50::GICD_IPRIORITYR50_SPEC>;
#[doc = "GICD interrupt priority register 50"]
pub mod gicd_ipriorityr50;
#[doc = "GICD_IPRIORITYR51 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR51_SPEC>`"]
pub type GICD_IPRIORITYR51 = crate::Reg<gicd_ipriorityr51::GICD_IPRIORITYR51_SPEC>;
#[doc = "GICD interrupt priority register 51"]
pub mod gicd_ipriorityr51;
#[doc = "GICD_IPRIORITYR52 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR52_SPEC>`"]
pub type GICD_IPRIORITYR52 = crate::Reg<gicd_ipriorityr52::GICD_IPRIORITYR52_SPEC>;
#[doc = "GICD interrupt priority register 52"]
pub mod gicd_ipriorityr52;
#[doc = "GICD_IPRIORITYR53 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR53_SPEC>`"]
pub type GICD_IPRIORITYR53 = crate::Reg<gicd_ipriorityr53::GICD_IPRIORITYR53_SPEC>;
#[doc = "GICD interrupt priority register 53"]
pub mod gicd_ipriorityr53;
#[doc = "GICD_IPRIORITYR54 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR54_SPEC>`"]
pub type GICD_IPRIORITYR54 = crate::Reg<gicd_ipriorityr54::GICD_IPRIORITYR54_SPEC>;
#[doc = "GICD interrupt priority register 54"]
pub mod gicd_ipriorityr54;
#[doc = "GICD_IPRIORITYR55 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR55_SPEC>`"]
pub type GICD_IPRIORITYR55 = crate::Reg<gicd_ipriorityr55::GICD_IPRIORITYR55_SPEC>;
#[doc = "GICD interrupt priority register 55"]
pub mod gicd_ipriorityr55;
#[doc = "GICD_IPRIORITYR56 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR56_SPEC>`"]
pub type GICD_IPRIORITYR56 = crate::Reg<gicd_ipriorityr56::GICD_IPRIORITYR56_SPEC>;
#[doc = "GICD interrupt priority register 56"]
pub mod gicd_ipriorityr56;
#[doc = "GICD_IPRIORITYR57 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR57_SPEC>`"]
pub type GICD_IPRIORITYR57 = crate::Reg<gicd_ipriorityr57::GICD_IPRIORITYR57_SPEC>;
#[doc = "GICD interrupt priority register 57"]
pub mod gicd_ipriorityr57;
#[doc = "GICD_IPRIORITYR58 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR58_SPEC>`"]
pub type GICD_IPRIORITYR58 = crate::Reg<gicd_ipriorityr58::GICD_IPRIORITYR58_SPEC>;
#[doc = "GICD interrupt priority register 58"]
pub mod gicd_ipriorityr58;
#[doc = "GICD_IPRIORITYR59 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR59_SPEC>`"]
pub type GICD_IPRIORITYR59 = crate::Reg<gicd_ipriorityr59::GICD_IPRIORITYR59_SPEC>;
#[doc = "GICD interrupt priority register 59"]
pub mod gicd_ipriorityr59;
#[doc = "GICD_IPRIORITYR60 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR60_SPEC>`"]
pub type GICD_IPRIORITYR60 = crate::Reg<gicd_ipriorityr60::GICD_IPRIORITYR60_SPEC>;
#[doc = "GICD interrupt priority register 60"]
pub mod gicd_ipriorityr60;
#[doc = "GICD_IPRIORITYR61 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR61_SPEC>`"]
pub type GICD_IPRIORITYR61 = crate::Reg<gicd_ipriorityr61::GICD_IPRIORITYR61_SPEC>;
#[doc = "GICD interrupt priority register 61"]
pub mod gicd_ipriorityr61;
#[doc = "GICD_IPRIORITYR62 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR62_SPEC>`"]
pub type GICD_IPRIORITYR62 = crate::Reg<gicd_ipriorityr62::GICD_IPRIORITYR62_SPEC>;
#[doc = "GICD interrupt priority register 62"]
pub mod gicd_ipriorityr62;
#[doc = "GICD_IPRIORITYR63 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR63_SPEC>`"]
pub type GICD_IPRIORITYR63 = crate::Reg<gicd_ipriorityr63::GICD_IPRIORITYR63_SPEC>;
#[doc = "GICD interrupt priority register 63"]
pub mod gicd_ipriorityr63;
#[doc = "GICD_IPRIORITYR64 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR64_SPEC>`"]
pub type GICD_IPRIORITYR64 = crate::Reg<gicd_ipriorityr64::GICD_IPRIORITYR64_SPEC>;
#[doc = "GICD interrupt priority register 64"]
pub mod gicd_ipriorityr64;
#[doc = "GICD_IPRIORITYR65 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR65_SPEC>`"]
pub type GICD_IPRIORITYR65 = crate::Reg<gicd_ipriorityr65::GICD_IPRIORITYR65_SPEC>;
#[doc = "GICD interrupt priority register 65"]
pub mod gicd_ipriorityr65;
#[doc = "GICD_IPRIORITYR66 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR66_SPEC>`"]
pub type GICD_IPRIORITYR66 = crate::Reg<gicd_ipriorityr66::GICD_IPRIORITYR66_SPEC>;
#[doc = "GICD interrupt priority register 66"]
pub mod gicd_ipriorityr66;
#[doc = "GICD_IPRIORITYR67 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR67_SPEC>`"]
pub type GICD_IPRIORITYR67 = crate::Reg<gicd_ipriorityr67::GICD_IPRIORITYR67_SPEC>;
#[doc = "GICD interrupt priority register 67"]
pub mod gicd_ipriorityr67;
#[doc = "GICD_IPRIORITYR68 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR68_SPEC>`"]
pub type GICD_IPRIORITYR68 = crate::Reg<gicd_ipriorityr68::GICD_IPRIORITYR68_SPEC>;
#[doc = "GICD interrupt priority register 68"]
pub mod gicd_ipriorityr68;
#[doc = "GICD_IPRIORITYR69 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR69_SPEC>`"]
pub type GICD_IPRIORITYR69 = crate::Reg<gicd_ipriorityr69::GICD_IPRIORITYR69_SPEC>;
#[doc = "GICD interrupt priority register 69"]
pub mod gicd_ipriorityr69;
#[doc = "GICD_IPRIORITYR70 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR70_SPEC>`"]
pub type GICD_IPRIORITYR70 = crate::Reg<gicd_ipriorityr70::GICD_IPRIORITYR70_SPEC>;
#[doc = "GICD interrupt priority register 70"]
pub mod gicd_ipriorityr70;
#[doc = "GICD_IPRIORITYR71 (rw) register accessor: an alias for `Reg<GICD_IPRIORITYR71_SPEC>`"]
pub type GICD_IPRIORITYR71 = crate::Reg<gicd_ipriorityr71::GICD_IPRIORITYR71_SPEC>;
#[doc = "GICD interrupt priority register 71"]
pub mod gicd_ipriorityr71;
#[doc = "GICD_ITARGETSR0 (r) register accessor: an alias for `Reg<GICD_ITARGETSR0_SPEC>`"]
pub type GICD_ITARGETSR0 = crate::Reg<gicd_itargetsr0::GICD_ITARGETSR0_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr0;
#[doc = "GICD_ITARGETSR1 (r) register accessor: an alias for `Reg<GICD_ITARGETSR1_SPEC>`"]
pub type GICD_ITARGETSR1 = crate::Reg<gicd_itargetsr1::GICD_ITARGETSR1_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr1;
#[doc = "GICD_ITARGETSR2 (r) register accessor: an alias for `Reg<GICD_ITARGETSR2_SPEC>`"]
pub type GICD_ITARGETSR2 = crate::Reg<gicd_itargetsr2::GICD_ITARGETSR2_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr2;
#[doc = "GICD_ITARGETSR3 (r) register accessor: an alias for `Reg<GICD_ITARGETSR3_SPEC>`"]
pub type GICD_ITARGETSR3 = crate::Reg<gicd_itargetsr3::GICD_ITARGETSR3_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr3;
#[doc = "GICD_ITARGETSR4 (r) register accessor: an alias for `Reg<GICD_ITARGETSR4_SPEC>`"]
pub type GICD_ITARGETSR4 = crate::Reg<gicd_itargetsr4::GICD_ITARGETSR4_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr4;
#[doc = "GICD_ITARGETSR5 (r) register accessor: an alias for `Reg<GICD_ITARGETSR5_SPEC>`"]
pub type GICD_ITARGETSR5 = crate::Reg<gicd_itargetsr5::GICD_ITARGETSR5_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr5;
#[doc = "GICD_ITARGETSR6 (r) register accessor: an alias for `Reg<GICD_ITARGETSR6_SPEC>`"]
pub type GICD_ITARGETSR6 = crate::Reg<gicd_itargetsr6::GICD_ITARGETSR6_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr6;
#[doc = "GICD_ITARGETSR7 (r) register accessor: an alias for `Reg<GICD_ITARGETSR7_SPEC>`"]
pub type GICD_ITARGETSR7 = crate::Reg<gicd_itargetsr7::GICD_ITARGETSR7_SPEC>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr7;
#[doc = "GICD_ITARGETSR8 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR8_SPEC>`"]
pub type GICD_ITARGETSR8 = crate::Reg<gicd_itargetsr8::GICD_ITARGETSR8_SPEC>;
#[doc = "GICD interrupt processor target register 8"]
pub mod gicd_itargetsr8;
#[doc = "GICD_ITARGETSR9 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR9_SPEC>`"]
pub type GICD_ITARGETSR9 = crate::Reg<gicd_itargetsr9::GICD_ITARGETSR9_SPEC>;
#[doc = "GICD interrupt processor target register 9"]
pub mod gicd_itargetsr9;
#[doc = "GICD_ITARGETSR10 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR10_SPEC>`"]
pub type GICD_ITARGETSR10 = crate::Reg<gicd_itargetsr10::GICD_ITARGETSR10_SPEC>;
#[doc = "GICD interrupt processor target register 10"]
pub mod gicd_itargetsr10;
#[doc = "GICD_ITARGETSR11 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR11_SPEC>`"]
pub type GICD_ITARGETSR11 = crate::Reg<gicd_itargetsr11::GICD_ITARGETSR11_SPEC>;
#[doc = "GICD interrupt processor target register 11"]
pub mod gicd_itargetsr11;
#[doc = "GICD_ITARGETSR12 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR12_SPEC>`"]
pub type GICD_ITARGETSR12 = crate::Reg<gicd_itargetsr12::GICD_ITARGETSR12_SPEC>;
#[doc = "GICD interrupt processor target register 12"]
pub mod gicd_itargetsr12;
#[doc = "GICD_ITARGETSR13 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR13_SPEC>`"]
pub type GICD_ITARGETSR13 = crate::Reg<gicd_itargetsr13::GICD_ITARGETSR13_SPEC>;
#[doc = "GICD interrupt processor target register 13"]
pub mod gicd_itargetsr13;
#[doc = "GICD_ITARGETSR14 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR14_SPEC>`"]
pub type GICD_ITARGETSR14 = crate::Reg<gicd_itargetsr14::GICD_ITARGETSR14_SPEC>;
#[doc = "GICD interrupt processor target register 14"]
pub mod gicd_itargetsr14;
#[doc = "GICD_ITARGETSR15 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR15_SPEC>`"]
pub type GICD_ITARGETSR15 = crate::Reg<gicd_itargetsr15::GICD_ITARGETSR15_SPEC>;
#[doc = "GICD interrupt processor target register 15"]
pub mod gicd_itargetsr15;
#[doc = "GICD_ITARGETSR16 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR16_SPEC>`"]
pub type GICD_ITARGETSR16 = crate::Reg<gicd_itargetsr16::GICD_ITARGETSR16_SPEC>;
#[doc = "GICD interrupt processor target register 16"]
pub mod gicd_itargetsr16;
#[doc = "GICD_ITARGETSR17 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR17_SPEC>`"]
pub type GICD_ITARGETSR17 = crate::Reg<gicd_itargetsr17::GICD_ITARGETSR17_SPEC>;
#[doc = "GICD interrupt processor target register 17"]
pub mod gicd_itargetsr17;
#[doc = "GICD_ITARGETSR18 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR18_SPEC>`"]
pub type GICD_ITARGETSR18 = crate::Reg<gicd_itargetsr18::GICD_ITARGETSR18_SPEC>;
#[doc = "GICD interrupt processor target register 18"]
pub mod gicd_itargetsr18;
#[doc = "GICD_ITARGETSR19 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR19_SPEC>`"]
pub type GICD_ITARGETSR19 = crate::Reg<gicd_itargetsr19::GICD_ITARGETSR19_SPEC>;
#[doc = "GICD interrupt processor target register 19"]
pub mod gicd_itargetsr19;
#[doc = "GICD_ITARGETSR20 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR20_SPEC>`"]
pub type GICD_ITARGETSR20 = crate::Reg<gicd_itargetsr20::GICD_ITARGETSR20_SPEC>;
#[doc = "GICD interrupt processor target register 20"]
pub mod gicd_itargetsr20;
#[doc = "GICD_ITARGETSR21 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR21_SPEC>`"]
pub type GICD_ITARGETSR21 = crate::Reg<gicd_itargetsr21::GICD_ITARGETSR21_SPEC>;
#[doc = "GICD interrupt processor target register 21"]
pub mod gicd_itargetsr21;
#[doc = "GICD_ITARGETSR22 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR22_SPEC>`"]
pub type GICD_ITARGETSR22 = crate::Reg<gicd_itargetsr22::GICD_ITARGETSR22_SPEC>;
#[doc = "GICD interrupt processor target register 22"]
pub mod gicd_itargetsr22;
#[doc = "GICD_ITARGETSR23 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR23_SPEC>`"]
pub type GICD_ITARGETSR23 = crate::Reg<gicd_itargetsr23::GICD_ITARGETSR23_SPEC>;
#[doc = "GICD interrupt processor target register 23"]
pub mod gicd_itargetsr23;
#[doc = "GICD_ITARGETSR24 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR24_SPEC>`"]
pub type GICD_ITARGETSR24 = crate::Reg<gicd_itargetsr24::GICD_ITARGETSR24_SPEC>;
#[doc = "GICD interrupt processor target register 24"]
pub mod gicd_itargetsr24;
#[doc = "GICD_ITARGETSR25 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR25_SPEC>`"]
pub type GICD_ITARGETSR25 = crate::Reg<gicd_itargetsr25::GICD_ITARGETSR25_SPEC>;
#[doc = "GICD interrupt processor target register 25"]
pub mod gicd_itargetsr25;
#[doc = "GICD_ITARGETSR26 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR26_SPEC>`"]
pub type GICD_ITARGETSR26 = crate::Reg<gicd_itargetsr26::GICD_ITARGETSR26_SPEC>;
#[doc = "GICD interrupt processor target register 26"]
pub mod gicd_itargetsr26;
#[doc = "GICD_ITARGETSR27 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR27_SPEC>`"]
pub type GICD_ITARGETSR27 = crate::Reg<gicd_itargetsr27::GICD_ITARGETSR27_SPEC>;
#[doc = "GICD interrupt processor target register 27"]
pub mod gicd_itargetsr27;
#[doc = "GICD_ITARGETSR28 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR28_SPEC>`"]
pub type GICD_ITARGETSR28 = crate::Reg<gicd_itargetsr28::GICD_ITARGETSR28_SPEC>;
#[doc = "GICD interrupt processor target register 28"]
pub mod gicd_itargetsr28;
#[doc = "GICD_ITARGETSR29 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR29_SPEC>`"]
pub type GICD_ITARGETSR29 = crate::Reg<gicd_itargetsr29::GICD_ITARGETSR29_SPEC>;
#[doc = "GICD interrupt processor target register 29"]
pub mod gicd_itargetsr29;
#[doc = "GICD_ITARGETSR30 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR30_SPEC>`"]
pub type GICD_ITARGETSR30 = crate::Reg<gicd_itargetsr30::GICD_ITARGETSR30_SPEC>;
#[doc = "GICD interrupt processor target register 30"]
pub mod gicd_itargetsr30;
#[doc = "GICD_ITARGETSR31 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR31_SPEC>`"]
pub type GICD_ITARGETSR31 = crate::Reg<gicd_itargetsr31::GICD_ITARGETSR31_SPEC>;
#[doc = "GICD interrupt processor target register 31"]
pub mod gicd_itargetsr31;
#[doc = "GICD_ITARGETSR32 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR32_SPEC>`"]
pub type GICD_ITARGETSR32 = crate::Reg<gicd_itargetsr32::GICD_ITARGETSR32_SPEC>;
#[doc = "GICD interrupt processor target register 32"]
pub mod gicd_itargetsr32;
#[doc = "GICD_ITARGETSR33 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR33_SPEC>`"]
pub type GICD_ITARGETSR33 = crate::Reg<gicd_itargetsr33::GICD_ITARGETSR33_SPEC>;
#[doc = "GICD interrupt processor target register 33"]
pub mod gicd_itargetsr33;
#[doc = "GICD_ITARGETSR34 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR34_SPEC>`"]
pub type GICD_ITARGETSR34 = crate::Reg<gicd_itargetsr34::GICD_ITARGETSR34_SPEC>;
#[doc = "GICD interrupt processor target register 34"]
pub mod gicd_itargetsr34;
#[doc = "GICD_ITARGETSR35 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR35_SPEC>`"]
pub type GICD_ITARGETSR35 = crate::Reg<gicd_itargetsr35::GICD_ITARGETSR35_SPEC>;
#[doc = "GICD interrupt processor target register 35"]
pub mod gicd_itargetsr35;
#[doc = "GICD_ITARGETSR36 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR36_SPEC>`"]
pub type GICD_ITARGETSR36 = crate::Reg<gicd_itargetsr36::GICD_ITARGETSR36_SPEC>;
#[doc = "GICD interrupt processor target register 36"]
pub mod gicd_itargetsr36;
#[doc = "GICD_ITARGETSR37 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR37_SPEC>`"]
pub type GICD_ITARGETSR37 = crate::Reg<gicd_itargetsr37::GICD_ITARGETSR37_SPEC>;
#[doc = "GICD interrupt processor target register 37"]
pub mod gicd_itargetsr37;
#[doc = "GICD_ITARGETSR38 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR38_SPEC>`"]
pub type GICD_ITARGETSR38 = crate::Reg<gicd_itargetsr38::GICD_ITARGETSR38_SPEC>;
#[doc = "GICD interrupt processor target register 38"]
pub mod gicd_itargetsr38;
#[doc = "GICD_ITARGETSR39 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR39_SPEC>`"]
pub type GICD_ITARGETSR39 = crate::Reg<gicd_itargetsr39::GICD_ITARGETSR39_SPEC>;
#[doc = "GICD interrupt processor target register 39"]
pub mod gicd_itargetsr39;
#[doc = "GICD_ITARGETSR40 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR40_SPEC>`"]
pub type GICD_ITARGETSR40 = crate::Reg<gicd_itargetsr40::GICD_ITARGETSR40_SPEC>;
#[doc = "GICD interrupt processor target register 40"]
pub mod gicd_itargetsr40;
#[doc = "GICD_ITARGETSR41 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR41_SPEC>`"]
pub type GICD_ITARGETSR41 = crate::Reg<gicd_itargetsr41::GICD_ITARGETSR41_SPEC>;
#[doc = "GICD interrupt processor target register 41"]
pub mod gicd_itargetsr41;
#[doc = "GICD_ITARGETSR42 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR42_SPEC>`"]
pub type GICD_ITARGETSR42 = crate::Reg<gicd_itargetsr42::GICD_ITARGETSR42_SPEC>;
#[doc = "GICD interrupt processor target register 42"]
pub mod gicd_itargetsr42;
#[doc = "GICD_ITARGETSR43 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR43_SPEC>`"]
pub type GICD_ITARGETSR43 = crate::Reg<gicd_itargetsr43::GICD_ITARGETSR43_SPEC>;
#[doc = "GICD interrupt processor target register 43"]
pub mod gicd_itargetsr43;
#[doc = "GICD_ITARGETSR44 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR44_SPEC>`"]
pub type GICD_ITARGETSR44 = crate::Reg<gicd_itargetsr44::GICD_ITARGETSR44_SPEC>;
#[doc = "GICD interrupt processor target register 44"]
pub mod gicd_itargetsr44;
#[doc = "GICD_ITARGETSR45 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR45_SPEC>`"]
pub type GICD_ITARGETSR45 = crate::Reg<gicd_itargetsr45::GICD_ITARGETSR45_SPEC>;
#[doc = "GICD interrupt processor target register 45"]
pub mod gicd_itargetsr45;
#[doc = "GICD_ITARGETSR46 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR46_SPEC>`"]
pub type GICD_ITARGETSR46 = crate::Reg<gicd_itargetsr46::GICD_ITARGETSR46_SPEC>;
#[doc = "GICD interrupt processor target register 46"]
pub mod gicd_itargetsr46;
#[doc = "GICD_ITARGETSR47 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR47_SPEC>`"]
pub type GICD_ITARGETSR47 = crate::Reg<gicd_itargetsr47::GICD_ITARGETSR47_SPEC>;
#[doc = "GICD interrupt processor target register 47"]
pub mod gicd_itargetsr47;
#[doc = "GICD_ITARGETSR48 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR48_SPEC>`"]
pub type GICD_ITARGETSR48 = crate::Reg<gicd_itargetsr48::GICD_ITARGETSR48_SPEC>;
#[doc = "GICD interrupt processor target register 48"]
pub mod gicd_itargetsr48;
#[doc = "GICD_ITARGETSR49 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR49_SPEC>`"]
pub type GICD_ITARGETSR49 = crate::Reg<gicd_itargetsr49::GICD_ITARGETSR49_SPEC>;
#[doc = "GICD interrupt processor target register 49"]
pub mod gicd_itargetsr49;
#[doc = "GICD_ITARGETSR50 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR50_SPEC>`"]
pub type GICD_ITARGETSR50 = crate::Reg<gicd_itargetsr50::GICD_ITARGETSR50_SPEC>;
#[doc = "GICD interrupt processor target register 50"]
pub mod gicd_itargetsr50;
#[doc = "GICD_ITARGETSR51 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR51_SPEC>`"]
pub type GICD_ITARGETSR51 = crate::Reg<gicd_itargetsr51::GICD_ITARGETSR51_SPEC>;
#[doc = "GICD interrupt processor target register 51"]
pub mod gicd_itargetsr51;
#[doc = "GICD_ITARGETSR52 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR52_SPEC>`"]
pub type GICD_ITARGETSR52 = crate::Reg<gicd_itargetsr52::GICD_ITARGETSR52_SPEC>;
#[doc = "GICD interrupt processor target register 52"]
pub mod gicd_itargetsr52;
#[doc = "GICD_ITARGETSR53 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR53_SPEC>`"]
pub type GICD_ITARGETSR53 = crate::Reg<gicd_itargetsr53::GICD_ITARGETSR53_SPEC>;
#[doc = "GICD interrupt processor target register 53"]
pub mod gicd_itargetsr53;
#[doc = "GICD_ITARGETSR54 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR54_SPEC>`"]
pub type GICD_ITARGETSR54 = crate::Reg<gicd_itargetsr54::GICD_ITARGETSR54_SPEC>;
#[doc = "GICD interrupt processor target register 54"]
pub mod gicd_itargetsr54;
#[doc = "GICD_ITARGETSR55 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR55_SPEC>`"]
pub type GICD_ITARGETSR55 = crate::Reg<gicd_itargetsr55::GICD_ITARGETSR55_SPEC>;
#[doc = "GICD interrupt processor target register 55"]
pub mod gicd_itargetsr55;
#[doc = "GICD_ITARGETSR56 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR56_SPEC>`"]
pub type GICD_ITARGETSR56 = crate::Reg<gicd_itargetsr56::GICD_ITARGETSR56_SPEC>;
#[doc = "GICD interrupt processor target register 56"]
pub mod gicd_itargetsr56;
#[doc = "GICD_ITARGETSR57 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR57_SPEC>`"]
pub type GICD_ITARGETSR57 = crate::Reg<gicd_itargetsr57::GICD_ITARGETSR57_SPEC>;
#[doc = "GICD interrupt processor target register 57"]
pub mod gicd_itargetsr57;
#[doc = "GICD_ITARGETSR58 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR58_SPEC>`"]
pub type GICD_ITARGETSR58 = crate::Reg<gicd_itargetsr58::GICD_ITARGETSR58_SPEC>;
#[doc = "GICD interrupt processor target register 58"]
pub mod gicd_itargetsr58;
#[doc = "GICD_ITARGETSR59 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR59_SPEC>`"]
pub type GICD_ITARGETSR59 = crate::Reg<gicd_itargetsr59::GICD_ITARGETSR59_SPEC>;
#[doc = "GICD interrupt processor target register 59"]
pub mod gicd_itargetsr59;
#[doc = "GICD_ITARGETSR60 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR60_SPEC>`"]
pub type GICD_ITARGETSR60 = crate::Reg<gicd_itargetsr60::GICD_ITARGETSR60_SPEC>;
#[doc = "GICD interrupt processor target register 60"]
pub mod gicd_itargetsr60;
#[doc = "GICD_ITARGETSR61 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR61_SPEC>`"]
pub type GICD_ITARGETSR61 = crate::Reg<gicd_itargetsr61::GICD_ITARGETSR61_SPEC>;
#[doc = "GICD interrupt processor target register 61"]
pub mod gicd_itargetsr61;
#[doc = "GICD_ITARGETSR62 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR62_SPEC>`"]
pub type GICD_ITARGETSR62 = crate::Reg<gicd_itargetsr62::GICD_ITARGETSR62_SPEC>;
#[doc = "GICD interrupt processor target register 62"]
pub mod gicd_itargetsr62;
#[doc = "GICD_ITARGETSR63 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR63_SPEC>`"]
pub type GICD_ITARGETSR63 = crate::Reg<gicd_itargetsr63::GICD_ITARGETSR63_SPEC>;
#[doc = "GICD interrupt processor target register 63"]
pub mod gicd_itargetsr63;
#[doc = "GICD_ITARGETSR64 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR64_SPEC>`"]
pub type GICD_ITARGETSR64 = crate::Reg<gicd_itargetsr64::GICD_ITARGETSR64_SPEC>;
#[doc = "GICD interrupt processor target register 64"]
pub mod gicd_itargetsr64;
#[doc = "GICD_ITARGETSR65 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR65_SPEC>`"]
pub type GICD_ITARGETSR65 = crate::Reg<gicd_itargetsr65::GICD_ITARGETSR65_SPEC>;
#[doc = "GICD interrupt processor target register 65"]
pub mod gicd_itargetsr65;
#[doc = "GICD_ITARGETSR66 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR66_SPEC>`"]
pub type GICD_ITARGETSR66 = crate::Reg<gicd_itargetsr66::GICD_ITARGETSR66_SPEC>;
#[doc = "GICD interrupt processor target register 66"]
pub mod gicd_itargetsr66;
#[doc = "GICD_ITARGETSR67 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR67_SPEC>`"]
pub type GICD_ITARGETSR67 = crate::Reg<gicd_itargetsr67::GICD_ITARGETSR67_SPEC>;
#[doc = "GICD interrupt processor target register 67"]
pub mod gicd_itargetsr67;
#[doc = "GICD_ITARGETSR68 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR68_SPEC>`"]
pub type GICD_ITARGETSR68 = crate::Reg<gicd_itargetsr68::GICD_ITARGETSR68_SPEC>;
#[doc = "GICD interrupt processor target register 68"]
pub mod gicd_itargetsr68;
#[doc = "GICD_ITARGETSR69 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR69_SPEC>`"]
pub type GICD_ITARGETSR69 = crate::Reg<gicd_itargetsr69::GICD_ITARGETSR69_SPEC>;
#[doc = "GICD interrupt processor target register 69"]
pub mod gicd_itargetsr69;
#[doc = "GICD_ITARGETSR70 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR70_SPEC>`"]
pub type GICD_ITARGETSR70 = crate::Reg<gicd_itargetsr70::GICD_ITARGETSR70_SPEC>;
#[doc = "GICD interrupt processor target register 70"]
pub mod gicd_itargetsr70;
#[doc = "GICD_ITARGETSR71 (rw) register accessor: an alias for `Reg<GICD_ITARGETSR71_SPEC>`"]
pub type GICD_ITARGETSR71 = crate::Reg<gicd_itargetsr71::GICD_ITARGETSR71_SPEC>;
#[doc = "GICD interrupt processor target register 71"]
pub mod gicd_itargetsr71;
#[doc = "GICD_ICFGR0 (rw) register accessor: an alias for `Reg<GICD_ICFGR0_SPEC>`"]
pub type GICD_ICFGR0 = crate::Reg<gicd_icfgr0::GICD_ICFGR0_SPEC>;
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr0;
#[doc = "GICD_ICFGR1 (rw) register accessor: an alias for `Reg<GICD_ICFGR1_SPEC>`"]
pub type GICD_ICFGR1 = crate::Reg<gicd_icfgr1::GICD_ICFGR1_SPEC>;
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr1;
#[doc = "GICD_ICFGR2 (rw) register accessor: an alias for `Reg<GICD_ICFGR2_SPEC>`"]
pub type GICD_ICFGR2 = crate::Reg<gicd_icfgr2::GICD_ICFGR2_SPEC>;
#[doc = "GICD interrupt configuration register 2"]
pub mod gicd_icfgr2;
#[doc = "GICD_ICFGR3 (rw) register accessor: an alias for `Reg<GICD_ICFGR3_SPEC>`"]
pub type GICD_ICFGR3 = crate::Reg<gicd_icfgr3::GICD_ICFGR3_SPEC>;
#[doc = "GICD interrupt configuration register 3"]
pub mod gicd_icfgr3;
#[doc = "GICD_ICFGR4 (rw) register accessor: an alias for `Reg<GICD_ICFGR4_SPEC>`"]
pub type GICD_ICFGR4 = crate::Reg<gicd_icfgr4::GICD_ICFGR4_SPEC>;
#[doc = "GICD interrupt configuration register 4"]
pub mod gicd_icfgr4;
#[doc = "GICD_ICFGR5 (rw) register accessor: an alias for `Reg<GICD_ICFGR5_SPEC>`"]
pub type GICD_ICFGR5 = crate::Reg<gicd_icfgr5::GICD_ICFGR5_SPEC>;
#[doc = "GICD interrupt configuration register 5"]
pub mod gicd_icfgr5;
#[doc = "GICD_ICFGR6 (rw) register accessor: an alias for `Reg<GICD_ICFGR6_SPEC>`"]
pub type GICD_ICFGR6 = crate::Reg<gicd_icfgr6::GICD_ICFGR6_SPEC>;
#[doc = "GICD interrupt configuration register 6"]
pub mod gicd_icfgr6;
#[doc = "GICD_ICFGR7 (rw) register accessor: an alias for `Reg<GICD_ICFGR7_SPEC>`"]
pub type GICD_ICFGR7 = crate::Reg<gicd_icfgr7::GICD_ICFGR7_SPEC>;
#[doc = "GICD interrupt configuration register 7"]
pub mod gicd_icfgr7;
#[doc = "GICD_ICFGR8 (rw) register accessor: an alias for `Reg<GICD_ICFGR8_SPEC>`"]
pub type GICD_ICFGR8 = crate::Reg<gicd_icfgr8::GICD_ICFGR8_SPEC>;
#[doc = "GICD interrupt configuration register 8"]
pub mod gicd_icfgr8;
#[doc = "GICD_ICFGR9 (rw) register accessor: an alias for `Reg<GICD_ICFGR9_SPEC>`"]
pub type GICD_ICFGR9 = crate::Reg<gicd_icfgr9::GICD_ICFGR9_SPEC>;
#[doc = "GICD interrupt configuration register 9"]
pub mod gicd_icfgr9;
#[doc = "GICD_ICFGR10 (rw) register accessor: an alias for `Reg<GICD_ICFGR10_SPEC>`"]
pub type GICD_ICFGR10 = crate::Reg<gicd_icfgr10::GICD_ICFGR10_SPEC>;
#[doc = "GICD interrupt configuration register 10"]
pub mod gicd_icfgr10;
#[doc = "GICD_ICFGR11 (rw) register accessor: an alias for `Reg<GICD_ICFGR11_SPEC>`"]
pub type GICD_ICFGR11 = crate::Reg<gicd_icfgr11::GICD_ICFGR11_SPEC>;
#[doc = "GICD interrupt configuration register 11"]
pub mod gicd_icfgr11;
#[doc = "GICD_ICFGR12 (rw) register accessor: an alias for `Reg<GICD_ICFGR12_SPEC>`"]
pub type GICD_ICFGR12 = crate::Reg<gicd_icfgr12::GICD_ICFGR12_SPEC>;
#[doc = "GICD interrupt configuration register 12"]
pub mod gicd_icfgr12;
#[doc = "GICD_ICFGR13 (rw) register accessor: an alias for `Reg<GICD_ICFGR13_SPEC>`"]
pub type GICD_ICFGR13 = crate::Reg<gicd_icfgr13::GICD_ICFGR13_SPEC>;
#[doc = "GICD interrupt configuration register 13"]
pub mod gicd_icfgr13;
#[doc = "GICD_ICFGR14 (rw) register accessor: an alias for `Reg<GICD_ICFGR14_SPEC>`"]
pub type GICD_ICFGR14 = crate::Reg<gicd_icfgr14::GICD_ICFGR14_SPEC>;
#[doc = "GICD interrupt configuration register 14"]
pub mod gicd_icfgr14;
#[doc = "GICD_ICFGR15 (rw) register accessor: an alias for `Reg<GICD_ICFGR15_SPEC>`"]
pub type GICD_ICFGR15 = crate::Reg<gicd_icfgr15::GICD_ICFGR15_SPEC>;
#[doc = "GICD interrupt configuration register 15"]
pub mod gicd_icfgr15;
#[doc = "GICD_ICFGR16 (rw) register accessor: an alias for `Reg<GICD_ICFGR16_SPEC>`"]
pub type GICD_ICFGR16 = crate::Reg<gicd_icfgr16::GICD_ICFGR16_SPEC>;
#[doc = "GICD interrupt configuration register 16"]
pub mod gicd_icfgr16;
#[doc = "GICD_ICFGR17 (rw) register accessor: an alias for `Reg<GICD_ICFGR17_SPEC>`"]
pub type GICD_ICFGR17 = crate::Reg<gicd_icfgr17::GICD_ICFGR17_SPEC>;
#[doc = "GICD interrupt configuration register 17"]
pub mod gicd_icfgr17;
#[doc = "GICD_PPISR (r) register accessor: an alias for `Reg<GICD_PPISR_SPEC>`"]
pub type GICD_PPISR = crate::Reg<gicd_ppisr::GICD_PPISR_SPEC>;
#[doc = "GICD private peripheral interrupt status register"]
pub mod gicd_ppisr;
#[doc = "GICD_SPISR1 (r) register accessor: an alias for `Reg<GICD_SPISR1_SPEC>`"]
pub type GICD_SPISR1 = crate::Reg<gicd_spisr1::GICD_SPISR1_SPEC>;
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
pub mod gicd_spisr1;
#[doc = "GICD_SPISR2 (r) register accessor: an alias for `Reg<GICD_SPISR2_SPEC>`"]
pub type GICD_SPISR2 = crate::Reg<gicd_spisr2::GICD_SPISR2_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr2;
#[doc = "GICD_SPISR3 (r) register accessor: an alias for `Reg<GICD_SPISR3_SPEC>`"]
pub type GICD_SPISR3 = crate::Reg<gicd_spisr3::GICD_SPISR3_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr3;
#[doc = "GICD_SPISR4 (r) register accessor: an alias for `Reg<GICD_SPISR4_SPEC>`"]
pub type GICD_SPISR4 = crate::Reg<gicd_spisr4::GICD_SPISR4_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr4;
#[doc = "GICD_SPISR5 (r) register accessor: an alias for `Reg<GICD_SPISR5_SPEC>`"]
pub type GICD_SPISR5 = crate::Reg<gicd_spisr5::GICD_SPISR5_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr5;
#[doc = "GICD_SPISR6 (r) register accessor: an alias for `Reg<GICD_SPISR6_SPEC>`"]
pub type GICD_SPISR6 = crate::Reg<gicd_spisr6::GICD_SPISR6_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr6;
#[doc = "GICD_SPISR7 (r) register accessor: an alias for `Reg<GICD_SPISR7_SPEC>`"]
pub type GICD_SPISR7 = crate::Reg<gicd_spisr7::GICD_SPISR7_SPEC>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr7;
#[doc = "GICD_SGIR (w) register accessor: an alias for `Reg<GICD_SGIR_SPEC>`"]
pub type GICD_SGIR = crate::Reg<gicd_sgir::GICD_SGIR_SPEC>;
#[doc = "GICD software generated interrupt register"]
pub mod gicd_sgir;
#[doc = "GICD_CPENDSGIR0 (rw) register accessor: an alias for `Reg<GICD_CPENDSGIR0_SPEC>`"]
pub type GICD_CPENDSGIR0 = crate::Reg<gicd_cpendsgir0::GICD_CPENDSGIR0_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir0;
#[doc = "GICD_CPENDSGIR1 (rw) register accessor: an alias for `Reg<GICD_CPENDSGIR1_SPEC>`"]
pub type GICD_CPENDSGIR1 = crate::Reg<gicd_cpendsgir1::GICD_CPENDSGIR1_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir1;
#[doc = "GICD_CPENDSGIR2 (rw) register accessor: an alias for `Reg<GICD_CPENDSGIR2_SPEC>`"]
pub type GICD_CPENDSGIR2 = crate::Reg<gicd_cpendsgir2::GICD_CPENDSGIR2_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir2;
#[doc = "GICD_CPENDSGIR3 (rw) register accessor: an alias for `Reg<GICD_CPENDSGIR3_SPEC>`"]
pub type GICD_CPENDSGIR3 = crate::Reg<gicd_cpendsgir3::GICD_CPENDSGIR3_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir3;
#[doc = "GICD_SPENDSGIR0 (rw) register accessor: an alias for `Reg<GICD_SPENDSGIR0_SPEC>`"]
pub type GICD_SPENDSGIR0 = crate::Reg<gicd_spendsgir0::GICD_SPENDSGIR0_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir0;
#[doc = "GICD_SPENDSGIR1 (rw) register accessor: an alias for `Reg<GICD_SPENDSGIR1_SPEC>`"]
pub type GICD_SPENDSGIR1 = crate::Reg<gicd_spendsgir1::GICD_SPENDSGIR1_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir1;
#[doc = "GICD_SPENDSGIR2 (rw) register accessor: an alias for `Reg<GICD_SPENDSGIR2_SPEC>`"]
pub type GICD_SPENDSGIR2 = crate::Reg<gicd_spendsgir2::GICD_SPENDSGIR2_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir2;
#[doc = "GICD_SPENDSGIR3 (rw) register accessor: an alias for `Reg<GICD_SPENDSGIR3_SPEC>`"]
pub type GICD_SPENDSGIR3 = crate::Reg<gicd_spendsgir3::GICD_SPENDSGIR3_SPEC>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir3;
#[doc = "GICD_PIDR4 (r) register accessor: an alias for `Reg<GICD_PIDR4_SPEC>`"]
pub type GICD_PIDR4 = crate::Reg<gicd_pidr4::GICD_PIDR4_SPEC>;
#[doc = "GICD peripheral ID4 register"]
pub mod gicd_pidr4;
#[doc = "GICD_PIDR5 (r) register accessor: an alias for `Reg<GICD_PIDR5_SPEC>`"]
pub type GICD_PIDR5 = crate::Reg<gicd_pidr5::GICD_PIDR5_SPEC>;
#[doc = "GICD peripheral ID5 to ID7 register 5"]
pub mod gicd_pidr5;
#[doc = "GICD_PIDR6 (r) register accessor: an alias for `Reg<GICD_PIDR6_SPEC>`"]
pub type GICD_PIDR6 = crate::Reg<gicd_pidr6::GICD_PIDR6_SPEC>;
#[doc = "GICD peripheral ID5 to ID7 register 6"]
pub mod gicd_pidr6;
#[doc = "GICD_PIDR7 (r) register accessor: an alias for `Reg<GICD_PIDR7_SPEC>`"]
pub type GICD_PIDR7 = crate::Reg<gicd_pidr7::GICD_PIDR7_SPEC>;
#[doc = "GICD peripheral ID5 to ID7 register 7"]
pub mod gicd_pidr7;
#[doc = "GICD_PIDR0 (r) register accessor: an alias for `Reg<GICD_PIDR0_SPEC>`"]
pub type GICD_PIDR0 = crate::Reg<gicd_pidr0::GICD_PIDR0_SPEC>;
#[doc = "GICD peripheral ID0 register"]
pub mod gicd_pidr0;
#[doc = "GICD_PIDR1 (r) register accessor: an alias for `Reg<GICD_PIDR1_SPEC>`"]
pub type GICD_PIDR1 = crate::Reg<gicd_pidr1::GICD_PIDR1_SPEC>;
#[doc = "GICD peripheral ID1 register"]
pub mod gicd_pidr1;
#[doc = "GICD_PIDR2 (r) register accessor: an alias for `Reg<GICD_PIDR2_SPEC>`"]
pub type GICD_PIDR2 = crate::Reg<gicd_pidr2::GICD_PIDR2_SPEC>;
#[doc = "GICD peripheral ID2 register"]
pub mod gicd_pidr2;
#[doc = "GICD_PIDR3 (r) register accessor: an alias for `Reg<GICD_PIDR3_SPEC>`"]
pub type GICD_PIDR3 = crate::Reg<gicd_pidr3::GICD_PIDR3_SPEC>;
#[doc = "GICD peripheral ID3 register"]
pub mod gicd_pidr3;
#[doc = "GICD_CIDR0 (r) register accessor: an alias for `Reg<GICD_CIDR0_SPEC>`"]
pub type GICD_CIDR0 = crate::Reg<gicd_cidr0::GICD_CIDR0_SPEC>;
#[doc = "GICD component ID0 register"]
pub mod gicd_cidr0;
#[doc = "GICD_CIDR1 (r) register accessor: an alias for `Reg<GICD_CIDR1_SPEC>`"]
pub type GICD_CIDR1 = crate::Reg<gicd_cidr1::GICD_CIDR1_SPEC>;
#[doc = "GICD component ID1 register"]
pub mod gicd_cidr1;
#[doc = "GICD_CIDR2 (r) register accessor: an alias for `Reg<GICD_CIDR2_SPEC>`"]
pub type GICD_CIDR2 = crate::Reg<gicd_cidr2::GICD_CIDR2_SPEC>;
#[doc = "GICD component ID2 register"]
pub mod gicd_cidr2;
#[doc = "GICD_CIDR3 (r) register accessor: an alias for `Reg<GICD_CIDR3_SPEC>`"]
pub type GICD_CIDR3 = crate::Reg<gicd_cidr3::GICD_CIDR3_SPEC>;
#[doc = "GICD component ID3 register"]
pub mod gicd_cidr3;
