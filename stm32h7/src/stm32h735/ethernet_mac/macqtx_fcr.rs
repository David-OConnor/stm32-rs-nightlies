#[doc = "Register `MACQTxFCR` reader"]
pub struct R(crate::R<MACQTX_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACQTX_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACQTX_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACQTX_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACQTxFCR` writer"]
pub struct W(crate::W<MACQTX_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACQTX_FCR_SPEC>;
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
impl From<crate::W<MACQTX_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACQTX_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FCB_BPA_R = crate::BitReader<bool>;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FCB_BPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACQTX_FCR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACQTX_FCR_SPEC, bool, O>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACQTX_FCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACQTX_FCR_SPEC, bool, O>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACQTX_FCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Queue flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macqtx_fcr](index.html) module"]
pub struct MACQTX_FCR_SPEC;
impl crate::RegisterSpec for MACQTX_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macqtx_fcr::R](R) reader structure"]
impl crate::Readable for MACQTX_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macqtx_fcr::W](W) writer structure"]
impl crate::Writable for MACQTX_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACQTxFCR to value 0"]
impl crate::Resettable for MACQTX_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
