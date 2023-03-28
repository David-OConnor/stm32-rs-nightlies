#[doc = "Register `BSEC_OTP_SWLOCK0` reader"]
pub struct R(crate::R<BSEC_OTP_SWLOCK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_SWLOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_SWLOCK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_SWLOCK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_SWLOCK0` writer"]
pub struct W(crate::W<BSEC_OTP_SWLOCK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_SWLOCK0_SPEC>;
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
impl From<crate::W<BSEC_OTP_SWLOCK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_SWLOCK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWLOCK` reader - SWLOCK"]
pub type SWLOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SWLOCK` writer - SWLOCK"]
pub type SWLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_SWLOCK0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    pub fn swlock(&self) -> SWLOCK_R {
        SWLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn swlock(&mut self) -> SWLOCK_W<0> {
        SWLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_swlock0](index.html) module"]
pub struct BSEC_OTP_SWLOCK0_SPEC;
impl crate::RegisterSpec for BSEC_OTP_SWLOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_swlock0::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_SWLOCK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_swlock0::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_SWLOCK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_SWLOCK0 to value 0x01"]
impl crate::Resettable for BSEC_OTP_SWLOCK0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
