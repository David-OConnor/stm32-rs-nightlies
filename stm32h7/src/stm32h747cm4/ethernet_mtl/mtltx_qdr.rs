#[doc = "Register `MTLTxQDR` reader"]
pub struct R(crate::R<MTLTX_QDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTX_QDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTX_QDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTX_QDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLTxQDR` writer"]
pub struct W(crate::W<MTLTX_QDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLTX_QDR_SPEC>;
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
impl From<crate::W<MTLTX_QDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLTX_QDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause"]
pub type TXQPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `TXQPAUSED` writer - Transmit Queue in Pause"]
pub type TXQPAUSED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QDR_SPEC, bool, O>;
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status"]
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRCSTS` writer - MTL Tx Queue Read Controller Status"]
pub type TRCSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status"]
pub type TWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `TWCSTS` writer - MTL Tx Queue Write Controller Status"]
pub type TWCSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QDR_SPEC, bool, O>;
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status"]
pub type TXQSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXQSTS` writer - MTL Tx Queue Not Empty Status"]
pub type TXQSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QDR_SPEC, bool, O>;
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status"]
pub type TXSTSFSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSFSTS` writer - MTL Tx Status FIFO Full Status"]
pub type TXSTSFSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QDR_SPEC, bool, O>;
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue"]
pub type PTXQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTXQ` writer - Number of Packets in the Transmit Queue"]
pub type PTXQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QDR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue"]
pub type STXSTSF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STXSTSF` writer - Number of Status Words in Tx Status FIFO of Queue"]
pub type STXSTSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QDR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    #[must_use]
    pub fn txqpaused(&mut self) -> TXQPAUSED_W<0> {
        TXQPAUSED_W::new(self)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    #[must_use]
    pub fn trcsts(&mut self) -> TRCSTS_W<1> {
        TRCSTS_W::new(self)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    #[must_use]
    pub fn twcsts(&mut self) -> TWCSTS_W<3> {
        TWCSTS_W::new(self)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    #[must_use]
    pub fn txqsts(&mut self) -> TXQSTS_W<4> {
        TXQSTS_W::new(self)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    #[must_use]
    pub fn txstsfsts(&mut self) -> TXSTSFSTS_W<5> {
        TXSTSFSTS_W::new(self)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    #[must_use]
    pub fn ptxq(&mut self) -> PTXQ_W<16> {
        PTXQ_W::new(self)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    #[must_use]
    pub fn stxstsf(&mut self) -> STXSTSF_W<20> {
        STXSTSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qdr](index.html) module"]
pub struct MTLTX_QDR_SPEC;
impl crate::RegisterSpec for MTLTX_QDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qdr::R](R) reader structure"]
impl crate::Readable for MTLTX_QDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtltx_qdr::W](W) writer structure"]
impl crate::Writable for MTLTX_QDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLTxQDR to value 0"]
impl crate::Resettable for MTLTX_QDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
