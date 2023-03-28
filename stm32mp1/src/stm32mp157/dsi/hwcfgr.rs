#[doc = "Register `HWCFGR` reader"]
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TECHNO` reader - TECHNO"]
pub type TECHNO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOSIZE` reader - FIFOSIZE"]
pub type FIFOSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - TECHNO"]
    #[inline(always)]
    pub fn techno(&self) -> TECHNO_R {
        TECHNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[doc = "DSI Host hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR to value 0x5a01"]
impl crate::Resettable for HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x5a01;
}
