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
#[doc = "Field `ALARMB` reader - ALARMB"]
pub type ALARMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAKEUP` reader - WAKEUP"]
pub type WAKEUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMOOTH_CALIB` reader - SMOOTH_CALIB"]
pub type SMOOTH_CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMESTAMP` reader - TIMESTAMP"]
pub type TIMESTAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPTIONREG_OUT` reader - OPTIONREG_OUT"]
pub type OPTIONREG_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub type TRUST_ZONE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - ALARMB"]
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    pub fn smooth_calib(&self) -> SMOOTH_CALIB_R {
        SMOOTH_CALIB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "RTC hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR to value 0x0103_1111"]
impl crate::Resettable for HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0103_1111;
}
