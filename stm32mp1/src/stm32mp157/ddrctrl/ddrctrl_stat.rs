#[doc = "Register `DDRCTRL_STAT` reader"]
pub struct R(crate::R<DDRCTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPERATING_MODE` reader - OPERATING_MODE"]
pub type OPERATING_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELFREF_TYPE` reader - SELFREF_TYPE"]
pub type SELFREF_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELFREF_CAM_NOT_EMPTY` reader - SELFREF_CAM_NOT_EMPTY"]
pub type SELFREF_CAM_NOT_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - OPERATING_MODE"]
    #[inline(always)]
    pub fn operating_mode(&self) -> OPERATING_MODE_R {
        OPERATING_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - SELFREF_TYPE"]
    #[inline(always)]
    pub fn selfref_type(&self) -> SELFREF_TYPE_R {
        SELFREF_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - SELFREF_CAM_NOT_EMPTY"]
    #[inline(always)]
    pub fn selfref_cam_not_empty(&self) -> SELFREF_CAM_NOT_EMPTY_R {
        SELFREF_CAM_NOT_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DDRCTRL operating mode status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_stat](index.html) module"]
pub struct DDRCTRL_STAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_stat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_STAT to value 0"]
impl crate::Resettable for DDRCTRL_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
