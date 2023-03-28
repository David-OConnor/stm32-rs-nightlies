#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVR_MIS` reader - Data buffer overrun/underrun masked interrupt status"]
pub type OVR_MIS_R = crate::BitReader<OVR_MIS_A>;
#[doc = "Data buffer overrun/underrun masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MIS_A {
    #[doc = "0: No interrupt is generated when an overrun/underrun error occurs"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER"]
    Enabled = 1,
}
impl From<OVR_MIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_MIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_MIS_A {
        match self.bits {
            false => OVR_MIS_A::Disabled,
            true => OVR_MIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_MIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_MIS_A::Enabled
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun masked interrupt status"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
