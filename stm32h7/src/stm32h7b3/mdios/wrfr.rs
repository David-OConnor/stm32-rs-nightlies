#[doc = "Register `WRFR` reader"]
pub struct R(crate::R<WRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRF` reader - Write flags for MDIO registers 0 to 31"]
pub type WRF_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new(self.bits)
    }
}
#[doc = "MDIOS write flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrfr](index.html) module"]
pub struct WRFR_SPEC;
impl crate::RegisterSpec for WRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrfr::R](R) reader structure"]
impl crate::Readable for WRFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRFR to value 0"]
impl crate::Resettable for WRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
