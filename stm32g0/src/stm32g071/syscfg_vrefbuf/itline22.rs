#[doc = "Register `ITLINE22` reader"]
pub struct R(crate::R<ITLINE22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM17` reader - TIM17"]
pub type TIM17_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TIM17"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 22 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline22](index.html) module"]
pub struct ITLINE22_SPEC;
impl crate::RegisterSpec for ITLINE22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline22::R](R) reader structure"]
impl crate::Readable for ITLINE22_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE22 to value 0"]
impl crate::Resettable for ITLINE22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
