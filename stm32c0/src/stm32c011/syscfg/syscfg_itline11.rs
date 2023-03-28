#[doc = "Register `SYSCFG_ITLINE11` reader"]
pub struct R(crate::R<SYSCFG_ITLINE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMAMUX` reader - DMAMUX interrupt request pending"]
pub type DMAMUX_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMAMUX interrupt request pending"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 11 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline11](index.html) module"]
pub struct SYSCFG_ITLINE11_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline11::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE11 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
