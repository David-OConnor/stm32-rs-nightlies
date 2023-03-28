#[doc = "Register `AXIMC_COMP_ID_1` reader"]
pub struct R(crate::R<AXIMC_COMP_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_COMP_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_COMP_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_COMP_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - PREAMBLE"]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLASS` reader - CLASS"]
pub type CLASS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PREAMBLE"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLASS"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXIMC component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_comp_id_1](index.html) module"]
pub struct AXIMC_COMP_ID_1_SPEC;
impl crate::RegisterSpec for AXIMC_COMP_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aximc_comp_id_1::R](R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AXIMC_COMP_ID_1 to value 0xf0"]
impl crate::Resettable for AXIMC_COMP_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
