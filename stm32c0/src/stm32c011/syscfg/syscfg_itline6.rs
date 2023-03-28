#[doc = "Register `SYSCFG_ITLINE6` reader"]
pub struct R(crate::R<SYSCFG_ITLINE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTI2` reader - EXTI line 2 interrupt request pending"]
pub type EXTI2_R = crate::BitReader<bool>;
#[doc = "Field `EXTI3` reader - EXTI line 3 interrupt request pending"]
pub type EXTI3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EXTI line 2 interrupt request pending"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 3 interrupt request pending"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 6 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline6](index.html) module"]
pub struct SYSCFG_ITLINE6_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline6::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE6 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
