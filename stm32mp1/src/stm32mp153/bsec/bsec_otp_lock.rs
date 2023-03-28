#[doc = "Register `BSEC_OTP_LOCK` reader"]
pub struct R(crate::R<BSEC_OTP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_LOCK` writer"]
pub struct W(crate::W<BSEC_OTP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_LOCK_SPEC>;
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
impl From<crate::W<BSEC_OTP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP` reader - OTP"]
pub type OTP_R = crate::BitReader<bool>;
#[doc = "Field `OTP` writer - OTP"]
pub type OTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `ROMLOCK` reader - ROMLOCK"]
pub type ROMLOCK_R = crate::BitReader<bool>;
#[doc = "Field `ROMLOCK` writer - ROMLOCK"]
pub type ROMLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `DENREG` reader - DENREG"]
pub type DENREG_R = crate::BitReader<bool>;
#[doc = "Field `DENREG` writer - DENREG"]
pub type DENREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_LOCK_SPEC, bool, O>;
#[doc = "Field `GPLOCK` reader - GPLOCK"]
pub type GPLOCK_R = crate::BitReader<bool>;
#[doc = "Field `GPLOCK` writer - GPLOCK"]
pub type GPLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&self) -> ROMLOCK_R {
        ROMLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&self) -> DENREG_R {
        DENREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&self) -> GPLOCK_R {
        GPLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    #[must_use]
    pub fn otp(&mut self) -> OTP_W<0> {
        OTP_W::new(self)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn romlock(&mut self) -> ROMLOCK_W<1> {
        ROMLOCK_W::new(self)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    #[must_use]
    pub fn denreg(&mut self) -> DENREG_W<2> {
        DENREG_W::new(self)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn gplock(&mut self) -> GPLOCK_W<4> {
        GPLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP lock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_lock](index.html) module"]
pub struct BSEC_OTP_LOCK_SPEC;
impl crate::RegisterSpec for BSEC_OTP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_lock::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_lock::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_LOCK to value 0"]
impl crate::Resettable for BSEC_OTP_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
