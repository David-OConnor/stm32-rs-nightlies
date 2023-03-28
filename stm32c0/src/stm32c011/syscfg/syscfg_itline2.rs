#[doc = "Register `SYSCFG_ITLINE2` reader"]
pub struct R(crate::R<SYSCFG_ITLINE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC` reader - RTC interrupt request pending (EXTI line 19)"]
pub type RTC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - RTC interrupt request pending (EXTI line 19)"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline2](index.html) module"]
pub struct SYSCFG_ITLINE2_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline2::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE2 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
