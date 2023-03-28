#[doc = "Register `OTPBLR_CUR` reader"]
pub struct R(crate::R<OTPBLR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPBLR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPBLR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPBLR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKBL` reader - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
pub type LOCKBL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new(self.bits)
    }
}
#[doc = "FLASH non-secure OTP block lock\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpblr_cur](index.html) module"]
pub struct OTPBLR_CUR_SPEC;
impl crate::RegisterSpec for OTPBLR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpblr_cur::R](R) reader structure"]
impl crate::Readable for OTPBLR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTPBLR_CUR to value 0"]
impl crate::Resettable for OTPBLR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
