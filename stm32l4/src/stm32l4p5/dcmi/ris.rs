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
#[doc = "Field `FRAME_RIS` reader - Capture complete raw interrupt status"]
pub type FRAME_RIS_R = crate::BitReader<FRAME_RIS_A>;
#[doc = "Capture complete raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_RIS_A {
    #[doc = "0: No new capture"]
    NoNewCapture = 0,
    #[doc = "1: A frame has been captured"]
    FrameCaptured = 1,
}
impl From<FRAME_RIS_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_RIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_RIS_A {
        match self.bits {
            false => FRAME_RIS_A::NoNewCapture,
            true => FRAME_RIS_A::FrameCaptured,
        }
    }
    #[doc = "Checks if the value of the field is `NoNewCapture`"]
    #[inline(always)]
    pub fn is_no_new_capture(&self) -> bool {
        *self == FRAME_RIS_A::NoNewCapture
    }
    #[doc = "Checks if the value of the field is `FrameCaptured`"]
    #[inline(always)]
    pub fn is_frame_captured(&self) -> bool {
        *self == FRAME_RIS_A::FrameCaptured
    }
}
#[doc = "Field `OVR_RIS` reader - Overrun raw interrupt status"]
pub type OVR_RIS_R = crate::BitReader<OVR_RIS_A>;
#[doc = "Overrun raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS_A {
    #[doc = "0: No data buffer overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: A data buffer overrun occurred and the data FIFO is corrupted. The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register"]
    OverrunOccured = 1,
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
            false => OVR_RIS_A::NoOverrun,
            true => OVR_RIS_A::OverrunOccured,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_RIS_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `OverrunOccured`"]
    #[inline(always)]
    pub fn is_overrun_occured(&self) -> bool {
        *self == OVR_RIS_A::OverrunOccured
    }
}
#[doc = "Field `ERR_RIS` reader - Synchronization error raw interrupt status"]
pub type ERR_RIS_R = crate::BitReader<ERR_RIS_A>;
#[doc = "Synchronization error raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_RIS_A {
    #[doc = "0: No synchronization error detected"]
    NoError = 0,
    #[doc = "1: Embedded synchronization characters are not received in the correct order"]
    SynchronizationError = 1,
}
impl From<ERR_RIS_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_RIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_RIS_A {
        match self.bits {
            false => ERR_RIS_A::NoError,
            true => ERR_RIS_A::SynchronizationError,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_RIS_A::NoError
    }
    #[doc = "Checks if the value of the field is `SynchronizationError`"]
    #[inline(always)]
    pub fn is_synchronization_error(&self) -> bool {
        *self == ERR_RIS_A::SynchronizationError
    }
}
#[doc = "Field `VSYNC_RIS` reader - VSYNC raw interrupt status"]
pub type VSYNC_RIS_R = crate::BitReader<VSYNC_RIS_A>;
#[doc = "VSYNC raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_RIS_A {
    #[doc = "0: Interrupt cleared"]
    Cleared = 0,
    #[doc = "1: Interrupt set"]
    Set = 1,
}
impl From<VSYNC_RIS_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_RIS_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_RIS_A {
        match self.bits {
            false => VSYNC_RIS_A::Cleared,
            true => VSYNC_RIS_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == VSYNC_RIS_A::Cleared
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == VSYNC_RIS_A::Set
    }
}
#[doc = "Field `LINE_RIS` reader - Line raw interrupt status"]
pub type LINE_RIS_R = crate::BitReader<LINE_RIS_A>;
#[doc = "Line raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_RIS_A {
    #[doc = "0: Interrupt cleared"]
    Cleared = 0,
    #[doc = "1: Interrupt set"]
    Set = 1,
}
impl From<LINE_RIS_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_RIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LINE_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_RIS_A {
        match self.bits {
            false => LINE_RIS_A::Cleared,
            true => LINE_RIS_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == LINE_RIS_A::Cleared
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LINE_RIS_A::Set
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete raw interrupt status"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error raw interrupt status"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC raw interrupt status"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line raw interrupt status"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
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
