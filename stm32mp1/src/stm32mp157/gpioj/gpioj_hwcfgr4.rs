#[doc = "Register `GPIOJ_HWCFGR4` reader"]
pub struct R(crate::R<GPIOJ_HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOJ_HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOJ_HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOJ_HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSPEED_RES` reader - OSPEED_RES"]
pub type OSPEED_RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - OSPEED_RES"]
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioj_hwcfgr4](index.html) module"]
pub struct GPIOJ_HWCFGR4_SPEC;
impl crate::RegisterSpec for GPIOJ_HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioj_hwcfgr4::R](R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOJ_HWCFGR4 to value 0"]
impl crate::Resettable for GPIOJ_HWCFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
