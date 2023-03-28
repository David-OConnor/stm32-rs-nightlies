#[doc = "Register `ITLINE13` reader"]
pub struct R(crate::R<ITLINE13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIM1_CCU` reader - TIM1_CCU"]
pub type TIM1_CCU_R = crate::BitReader<bool>;
#[doc = "Field `TIM1_TRG` reader - TIM1_TRG"]
pub type TIM1_TRG_R = crate::BitReader<bool>;
#[doc = "Field `TIM1_UPD` reader - TIM1_UPD"]
pub type TIM1_UPD_R = crate::BitReader<bool>;
#[doc = "Field `TIM1_BRK` reader - TIM1_BRK"]
pub type TIM1_BRK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TIM1_CCU"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM1_TRG"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM1_UPD"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM1_BRK"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "interrupt line 13 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline13](index.html) module"]
pub struct ITLINE13_SPEC;
impl crate::RegisterSpec for ITLINE13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline13::R](R) reader structure"]
impl crate::Readable for ITLINE13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE13 to value 0"]
impl crate::Resettable for ITLINE13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
