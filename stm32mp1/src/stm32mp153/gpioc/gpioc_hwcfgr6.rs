#[doc = "Register `GPIOC_HWCFGR6` reader"]
pub struct R(crate::R<GPIOC_HWCFGR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOC_HWCFGR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOC_HWCFGR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOC_HWCFGR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODER_RES` reader - MODER_RES"]
pub type MODER_RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - MODER_RES"]
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_hwcfgr6](index.html) module"]
pub struct GPIOC_HWCFGR6_SPEC;
impl crate::RegisterSpec for GPIOC_HWCFGR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioc_hwcfgr6::R](R) reader structure"]
impl crate::Readable for GPIOC_HWCFGR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOC_HWCFGR6 to value 0xffff_ffff"]
impl crate::Resettable for GPIOC_HWCFGR6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
