#[doc = "Register `SECWM2R_CUR` reader"]
pub struct R(crate::R<SECWM2R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM2R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM2R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM2R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECWM_STRT2` reader - Bank2 security WM area start sector"]
pub type SECWM_STRT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM_END2` reader - Bank2 security WM end sector"]
pub type SECWM_END2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Bank2 security WM area start sector"]
    #[inline(always)]
    pub fn secwm_strt2(&self) -> SECWM_STRT2_R {
        SECWM_STRT2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank2 security WM end sector"]
    #[inline(always)]
    pub fn secwm_end2(&self) -> SECWM_END2_R {
        SECWM_END2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH security watermark for Bank 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r_cur](index.html) module"]
pub struct SECWM2R_CUR_SPEC;
impl crate::RegisterSpec for SECWM2R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm2r_cur::R](R) reader structure"]
impl crate::Readable for SECWM2R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECWM2R_CUR to value 0"]
impl crate::Resettable for SECWM2R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
