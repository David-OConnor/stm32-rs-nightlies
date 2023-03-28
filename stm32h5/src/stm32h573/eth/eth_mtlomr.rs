#[doc = "Register `ETH_MTLOMR` reader"]
pub struct R(crate::R<ETH_MTLOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLOMR` writer"]
pub struct W(crate::W<ETH_MTLOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLOMR_SPEC>;
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
impl From<crate::W<ETH_MTLOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL. When this bit is reset, the Tx packet status received from the MAC is forwarded to the application."]
pub type DTXSTS_R = crate::BitReader<bool>;
#[doc = "Field `DTXSTS` writer - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL. When this bit is reset, the Tx packet status received from the MAC is forwarded to the application."]
pub type DTXSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
#[doc = "Field `CNTPRST` reader - Counters Preset When this bit is set: Tx queue underflow register (ETH_MTLTXQUR) is initialized/preset to 0x7F0. Missed Packet and Overflow Packet counters in Rx queue missed packet and overflow counter register (ETH_MTLRXQMPOCR) is initialized/preset to 0x7F0 This bit is cleared automatically."]
pub type CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRST` writer - Counters Preset When this bit is set: Tx queue underflow register (ETH_MTLTXQUR) is initialized/preset to 0x7F0. Missed Packet and Overflow Packet counters in Rx queue missed packet and overflow counter register (ETH_MTLRXQMPOCR) is initialized/preset to 0x7F0 This bit is cleared automatically."]
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
#[doc = "Field `CNTCLR` reader - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle. If this bit is set along with CNTPRST bit, CNTPRST has precedence."]
pub type CNTCLR_R = crate::BitReader<bool>;
#[doc = "Field `CNTCLR` writer - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle. If this bit is set along with CNTPRST bit, CNTPRST has precedence."]
pub type CNTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL. When this bit is reset, the Tx packet status received from the MAC is forwarded to the application."]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set: Tx queue underflow register (ETH_MTLTXQUR) is initialized/preset to 0x7F0. Missed Packet and Overflow Packet counters in Rx queue missed packet and overflow counter register (ETH_MTLRXQMPOCR) is initialized/preset to 0x7F0 This bit is cleared automatically."]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle. If this bit is set along with CNTPRST bit, CNTPRST has precedence."]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL. When this bit is reset, the Tx packet status received from the MAC is forwarded to the application."]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<1> {
        DTXSTS_W::new(self)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set: Tx queue underflow register (ETH_MTLTXQUR) is initialized/preset to 0x7F0. Missed Packet and Overflow Packet counters in Rx queue missed packet and overflow counter register (ETH_MTLRXQMPOCR) is initialized/preset to 0x7F0 This bit is cleared automatically."]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<8> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle. If this bit is set along with CNTPRST bit, CNTPRST has precedence."]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<9> {
        CNTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlomr](index.html) module"]
pub struct ETH_MTLOMR_SPEC;
impl crate::RegisterSpec for ETH_MTLOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlomr::R](R) reader structure"]
impl crate::Readable for ETH_MTLOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlomr::W](W) writer structure"]
impl crate::Writable for ETH_MTLOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLOMR to value 0"]
impl crate::Resettable for ETH_MTLOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
