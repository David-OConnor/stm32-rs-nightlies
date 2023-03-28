#[doc = "Register `MTLRxQDR` reader"]
pub struct R(crate::R<MTLRX_QDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRX_QDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRX_QDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRX_QDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLRxQDR` writer"]
pub struct W(crate::W<MTLRX_QDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLRX_QDR_SPEC>;
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
impl From<crate::W<MTLRX_QDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLRX_QDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RWCSTS` writer - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QDR_SPEC, bool, O>;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State"]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RRCSTS` writer - MTL Rx Queue Read Controller State"]
pub type RRCSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQSTS` writer - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue"]
pub type PRXQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRXQ` writer - Number of Packets in Receive Queue"]
pub type PRXQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QDR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    #[must_use]
    pub fn rwcsts(&mut self) -> RWCSTS_W<0> {
        RWCSTS_W::new(self)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    #[must_use]
    pub fn rrcsts(&mut self) -> RRCSTS_W<1> {
        RRCSTS_W::new(self)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxqsts(&mut self) -> RXQSTS_W<4> {
        RXQSTS_W::new(self)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    #[must_use]
    pub fn prxq(&mut self) -> PRXQ_W<16> {
        PRXQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qdr](index.html) module"]
pub struct MTLRX_QDR_SPEC;
impl crate::RegisterSpec for MTLRX_QDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qdr::R](R) reader structure"]
impl crate::Readable for MTLRX_QDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlrx_qdr::W](W) writer structure"]
impl crate::Writable for MTLRX_QDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRX_QDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
