#[doc = "Register `OTPBLR_PRG` reader"]
pub struct R(crate::R<OTPBLR_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPBLR_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPBLR_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPBLR_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPBLR_PRG` writer"]
pub struct W(crate::W<OTPBLR_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPBLR_PRG_SPEC>;
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
impl From<crate::W<OTPBLR_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPBLR_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKBL` reader - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
pub type LOCKBL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCKBL` writer - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
pub type LOCKBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTPBLR_PRG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn lockbl(&mut self) -> LOCKBL_W<0> {
        LOCKBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH non-secure OTP block lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpblr_prg](index.html) module"]
pub struct OTPBLR_PRG_SPEC;
impl crate::RegisterSpec for OTPBLR_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpblr_prg::R](R) reader structure"]
impl crate::Readable for OTPBLR_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpblr_prg::W](W) writer structure"]
impl crate::Writable for OTPBLR_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTPBLR_PRG to value 0"]
impl crate::Resettable for OTPBLR_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
