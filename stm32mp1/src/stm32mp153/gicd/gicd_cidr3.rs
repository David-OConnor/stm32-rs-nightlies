#[doc = "Register `GICD_CIDR3` reader"]
pub struct R(crate::R<GICD_CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIDR3` reader - CIDR3"]
pub type CIDR3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDR3"]
    #[inline(always)]
    pub fn cidr3(&self) -> CIDR3_R {
        CIDR3_R::new(self.bits)
    }
}
#[doc = "GICD component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr3](index.html) module"]
pub struct GICD_CIDR3_SPEC;
impl crate::RegisterSpec for GICD_CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr3::R](R) reader structure"]
impl crate::Readable for GICD_CIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR3 to value 0xb1"]
impl crate::Resettable for GICD_CIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
