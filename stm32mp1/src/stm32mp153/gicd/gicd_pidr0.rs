#[doc = "Register `GICD_PIDR0` reader"]
pub struct R(crate::R<GICD_PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR0` reader - PIDR0"]
pub type PIDR0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR0"]
    #[inline(always)]
    pub fn pidr0(&self) -> PIDR0_R {
        PIDR0_R::new(self.bits)
    }
}
#[doc = "GICD peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr0](index.html) module"]
pub struct GICD_PIDR0_SPEC;
impl crate::RegisterSpec for GICD_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr0::R](R) reader structure"]
impl crate::Readable for GICD_PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GICD_PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x90;
}
