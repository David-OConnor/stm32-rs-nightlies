#[doc = "Register `RGSR` reader"]
pub struct R(crate::R<RGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OF0` reader - OF0"]
pub type OF0_R = crate::BitReader<OF0_A>;
#[doc = "OF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF0_A {
    #[doc = "0: No new trigger event occured on DMA request generator channel x, before the request counter underrun"]
    NoTrigger = 0,
    #[doc = "1: New trigger event occured on DMA request generator channel x, before the request counter underrun"]
    Trigger = 1,
}
impl From<OF0_A> for bool {
    #[inline(always)]
    fn from(variant: OF0_A) -> Self {
        variant as u8 != 0
    }
}
impl OF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF0_A {
        match self.bits {
            false => OF0_A::NoTrigger,
            true => OF0_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoTrigger`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == OF0_A::NoTrigger
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OF0_A::Trigger
    }
}
#[doc = "Field `OF1` reader - OF1"]
pub use OF0_R as OF1_R;
#[doc = "Field `OF2` reader - OF2"]
pub use OF0_R as OF2_R;
#[doc = "Field `OF3` reader - Trigger overrun event flag"]
pub use OF0_R as OF3_R;
impl R {
    #[doc = "Bit 0 - OF0"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OF1"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OF2"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun event flag"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "request generator interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgsr](index.html) module"]
pub struct RGSR_SPEC;
impl crate::RegisterSpec for RGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgsr::R](R) reader structure"]
impl crate::Readable for RGSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
