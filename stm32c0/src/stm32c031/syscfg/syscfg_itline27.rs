#[doc = "Register `SYSCFG_ITLINE27` reader"]
pub struct R(crate::R<SYSCFG_ITLINE27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USART1` reader - USART1 interrupt request pending, combined with EXTI line 25"]
pub type USART1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USART1 interrupt request pending, combined with EXTI line 25"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 27 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline27](index.html) module"]
pub struct SYSCFG_ITLINE27_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline27::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE27_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE27 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
