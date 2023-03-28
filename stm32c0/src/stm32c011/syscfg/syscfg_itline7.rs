#[doc = "Register `SYSCFG_ITLINE7` reader"]
pub struct R(crate::R<SYSCFG_ITLINE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTI4` reader - EXTI line 4 interrupt request pending"]
pub type EXTI4_R = crate::BitReader<bool>;
#[doc = "Field `EXTI5` reader - EXTI line 5 interrupt request pending"]
pub type EXTI5_R = crate::BitReader<bool>;
#[doc = "Field `EXTI6` reader - EXTI line 6 interrupt request pending"]
pub type EXTI6_R = crate::BitReader<bool>;
#[doc = "Field `EXTI7` reader - EXTI line 7 interrupt request pending"]
pub type EXTI7_R = crate::BitReader<bool>;
#[doc = "Field `EXTI8` reader - EXTI line 8 interrupt request pending"]
pub type EXTI8_R = crate::BitReader<bool>;
#[doc = "Field `EXTI9` reader - EXTI line 9 interrupt request pending"]
pub type EXTI9_R = crate::BitReader<bool>;
#[doc = "Field `EXTI10` reader - EXTI line 10 interrupt request pending"]
pub type EXTI10_R = crate::BitReader<bool>;
#[doc = "Field `EXTI11` reader - EXTI line 11 interrupt request pending"]
pub type EXTI11_R = crate::BitReader<bool>;
#[doc = "Field `EXTI12` reader - EXTI line 12 interrupt request pending"]
pub type EXTI12_R = crate::BitReader<bool>;
#[doc = "Field `EXTI13` reader - EXTI line 13 interrupt request pending"]
pub type EXTI13_R = crate::BitReader<bool>;
#[doc = "Field `EXTI14` reader - EXTI line 14 interrupt request pending"]
pub type EXTI14_R = crate::BitReader<bool>;
#[doc = "Field `EXTI15` reader - EXTI line 15 interrupt request pending"]
pub type EXTI15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EXTI line 4 interrupt request pending"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI line 5 interrupt request pending"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTI line 6 interrupt request pending"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTI line 7 interrupt request pending"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTI line 8 interrupt request pending"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTI line 9 interrupt request pending"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTI line 10 interrupt request pending"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTI line 11 interrupt request pending"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EXTI line 12 interrupt request pending"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EXTI line 13 interrupt request pending"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EXTI line 14 interrupt request pending"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EXTI line 15 interrupt request pending"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 7 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline7](index.html) module"]
pub struct SYSCFG_ITLINE7_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline7::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE7 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
