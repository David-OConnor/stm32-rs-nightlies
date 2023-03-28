#[doc = "Register `MTLTxQOMR` reader"]
pub struct R(crate::R<MTLTX_QOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTX_QOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTX_QOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTX_QOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLTxQOMR` writer"]
pub struct W(crate::W<MTLTX_QOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLTX_QOMR_SPEC>;
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
impl From<crate::W<MTLTX_QOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLTX_QOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTQ` reader - Flush Transmit Queue"]
pub type FTQ_R = crate::BitReader<bool>;
#[doc = "Field `FTQ` writer - Flush Transmit Queue"]
pub type FTQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QOMR_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QOMR_SPEC, bool, O>;
#[doc = "Field `TXQEN` reader - Transmit Queue Enable"]
pub type TXQEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` reader - Transmit Threshold Control"]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - Transmit Threshold Control"]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QOMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TQS` reader - Transmit Queue Size"]
pub type TQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TQS` writer - Transmit Queue Size"]
pub type TQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QOMR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable"]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size"]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    #[must_use]
    pub fn ftq(&mut self) -> FTQ_W<0> {
        FTQ_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<1> {
        TSF_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<4> {
        TTC_W::new(self)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TQS_W<16> {
        TQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qomr](index.html) module"]
pub struct MTLTX_QOMR_SPEC;
impl crate::RegisterSpec for MTLTX_QOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qomr::R](R) reader structure"]
impl crate::Readable for MTLTX_QOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtltx_qomr::W](W) writer structure"]
impl crate::Writable for MTLTX_QOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLTxQOMR to value 0x0007_0008"]
impl crate::Resettable for MTLTX_QOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0008;
}
