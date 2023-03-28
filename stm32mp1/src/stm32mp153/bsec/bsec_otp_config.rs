#[doc = "Register `BSEC_OTP_CONFIG` reader"]
pub struct R(crate::R<BSEC_OTP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_CONFIG` writer"]
pub struct W(crate::W<BSEC_OTP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_CONFIG_SPEC>;
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
impl From<crate::W<BSEC_OTP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRUP` reader - PWRUP"]
pub type PWRUP_R = crate::BitReader<bool>;
#[doc = "Field `PWRUP` writer - PWRUP"]
pub type PWRUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSEC_OTP_CONFIG_SPEC, bool, O>;
#[doc = "Field `FRC` reader - FRC"]
pub type FRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRC` writer - FRC"]
pub type FRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSEC_OTP_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRGWIDTH` reader - PRGWIDTH"]
pub type PRGWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRGWIDTH` writer - PRGWIDTH"]
pub type PRGWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TREAD` reader - TREAD"]
pub type TREAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREAD` writer - TREAD"]
pub type TREAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSEC_OTP_CONFIG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    pub fn frc(&self) -> FRC_R {
        FRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    pub fn prgwidth(&self) -> PRGWIDTH_R {
        PRGWIDTH_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    pub fn tread(&self) -> TREAD_R {
        TREAD_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup(&mut self) -> PWRUP_W<0> {
        PWRUP_W::new(self)
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    #[must_use]
    pub fn frc(&mut self) -> FRC_W<1> {
        FRC_W::new(self)
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn prgwidth(&mut self) -> PRGWIDTH_W<3> {
        PRGWIDTH_W::new(self)
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    #[must_use]
    pub fn tread(&mut self) -> TREAD_W<7> {
        TREAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_config](index.html) module"]
pub struct BSEC_OTP_CONFIG_SPEC;
impl crate::RegisterSpec for BSEC_OTP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_config::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_config::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_CONFIG to value 0x0e"]
impl crate::Resettable for BSEC_OTP_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
