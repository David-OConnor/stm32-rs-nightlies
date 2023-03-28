#[doc = "Register `SYSCFG_ITLINE5` reader"]
pub struct R(crate::R<SYSCFG_ITLINE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTI0` reader - EXTI line 0 interrupt request pending"]
pub type EXTI0_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1` reader - EXTI line 1 interrupt request pending"]
pub type EXTI1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EXTI line 0 interrupt request pending"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 1 interrupt request pending"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline5](index.html) module"]
pub struct SYSCFG_ITLINE5_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline5::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE5 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
