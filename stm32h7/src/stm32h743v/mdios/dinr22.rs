#[doc = "Register `DINR22` reader"]
pub struct R(crate::R<DINR22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN22` reader - Input data received from MDIO Master during write frames"]
pub type DIN22_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din22(&self) -> DIN22_R {
        DIN22_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 22\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr22](index.html) module"]
pub struct DINR22_SPEC;
impl crate::RegisterSpec for DINR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr22::R](R) reader structure"]
impl crate::Readable for DINR22_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR22 to value 0"]
impl crate::Resettable for DINR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
