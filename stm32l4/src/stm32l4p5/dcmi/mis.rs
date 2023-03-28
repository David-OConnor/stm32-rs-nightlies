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
#[doc = "Field `FRAME_MIS` reader - Capture complete masked interrupt status"]
pub type FRAME_MIS_R = crate::BitReader<FRAME_MIS_A>;
#[doc = "Capture complete masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_MIS_A {
    #[doc = "0: No interrupt is generated after a complete capture"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated at the end of each received frame/crop window (in crop mode) and the FRAME_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<FRAME_MIS_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_MIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_MIS_A {
        match self.bits {
            false => FRAME_MIS_A::Disabled,
            true => FRAME_MIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAME_MIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAME_MIS_A::Enabled
    }
}
#[doc = "Field `OVR_MIS` reader - Overrun masked interrupt status"]
pub type OVR_MIS_R = crate::BitReader<OVR_MIS_A>;
#[doc = "Overrun masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MIS_A {
    #[doc = "0: No interrupt is generated on overrun"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received and the OVR_IE bit is set in DCMI_IER"]
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
#[doc = "Field `ERR_MIS` reader - Synchronization error masked interrupt status"]
pub type ERR_MIS_R = crate::BitReader<ERR_MIS_A>;
#[doc = "Synchronization error masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_MIS_A {
    #[doc = "0: No interrupt is generated on a synchronization error"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the embedded synchronization codes are not received in the correct order and the ERR_IE bit in DCMI_IER is set"]
    Enabled = 1,
}
impl From<ERR_MIS_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_MIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_MIS_A {
        match self.bits {
            false => ERR_MIS_A::Disabled,
            true => ERR_MIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERR_MIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERR_MIS_A::Enabled
    }
}
#[doc = "Field `VSYNC_MIS` reader - VSYNC masked interrupt status"]
pub type VSYNC_MIS_R = crate::BitReader<VSYNC_MIS_A>;
#[doc = "VSYNC masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_MIS_A {
    #[doc = "0: No interrupt is generated on DCMI_VSYNC transitions"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state and the VSYNC_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<VSYNC_MIS_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_MIS_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_MIS_A {
        match self.bits {
            false => VSYNC_MIS_A::Disabled,
            true => VSYNC_MIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSYNC_MIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSYNC_MIS_A::Enabled
    }
}
#[doc = "Field `LINE_MIS` reader - Line masked interrupt status"]
pub type LINE_MIS_R = crate::BitReader<LINE_MIS_A>;
#[doc = "Line masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_MIS_A {
    #[doc = "0: No interrupt generation when the line is received"]
    Disabled = 0,
    #[doc = "1: An Interrupt is generated when a line has been completely received and the LINE_IE bit is set in DCMI_IER"]
    Enabled = 1,
}
impl From<LINE_MIS_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_MIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LINE_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_MIS_A {
        match self.bits {
            false => LINE_MIS_A::Disabled,
            true => LINE_MIS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINE_MIS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINE_MIS_A::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete masked interrupt status"]
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun masked interrupt status"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error masked interrupt status"]
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC masked interrupt status"]
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line masked interrupt status"]
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
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
