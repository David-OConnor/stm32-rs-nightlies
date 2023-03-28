#[doc = "Register `SYSCFG_ITLINE14` reader"]
pub struct R(crate::R<SYSCFG_ITLINE14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM1_CC` reader - Timer 1 capture compare interrupt request pending"]
pub type TIM1_CC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Timer 1 capture compare interrupt request pending"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 14 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline14](index.html) module"]
pub struct SYSCFG_ITLINE14_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline14::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE14 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
