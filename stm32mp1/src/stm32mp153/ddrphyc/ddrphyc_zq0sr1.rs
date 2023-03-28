#[doc = "Register `DDRPHYC_ZQ0SR1` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZPD` reader - ZPD"]
pub type ZPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZPU` reader - ZPU"]
pub type ZPU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPD` reader - OPD"]
pub type OPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPU` reader - OPU"]
pub type OPU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - ZPD"]
    #[inline(always)]
    pub fn zpd(&self) -> ZPD_R {
        ZPD_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - ZPU"]
    #[inline(always)]
    pub fn zpu(&self) -> ZPU_R {
        ZPU_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - OPD"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - OPU"]
    #[inline(always)]
    pub fn opu(&self) -> OPU_R {
        OPU_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "DDRPHYC ZQ0S register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr1](index.html) module"]
pub struct DDRPHYC_ZQ0SR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrphyc_zq0sr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR1 to value 0"]
impl crate::Resettable for DDRPHYC_ZQ0SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
