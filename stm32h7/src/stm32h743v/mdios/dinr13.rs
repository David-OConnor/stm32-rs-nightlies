#[doc = "Register `DINR13` reader"]
pub struct R(crate::R<DINR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN13` reader - Input data received from MDIO Master during write frames"]
pub type DIN13_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr13](index.html) module"]
pub struct DINR13_SPEC;
impl crate::RegisterSpec for DINR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr13::R](R) reader structure"]
impl crate::Readable for DINR13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR13 to value 0"]
impl crate::Resettable for DINR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
