#[doc = "Register `COMP_ID_1` reader"]
pub struct R(crate::R<COMP_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - Preamble bits 8 to 11"]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLASS` reader - Component class"]
pub type CLASS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Preamble bits 8 to 11"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_1](index.html) module"]
pub struct COMP_ID_1_SPEC;
impl crate::RegisterSpec for COMP_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_id_1::R](R) reader structure"]
impl crate::Readable for COMP_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP_ID_1 to value 0x04"]
impl crate::Resettable for COMP_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
