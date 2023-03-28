#[doc = "Register `HWCFGR5` reader"]
pub struct R(crate::R<HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR5_SPEC>) -> Self {
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
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr5](index.html) module"]
pub struct HWCFGR5_SPEC;
impl crate::RegisterSpec for HWCFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr5::R](R) reader structure"]
impl crate::Readable for HWCFGR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR5 to value 0x003e_ffff"]
impl crate::Resettable for HWCFGR5_SPEC {
    const RESET_VALUE: Self::Ux = 0x003e_ffff;
}
