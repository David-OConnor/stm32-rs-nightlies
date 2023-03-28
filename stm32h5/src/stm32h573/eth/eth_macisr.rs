#[doc = "Register `ETH_MACISR` reader"]
pub struct R(crate::R<ETH_MACISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACISR` writer"]
pub struct W(crate::W<ETH_MACISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH_MACISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYIS` reader - PHY Interrupt This bit is set when rising edge is detected on the ETH_PHY_INTN input. This bit is cleared when this register is read."]
pub type PHYIS_R = crate::BitReader<bool>;
#[doc = "Field `PMTIS` reader - PMT Interrupt Status This bit is set when a Magic packet or Wake-on-LAN packet is received in the power-down mode (RWKPRCVD and MGKPRCVD bits in ETH_MACPCSR register). This bit is cleared when Bits\\[6:5\\]
are cleared because of a Read operation to the PMT control status register (ETH_MACPCSR)."]
pub type PMTIS_R = crate::BitReader<bool>;
#[doc = "Field `LPIIS` reader - LPI Interrupt Status This bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared when the TLPIEN bit of LPI control and status register (ETH_MACLCSR) is read."]
pub type LPIIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCIS` reader - MMC Interrupt Status This bit is set high when MMCTXIS or MMCRXIS is set high. This bit is cleared only when all these bits are low."]
pub type MMCIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Rx interrupt register (ETH_MMC_RX_INTERRUPT). This bit is cleared when all bits in this interrupt register are cleared."]
pub type MMCRXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Tx interrupt register (ETH_MMC_TX_INTERRUPT). This bit is cleared when all bits in this interrupt register are cleared."]
pub type MMCTXIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status If the Timestamp feature is enabled, this bit is set when any of the following conditions is true: The system time value is equal to or exceeds the value specified in the Target Time High and Low registers. There is an overflow in the Seconds register. The Target Time Error occurred, that is, programmed target time already elapsed. If the Auxiliary Snapshot feature is enabled, this bit is set when the auxiliary snapshot trigger is asserted. When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers. When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers, for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the Timestamp status register (ETH_MACTSSR).\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TSIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` writer - Timestamp Interrupt Status If the Timestamp feature is enabled, this bit is set when any of the following conditions is true: The system time value is equal to or exceeds the value specified in the Target Time High and Low registers. There is an overflow in the Seconds register. The Target Time Error occurred, that is, programmed target time already elapsed. If the Auxiliary Snapshot feature is enabled, this bit is set when the auxiliary snapshot trigger is asserted. When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers. When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers, for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the Timestamp status register (ETH_MACTSSR)."]
pub type TSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACISR_SPEC, bool, O>;
#[doc = "Field `TXSTSIS` reader - Transmit Status Interrupt This bit indicates the status of transmitted packets. This bit is set when any of the following bits is set in the Rx Tx status register (ETH_MACRXTXSR): Excessive Collision (EXCOL) Late Collision (LCOL) Excessive Deferral (EXDEF) Loss of Carrier (LCARR) No Carrier (NCARR) Jabber Timeout (TJT) This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXSTSIS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSIS` writer - Transmit Status Interrupt This bit indicates the status of transmitted packets. This bit is set when any of the following bits is set in the Rx Tx status register (ETH_MACRXTXSR): Excessive Collision (EXCOL) Late Collision (LCOL) Excessive Deferral (EXDEF) Loss of Carrier (LCARR) No Carrier (NCARR) Jabber Timeout (TJT) This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
pub type TXSTSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACISR_SPEC, bool, O>;
#[doc = "Field `RXSTSIS` reader - Receive Status Interrupt This bit indicates the status of received packets. This bit is set when the RWT bit is set in the Rx Tx status register (ETH_MACRXTXSR). This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXSTSIS_R = crate::BitReader<bool>;
#[doc = "Field `RXSTSIS` writer - Receive Status Interrupt This bit indicates the status of received packets. This bit is set when the RWT bit is set in the Rx Tx status register (ETH_MACRXTXSR). This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
pub type RXSTSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt This bit is set when rising edge is detected on the ETH_PHY_INTN input. This bit is cleared when this register is read."]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Status This bit is set when a Magic packet or Wake-on-LAN packet is received in the power-down mode (RWKPRCVD and MGKPRCVD bits in ETH_MACPCSR register). This bit is cleared when Bits\\[6:5\\]
are cleared because of a Read operation to the PMT control status register (ETH_MACPCSR)."]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Status This bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared when the TLPIEN bit of LPI control and status register (ETH_MACLCSR) is read."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Interrupt Status This bit is set high when MMCTXIS or MMCRXIS is set high. This bit is cleared only when all these bits are low."]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Rx interrupt register (ETH_MMC_RX_INTERRUPT). This bit is cleared when all bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Tx interrupt register (ETH_MMC_TX_INTERRUPT). This bit is cleared when all bits in this interrupt register are cleared."]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Status If the Timestamp feature is enabled, this bit is set when any of the following conditions is true: The system time value is equal to or exceeds the value specified in the Target Time High and Low registers. There is an overflow in the Seconds register. The Target Time Error occurred, that is, programmed target time already elapsed. If the Auxiliary Snapshot feature is enabled, this bit is set when the auxiliary snapshot trigger is asserted. When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers. When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers, for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the Timestamp status register (ETH_MACTSSR)."]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt This bit indicates the status of transmitted packets. This bit is set when any of the following bits is set in the Rx Tx status register (ETH_MACRXTXSR): Excessive Collision (EXCOL) Late Collision (LCOL) Excessive Deferral (EXDEF) Loss of Carrier (LCARR) No Carrier (NCARR) Jabber Timeout (TJT) This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt This bit indicates the status of received packets. This bit is set when the RWT bit is set in the Rx Tx status register (ETH_MACRXTXSR). This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Timestamp Interrupt Status If the Timestamp feature is enabled, this bit is set when any of the following conditions is true: The system time value is equal to or exceeds the value specified in the Target Time High and Low registers. There is an overflow in the Seconds register. The Target Time Error occurred, that is, programmed target time already elapsed. If the Auxiliary Snapshot feature is enabled, this bit is set when the auxiliary snapshot trigger is asserted. When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers. When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR) registers, for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the Timestamp status register (ETH_MACTSSR)."]
    #[inline(always)]
    #[must_use]
    pub fn tsis(&mut self) -> TSIS_W<12> {
        TSIS_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt This bit indicates the status of transmitted packets. This bit is set when any of the following bits is set in the Rx Tx status register (ETH_MACRXTXSR): Excessive Collision (EXCOL) Late Collision (LCOL) Excessive Deferral (EXDEF) Loss of Carrier (LCARR) No Carrier (NCARR) Jabber Timeout (TJT) This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
    #[inline(always)]
    #[must_use]
    pub fn txstsis(&mut self) -> TXSTSIS_W<13> {
        TXSTSIS_W::new(self)
    }
    #[doc = "Bit 14 - Receive Status Interrupt This bit indicates the status of received packets. This bit is set when the RWT bit is set in the Rx Tx status register (ETH_MACRXTXSR). This bit is cleared when the corresponding interrupt source bit is read (or corresponding interrupt source bit is written to 1 when RCWE bit of CSR software control register (ETH_MACCSRSWCR) is set) in the ETH_MACISR register."]
    #[inline(always)]
    #[must_use]
    pub fn rxstsis(&mut self) -> RXSTSIS_W<14> {
        RXSTSIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macisr](index.html) module"]
pub struct ETH_MACISR_SPEC;
impl crate::RegisterSpec for ETH_MACISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macisr::R](R) reader structure"]
impl crate::Readable for ETH_MACISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macisr::W](W) writer structure"]
impl crate::Writable for ETH_MACISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACISR to value 0"]
impl crate::Resettable for ETH_MACISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
