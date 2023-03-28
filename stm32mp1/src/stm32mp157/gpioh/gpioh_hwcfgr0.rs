#[doc = "Register `GPIOH_HWCFGR0` reader"]
pub struct R(crate::R<GPIOH_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOH_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOH_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOH_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OR_RES` reader - OR_RES"]
pub type OR_RES_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - OR_RES"]
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr0](index.html) module"]
pub struct GPIOH_HWCFGR0_SPEC;
impl crate::RegisterSpec for GPIOH_HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioh_hwcfgr0::R](R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOH_HWCFGR0 to value 0"]
impl crate::Resettable for GPIOH_HWCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}