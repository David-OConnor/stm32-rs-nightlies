#[doc = "Register `HWCFGR7` reader"]
pub struct R(crate::R<HWCFGR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub type CPUEVENT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "EXTI Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr7](index.html) module"]
pub struct HWCFGR7_SPEC;
impl crate::RegisterSpec for HWCFGR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr7::R](R) reader structure"]
impl crate::Readable for HWCFGR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR7 to value 0"]
impl crate::Resettable for HWCFGR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
