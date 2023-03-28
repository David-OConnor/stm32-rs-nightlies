#[doc = "Register `BSEC_OTP_WRDATA` reader"]
pub struct R(crate::R<BSEC_OTP_WRDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_WRDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_WRDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_WRDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_WRDATA` writer"]
pub struct W(crate::W<BSEC_OTP_WRDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_WRDATA_SPEC>;
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
impl From<crate::W<BSEC_OTP_WRDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_WRDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRDATA` reader - WRDATA"]
pub type WRDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRDATA` writer - WRDATA"]
pub type WRDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_WRDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&self) -> WRDATA_R {
        WRDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    #[must_use]
    pub fn wrdata(&mut self) -> WRDATA_W<0> {
        WRDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP write data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrdata](index.html) module"]
pub struct BSEC_OTP_WRDATA_SPEC;
impl crate::RegisterSpec for BSEC_OTP_WRDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_wrdata::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_WRDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_wrdata::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_WRDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_WRDATA to value 0"]
impl crate::Resettable for BSEC_OTP_WRDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
