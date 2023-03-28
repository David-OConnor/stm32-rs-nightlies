#[doc = "Register `SYSCFG_ITLINE3` reader"]
pub struct R(crate::R<SYSCFG_ITLINE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH_ITF` reader - Flash interface interrupt request pending"]
pub type FLASH_ITF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Flash interface interrupt request pending"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline3](index.html) module"]
pub struct SYSCFG_ITLINE3_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline3::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE3 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
