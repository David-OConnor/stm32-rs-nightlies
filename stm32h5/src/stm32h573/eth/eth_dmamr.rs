#[doc = "Register `ETH_DMAMR` reader"]
pub struct R(crate::R<ETH_DMAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAMR` writer"]
pub struct W(crate::W<ETH_DMAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAMR_SPEC>;
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
impl From<crate::W<ETH_DMAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAMR_SPEC, bool, O>;
#[doc = "Field `DA` reader - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\\[14:12\\]
and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path."]
pub type DA_R = crate::BitReader<bool>;
#[doc = "Field `DA` writer - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\\[14:12\\]
and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path."]
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAMR_SPEC, bool, O>;
#[doc = "Field `TXPR` reader - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
pub type TXPR_R = crate::BitReader<bool>;
#[doc = "Field `TXPR` writer - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
pub type TXPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAMR_SPEC, bool, O>;
#[doc = "Field `PR` reader - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set."]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set."]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTM` reader - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table�651: Transfer complete interrupt behavior)."]
pub type INTM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTM` writer - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table�651: Transfer complete interrupt behavior)."]
pub type INTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAMR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\\[14:12\\]
and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table�651: Transfer complete interrupt behavior)."]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC. This bit is automatically cleared after the reset operation is complete in all clock domains. Before reprogramming any register, a value of zero should be read in this bit. Note: The reset operation is complete only when all resets in all active clock domains are deasserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The priority between the paths is according to the priority specified in Bits\\[14:12\\]
and the priority weight is specified in the TXPR bit. The Tx path has priority over the Rx path when the TXPR bit is set. Otherwise, the Rx path has priority over the Tx path."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    #[doc = "Bit 11 - Transmit priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<11> {
        TXPR_W::new(self)
    }
    #[doc = "Bits 12:14 - Priority ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when the DA bit is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether the TXPR bit is reset or set."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<12> {
        PR_W::new(self)
    }
    #[doc = "Bits 16:17 - Interrupt Mode This field defines the interrupt mode of the Ethernet peripheral. The behavior of the interrupt signal and of the RI/TI bits in the ETH_DMACSR register changes depending on the INTM value (refer to Table�651: Transfer complete interrupt behavior)."]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<16> {
        INTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmamr](index.html) module"]
pub struct ETH_DMAMR_SPEC;
impl crate::RegisterSpec for ETH_DMAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmamr::R](R) reader structure"]
impl crate::Readable for ETH_DMAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmamr::W](W) writer structure"]
impl crate::Writable for ETH_DMAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAMR to value 0"]
impl crate::Resettable for ETH_DMAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
