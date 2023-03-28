#[doc = "Register `HDP1R_CUR` reader"]
pub struct R(crate::R<HDP1R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP1R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP1R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP1R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HDP1_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors"]
pub type HDP1_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDP1_END` reader - HDPL barrier end set in number of 8-Kbyte sectors"]
pub type HDP1_END_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "FLASH HDP Bank 1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp1r_cur](index.html) module"]
pub struct HDP1R_CUR_SPEC;
impl crate::RegisterSpec for HDP1R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp1r_cur::R](R) reader structure"]
impl crate::Readable for HDP1R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HDP1R_CUR to value 0"]
impl crate::Resettable for HDP1R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
