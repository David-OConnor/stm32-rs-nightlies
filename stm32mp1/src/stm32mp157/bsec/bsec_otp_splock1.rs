#[doc = "Register `BSEC_OTP_SPLOCK1` reader"]
pub struct R(crate::R<BSEC_OTP_SPLOCK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_SPLOCK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_SPLOCK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_SPLOCK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_SPLOCK1` writer"]
pub struct W(crate::W<BSEC_OTP_SPLOCK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_SPLOCK1_SPEC>;
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
impl From<crate::W<BSEC_OTP_SPLOCK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_SPLOCK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLOCK` reader - SPLOCK"]
pub type SPLOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPLOCK` writer - SPLOCK"]
pub type SPLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_SPLOCK1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&self) -> SPLOCK_R {
        SPLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn splock(&mut self) -> SPLOCK_W<0> {
        SPLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_splock1](index.html) module"]
pub struct BSEC_OTP_SPLOCK1_SPEC;
impl crate::RegisterSpec for BSEC_OTP_SPLOCK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_splock1::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_splock1::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_SPLOCK1 to value 0"]
impl crate::Resettable for BSEC_OTP_SPLOCK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}