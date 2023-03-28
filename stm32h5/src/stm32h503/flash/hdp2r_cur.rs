#[doc = "Register `HDP2R_CUR` reader"]
pub struct R(crate::R<HDP2R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP2R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP2R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP2R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HDP2_STRT` reader - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
pub type HDP2_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDP2_END` reader - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
pub type HDP2_END_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp2_strt(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn hdp2_end(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "FLASH HDP Bank2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp2r_cur](index.html) module"]
pub struct HDP2R_CUR_SPEC;
impl crate::RegisterSpec for HDP2R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp2r_cur::R](R) reader structure"]
impl crate::Readable for HDP2R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HDP2R_CUR to value 0"]
impl crate::Resettable for HDP2R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
