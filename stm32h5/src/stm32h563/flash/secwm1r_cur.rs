#[doc = "Register `SECWM1R_CUR` reader"]
pub struct R(crate::R<SECWM1R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector"]
pub type SECWM1_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM1_END` reader - Bank1 security WM area 1 end sector"]
pub type SECWM1_END_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Bank1 security WM area 1 start sector"]
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank1 security WM area 1 end sector"]
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH security watermark for Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r_cur](index.html) module"]
pub struct SECWM1R_CUR_SPEC;
impl crate::RegisterSpec for SECWM1R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm1r_cur::R](R) reader structure"]
impl crate::Readable for SECWM1R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECWM1R_CUR to value 0"]
impl crate::Resettable for SECWM1R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
