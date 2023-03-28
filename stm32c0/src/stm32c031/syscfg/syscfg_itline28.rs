#[doc = "Register `SYSCFG_ITLINE28` reader"]
pub struct R(crate::R<SYSCFG_ITLINE28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USART2` reader - USART2 interrupt request pending (EXTI line 26)"]
pub type USART2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USART2 interrupt request pending (EXTI line 26)"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 28 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline28](index.html) module"]
pub struct SYSCFG_ITLINE28_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline28::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE28_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE28 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
