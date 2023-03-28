#[doc = "Register `GICD_PIDR1` reader"]
pub struct R(crate::R<GICD_PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR1` reader - PIDR1"]
pub type PIDR1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR1"]
    #[inline(always)]
    pub fn pidr1(&self) -> PIDR1_R {
        PIDR1_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr1](index.html) module"]
pub struct GICD_PIDR1_SPEC;
impl crate::RegisterSpec for GICD_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr1::R](R) reader structure"]
impl crate::Readable for GICD_PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR1 to value 0xb4"]
impl crate::Resettable for GICD_PIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
