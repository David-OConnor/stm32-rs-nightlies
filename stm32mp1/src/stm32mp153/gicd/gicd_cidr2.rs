#[doc = "Register `GICD_CIDR2` reader"]
pub struct R(crate::R<GICD_CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIDR2` reader - CIDR2"]
pub type CIDR2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR2"]
    #[inline(always)]
    pub fn cidr2(&self) -> CIDR2_R {
        CIDR2_R::new(self.bits)
    }
}
#[doc = "GICD component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr2](index.html) module"]
pub struct GICD_CIDR2_SPEC;
impl crate::RegisterSpec for GICD_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr2::R](R) reader structure"]
impl crate::Readable for GICD_CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR2 to value 0x05"]
impl crate::Resettable for GICD_CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
