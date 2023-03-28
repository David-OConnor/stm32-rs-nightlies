#[doc = "Register `BSEC_OTP_CONTROL` reader"]
pub struct R(crate::R<BSEC_OTP_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_CONTROL` writer"]
pub struct W(crate::W<BSEC_OTP_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_CONTROL_SPEC>;
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
impl From<crate::W<BSEC_OTP_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSEC_OTP_CONTROL_SPEC, u8, u8, 7, O>;
#[doc = "Field `PROG` reader - PROG"]
pub type PROG_R = crate::BitReader<bool>;
#[doc = "Field `PROG` writer - PROG"]
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_CONTROL_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - PROG"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 8 - PROG"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<8> {
        PROG_W::new(self)
    }
    #[doc = "Bit 9 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<9> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_control](index.html) module"]
pub struct BSEC_OTP_CONTROL_SPEC;
impl crate::RegisterSpec for BSEC_OTP_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_control::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_control::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_CONTROL to value 0"]
impl crate::Resettable for BSEC_OTP_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
