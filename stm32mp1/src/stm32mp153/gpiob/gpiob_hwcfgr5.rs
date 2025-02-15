#[doc = "Register `GPIOB_HWCFGR5` reader"]
pub struct R(crate::R<GPIOB_HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PUPDR_RES` reader - PUPDR_RES"]
pub type PUPDR_RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PUPDR_RES"]
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr5](index.html) module"]
pub struct GPIOB_HWCFGR5_SPEC;
impl crate::RegisterSpec for GPIOB_HWCFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_hwcfgr5::R](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOB_HWCFGR5 to value 0"]
impl crate::Resettable for GPIOB_HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
