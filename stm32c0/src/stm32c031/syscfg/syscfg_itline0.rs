#[doc = "Register `SYSCFG_ITLINE0` reader"]
pub struct R(crate::R<SYSCFG_ITLINE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WWDG` reader - Window watchdog interrupt pending flag"]
pub type WWDG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Window watchdog interrupt pending flag"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline0](index.html) module"]
pub struct SYSCFG_ITLINE0_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline0::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE0 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
