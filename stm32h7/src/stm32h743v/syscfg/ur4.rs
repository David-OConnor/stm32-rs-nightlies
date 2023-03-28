#[doc = "Register `UR4` reader"]
pub struct R(crate::R<UR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEPAD_1` reader - Mass Erase Protected Area Disabled for bank 1"]
pub type MEPAD_1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 16 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur4](index.html) module"]
pub struct UR4_SPEC;
impl crate::RegisterSpec for UR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur4::R](R) reader structure"]
impl crate::Readable for UR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR4 to value 0"]
impl crate::Resettable for UR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
