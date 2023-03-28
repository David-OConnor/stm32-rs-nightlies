#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVR_RIS` reader - Data buffer overrun/underrun raw interrupt status"]
pub type OVR_RIS_R = crate::BitReader<OVR_RIS_A>;
#[doc = "Data buffer overrun/underrun raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS_A {
    #[doc = "0: No overrun/underrun occurred"]
    Cleared = 0,
    #[doc = "1: An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR"]
    Occurred = 1,
}
impl From<OVR_RIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_RIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_RIS_A {
        match self.bits {
            false => OVR_RIS_A::Cleared,
            true => OVR_RIS_A::Occurred,
        }
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == OVR_RIS_A::Cleared
    }
    #[doc = "Checks if the value of the field is `Occurred`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == OVR_RIS_A::Occurred
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
