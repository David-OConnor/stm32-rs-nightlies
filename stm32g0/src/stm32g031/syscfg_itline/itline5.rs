#[doc = "Register `ITLINE5` reader"]
pub struct R(crate::R<ITLINE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTI0` reader - EXTI0"]
pub type EXTI0_R = crate::BitReader<bool>;
#[doc = "Field `EXTI1` reader - EXTI1"]
pub type EXTI1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline5](index.html) module"]
pub struct ITLINE5_SPEC;
impl crate::RegisterSpec for ITLINE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline5::R](R) reader structure"]
impl crate::Readable for ITLINE5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE5 to value 0"]
impl crate::Resettable for ITLINE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
