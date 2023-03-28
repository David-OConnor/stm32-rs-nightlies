#[doc = "Register `GPIOI_HWCFGR3` reader"]
pub struct R(crate::R<GPIOI_HWCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOI_HWCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOI_HWCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOI_HWCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODR_RES` reader - ODR_RES"]
pub type ODR_RES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OTYPER_RES` reader - OTYPER_RES"]
pub type OTYPER_RES_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ODR_RES"]
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OTYPER_RES"]
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr3](index.html) module"]
pub struct GPIOI_HWCFGR3_SPEC;
impl crate::RegisterSpec for GPIOI_HWCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioi_hwcfgr3::R](R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOI_HWCFGR3 to value 0"]
impl crate::Resettable for GPIOI_HWCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}