#[doc = "Register `DINR15` reader"]
pub struct R(crate::R<DINR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN15` reader - Input data received from MDIO Master during write frames"]
pub type DIN15_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din15(&self) -> DIN15_R {
        DIN15_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr15](index.html) module"]
pub struct DINR15_SPEC;
impl crate::RegisterSpec for DINR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr15::R](R) reader structure"]
impl crate::Readable for DINR15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR15 to value 0"]
impl crate::Resettable for DINR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}