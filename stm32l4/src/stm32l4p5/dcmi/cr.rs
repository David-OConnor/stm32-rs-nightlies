#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE` reader - Capture enable"]
pub type CAPTURE_R = crate::BitReader<CAPTURE_A>;
#[doc = "Capture enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTURE_A {
    #[doc = "0: Capture disabled"]
    Disabled = 0,
    #[doc = "1: Capture enabled"]
    Enabled = 1,
}
impl From<CAPTURE_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A {
        match self.bits {
            false => CAPTURE_A::Disabled,
            true => CAPTURE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPTURE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPTURE_A::Enabled
    }
}
#[doc = "Field `CAPTURE` writer - Capture enable"]
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CAPTURE_A, O>;
impl<'a, const O: u8> CAPTURE_W<'a, O> {
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPTURE_A::Disabled)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPTURE_A::Enabled)
    }
}
#[doc = "Field `CM` reader - Capture mode"]
pub type CM_R = crate::BitReader<CM_A>;
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CM_A {
    #[doc = "0: Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA"]
    Continuous = 0,
    #[doc = "1: Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset"]
    Snapshot = 1,
}
impl From<CM_A> for bool {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as u8 != 0
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            false => CM_A::Continuous,
            true => CM_A::Snapshot,
        }
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CM_A::Continuous
    }
    #[doc = "Checks if the value of the field is `Snapshot`"]
    #[inline(always)]
    pub fn is_snapshot(&self) -> bool {
        *self == CM_A::Snapshot
    }
}
#[doc = "Field `CM` writer - Capture mode"]
pub type CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CM_A, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "Continuous grab mode - The received data are transferred into the destination memory through the DMA. The buffer location and mode (linear or circular buffer) is controlled through the system DMA"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CM_A::Continuous)
    }
    #[doc = "Snapshot mode (single frame) - Once activated, the interface waits for the start of frame and then transfers a single frame through the DMA. At the end of the frame, the CAPTURE bit is automatically reset"]
    #[inline(always)]
    pub fn snapshot(self) -> &'a mut W {
        self.variant(CM_A::Snapshot)
    }
}
#[doc = "Field `CROP` reader - Crop feature"]
pub type CROP_R = crate::BitReader<CROP_A>;
#[doc = "Crop feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CROP_A {
    #[doc = "0: The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four"]
    Full = 0,
    #[doc = "1: Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured"]
    Cropped = 1,
}
impl From<CROP_A> for bool {
    #[inline(always)]
    fn from(variant: CROP_A) -> Self {
        variant as u8 != 0
    }
}
impl CROP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROP_A {
        match self.bits {
            false => CROP_A::Full,
            true => CROP_A::Cropped,
        }
    }
    #[doc = "Checks if the value of the field is `Full`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CROP_A::Full
    }
    #[doc = "Checks if the value of the field is `Cropped`"]
    #[inline(always)]
    pub fn is_cropped(&self) -> bool {
        *self == CROP_A::Cropped
    }
}
#[doc = "Field `CROP` writer - Crop feature"]
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CROP_A, O>;
impl<'a, const O: u8> CROP_W<'a, O> {
    #[doc = "The full image is captured. In this case the total number of bytes in an image frame must be a multiple of four"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(CROP_A::Full)
    }
    #[doc = "Only the data inside the window specified by the crop register is captured. If the size of the crop window exceeds the picture size, then only the picture size is captured"]
    #[inline(always)]
    pub fn cropped(self) -> &'a mut W {
        self.variant(CROP_A::Cropped)
    }
}
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JPEG_R = crate::BitReader<JPEG_A>;
#[doc = "JPEG format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JPEG_A {
    #[doc = "0: Uncompressed video format"]
    Uncompressed = 0,
    #[doc = "1: This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode"]
    Jpeg = 1,
}
impl From<JPEG_A> for bool {
    #[inline(always)]
    fn from(variant: JPEG_A) -> Self {
        variant as u8 != 0
    }
}
impl JPEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JPEG_A {
        match self.bits {
            false => JPEG_A::Uncompressed,
            true => JPEG_A::Jpeg,
        }
    }
    #[doc = "Checks if the value of the field is `Uncompressed`"]
    #[inline(always)]
    pub fn is_uncompressed(&self) -> bool {
        *self == JPEG_A::Uncompressed
    }
    #[doc = "Checks if the value of the field is `Jpeg`"]
    #[inline(always)]
    pub fn is_jpeg(&self) -> bool {
        *self == JPEG_A::Jpeg
    }
}
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, JPEG_A, O>;
impl<'a, const O: u8> JPEG_W<'a, O> {
    #[doc = "Uncompressed video format"]
    #[inline(always)]
    pub fn uncompressed(self) -> &'a mut W {
        self.variant(JPEG_A::Uncompressed)
    }
    #[doc = "This bit is used for JPEG data transfers. The DCMI_HSYNC signal is used as data enable. The crop and embedded synchronization features (ESS bit) cannot be used in this mode"]
    #[inline(always)]
    pub fn jpeg(self) -> &'a mut W {
        self.variant(JPEG_A::Jpeg)
    }
}
#[doc = "Field `ESS` reader - Embedded synchronization select"]
pub type ESS_R = crate::BitReader<ESS_A>;
#[doc = "Embedded synchronization select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESS_A {
    #[doc = "0: Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals"]
    Hardware = 0,
    #[doc = "1: Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow"]
    Embedded = 1,
}
impl From<ESS_A> for bool {
    #[inline(always)]
    fn from(variant: ESS_A) -> Self {
        variant as u8 != 0
    }
}
impl ESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESS_A {
        match self.bits {
            false => ESS_A::Hardware,
            true => ESS_A::Embedded,
        }
    }
    #[doc = "Checks if the value of the field is `Hardware`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == ESS_A::Hardware
    }
    #[doc = "Checks if the value of the field is `Embedded`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == ESS_A::Embedded
    }
}
#[doc = "Field `ESS` writer - Embedded synchronization select"]
pub type ESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ESS_A, O>;
impl<'a, const O: u8> ESS_W<'a, O> {
    #[doc = "Hardware synchronization data capture (frame/line start/stop) is synchronized with the DCMI_HSYNC/DCMI_VSYNC signals"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(ESS_A::Hardware)
    }
    #[doc = "Embedded synchronization data capture is synchronized with synchronization codes embedded in the data flow"]
    #[inline(always)]
    pub fn embedded(self) -> &'a mut W {
        self.variant(ESS_A::Embedded)
    }
}
#[doc = "Field `PCKPOL` reader - Pixel clock polarity"]
pub type PCKPOL_R = crate::BitReader<PCKPOL_A>;
#[doc = "Pixel clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCKPOL_A {
    #[doc = "0: Falling edge active"]
    FallingEdge = 0,
    #[doc = "1: Rising edge active"]
    RisingEdge = 1,
}
impl From<PCKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PCKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PCKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKPOL_A {
        match self.bits {
            false => PCKPOL_A::FallingEdge,
            true => PCKPOL_A::RisingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCKPOL_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCKPOL_A::RisingEdge
    }
}
#[doc = "Field `PCKPOL` writer - Pixel clock polarity"]
pub type PCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PCKPOL_A, O>;
impl<'a, const O: u8> PCKPOL_W<'a, O> {
    #[doc = "Falling edge active"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PCKPOL_A::FallingEdge)
    }
    #[doc = "Rising edge active"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PCKPOL_A::RisingEdge)
    }
}
#[doc = "Field `HSPOL` reader - Horizontal synchronization polarity"]
pub type HSPOL_R = crate::BitReader<HSPOL_A>;
#[doc = "Horizontal synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSPOL_A {
    #[doc = "0: DCMI_HSYNC active low"]
    ActiveLow = 0,
    #[doc = "1: DCMI_HSYNC active high"]
    ActiveHigh = 1,
}
impl From<HSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HSPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl HSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPOL_A {
        match self.bits {
            false => HSPOL_A::ActiveLow,
            true => HSPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL_A::ActiveHigh
    }
}
#[doc = "Field `HSPOL` writer - Horizontal synchronization polarity"]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSPOL_A, O>;
impl<'a, const O: u8> HSPOL_W<'a, O> {
    #[doc = "DCMI_HSYNC active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(HSPOL_A::ActiveLow)
    }
    #[doc = "DCMI_HSYNC active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(HSPOL_A::ActiveHigh)
    }
}
#[doc = "Field `VSPOL` reader - Vertical synchronization polarity"]
pub type VSPOL_R = crate::BitReader<VSPOL_A>;
#[doc = "Vertical synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSPOL_A {
    #[doc = "0: DCMI_VSYNC active low"]
    ActiveLow = 0,
    #[doc = "1: DCMI_VSYNC active high"]
    ActiveHigh = 1,
}
impl From<VSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VSPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl VSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSPOL_A {
        match self.bits {
            false => VSPOL_A::ActiveLow,
            true => VSPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL_A::ActiveHigh
    }
}
#[doc = "Field `VSPOL` writer - Vertical synchronization polarity"]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, VSPOL_A, O>;
impl<'a, const O: u8> VSPOL_W<'a, O> {
    #[doc = "DCMI_VSYNC active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(VSPOL_A::ActiveLow)
    }
    #[doc = "DCMI_VSYNC active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(VSPOL_A::ActiveHigh)
    }
}
#[doc = "Field `FCRC` reader - Frame capture rate control"]
pub type FCRC_R = crate::FieldReader<u8, FCRC_A>;
#[doc = "Frame capture rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCRC_A {
    #[doc = "0: All frames are captured"]
    All = 0,
    #[doc = "1: Every alternate frame captured (50% bandwidth reduction)"]
    Alternate = 1,
    #[doc = "2: One frame out of four captured (75% bandwidth reduction)"]
    OneOfFour = 2,
}
impl From<FCRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FCRC_A) -> Self {
        variant as _
    }
}
impl FCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCRC_A> {
        match self.bits {
            0 => Some(FCRC_A::All),
            1 => Some(FCRC_A::Alternate),
            2 => Some(FCRC_A::OneOfFour),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == FCRC_A::All
    }
    #[doc = "Checks if the value of the field is `Alternate`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == FCRC_A::Alternate
    }
    #[doc = "Checks if the value of the field is `OneOfFour`"]
    #[inline(always)]
    pub fn is_one_of_four(&self) -> bool {
        *self == FCRC_A::OneOfFour
    }
}
#[doc = "Field `FCRC` writer - Frame capture rate control"]
pub type FCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, FCRC_A, 2, O>;
impl<'a, const O: u8> FCRC_W<'a, O> {
    #[doc = "All frames are captured"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(FCRC_A::All)
    }
    #[doc = "Every alternate frame captured (50% bandwidth reduction)"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(FCRC_A::Alternate)
    }
    #[doc = "One frame out of four captured (75% bandwidth reduction)"]
    #[inline(always)]
    pub fn one_of_four(self) -> &'a mut W {
        self.variant(FCRC_A::OneOfFour)
    }
}
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<u8, EDM_A>;
#[doc = "Extended data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDM_A {
    #[doc = "0: Interface captures 8-bit data on every pixel clock"]
    BitWidth8 = 0,
    #[doc = "1: Interface captures 10-bit data on every pixel clock"]
    BitWidth10 = 1,
    #[doc = "2: Interface captures 12-bit data on every pixel clock"]
    BitWidth12 = 2,
    #[doc = "3: Interface captures 14-bit data on every pixel clock"]
    BitWidth14 = 3,
}
impl From<EDM_A> for u8 {
    #[inline(always)]
    fn from(variant: EDM_A) -> Self {
        variant as _
    }
}
impl EDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDM_A {
        match self.bits {
            0 => EDM_A::BitWidth8,
            1 => EDM_A::BitWidth10,
            2 => EDM_A::BitWidth12,
            3 => EDM_A::BitWidth14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BitWidth8`"]
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == EDM_A::BitWidth8
    }
    #[doc = "Checks if the value of the field is `BitWidth10`"]
    #[inline(always)]
    pub fn is_bit_width10(&self) -> bool {
        *self == EDM_A::BitWidth10
    }
    #[doc = "Checks if the value of the field is `BitWidth12`"]
    #[inline(always)]
    pub fn is_bit_width12(&self) -> bool {
        *self == EDM_A::BitWidth12
    }
    #[doc = "Checks if the value of the field is `BitWidth14`"]
    #[inline(always)]
    pub fn is_bit_width14(&self) -> bool {
        *self == EDM_A::BitWidth14
    }
}
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, EDM_A, 2, O>;
impl<'a, const O: u8> EDM_W<'a, O> {
    #[doc = "Interface captures 8-bit data on every pixel clock"]
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth8)
    }
    #[doc = "Interface captures 10-bit data on every pixel clock"]
    #[inline(always)]
    pub fn bit_width10(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth10)
    }
    #[doc = "Interface captures 12-bit data on every pixel clock"]
    #[inline(always)]
    pub fn bit_width12(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth12)
    }
    #[doc = "Interface captures 14-bit data on every pixel clock"]
    #[inline(always)]
    pub fn bit_width14(self) -> &'a mut W {
        self.variant(EDM_A::BitWidth14)
    }
}
#[doc = "Field `ENABLE` reader - DCMI enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "DCMI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: DCMI disabled"]
    Disabled = 0,
    #[doc = "1: DCMI enabled"]
    Enabled = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::Disabled,
            true => ENABLE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::Enabled
    }
}
#[doc = "Field `ENABLE` writer - DCMI enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "DCMI disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Disabled)
    }
    #[doc = "DCMI enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::Enabled)
    }
}
#[doc = "Field `BSM` reader - Byte Select mode"]
pub type BSM_R = crate::FieldReader<u8, BSM_A>;
#[doc = "Byte Select mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSM_A {
    #[doc = "0: Interface captures all received data"]
    All = 0,
    #[doc = "1: Interface captures every other byte from the received data"]
    EveryOther = 1,
    #[doc = "2: Interface captures one byte out of four"]
    Fourth = 2,
    #[doc = "3: Interface captures two bytes out of four"]
    TwoOfFour = 3,
}
impl From<BSM_A> for u8 {
    #[inline(always)]
    fn from(variant: BSM_A) -> Self {
        variant as _
    }
}
impl BSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSM_A {
        match self.bits {
            0 => BSM_A::All,
            1 => BSM_A::EveryOther,
            2 => BSM_A::Fourth,
            3 => BSM_A::TwoOfFour,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == BSM_A::All
    }
    #[doc = "Checks if the value of the field is `EveryOther`"]
    #[inline(always)]
    pub fn is_every_other(&self) -> bool {
        *self == BSM_A::EveryOther
    }
    #[doc = "Checks if the value of the field is `Fourth`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == BSM_A::Fourth
    }
    #[doc = "Checks if the value of the field is `TwoOfFour`"]
    #[inline(always)]
    pub fn is_two_of_four(&self) -> bool {
        *self == BSM_A::TwoOfFour
    }
}
#[doc = "Field `BSM` writer - Byte Select mode"]
pub type BSM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, BSM_A, 2, O>;
impl<'a, const O: u8> BSM_W<'a, O> {
    #[doc = "Interface captures all received data"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(BSM_A::All)
    }
    #[doc = "Interface captures every other byte from the received data"]
    #[inline(always)]
    pub fn every_other(self) -> &'a mut W {
        self.variant(BSM_A::EveryOther)
    }
    #[doc = "Interface captures one byte out of four"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(BSM_A::Fourth)
    }
    #[doc = "Interface captures two bytes out of four"]
    #[inline(always)]
    pub fn two_of_four(self) -> &'a mut W {
        self.variant(BSM_A::TwoOfFour)
    }
}
#[doc = "Field `OEBS` reader - Odd/Even Byte Select (Byte Select Start)"]
pub type OEBS_R = crate::BitReader<OEBS_A>;
#[doc = "Odd/Even Byte Select (Byte Select Start)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OEBS_A {
    #[doc = "0: Interface captures first data (byte or double byte) from the frame/line start, second one being dropped"]
    Odd = 0,
    #[doc = "1: Interface captures second data (byte or double byte) from the frame/line start, first one being dropped"]
    Even = 1,
}
impl From<OEBS_A> for bool {
    #[inline(always)]
    fn from(variant: OEBS_A) -> Self {
        variant as u8 != 0
    }
}
impl OEBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEBS_A {
        match self.bits {
            false => OEBS_A::Odd,
            true => OEBS_A::Even,
        }
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == OEBS_A::Odd
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == OEBS_A::Even
    }
}
#[doc = "Field `OEBS` writer - Odd/Even Byte Select (Byte Select Start)"]
pub type OEBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OEBS_A, O>;
impl<'a, const O: u8> OEBS_W<'a, O> {
    #[doc = "Interface captures first data (byte or double byte) from the frame/line start, second one being dropped"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(OEBS_A::Odd)
    }
    #[doc = "Interface captures second data (byte or double byte) from the frame/line start, first one being dropped"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(OEBS_A::Even)
    }
}
#[doc = "Field `LSM` reader - Line Select mode"]
pub type LSM_R = crate::BitReader<LSM_A>;
#[doc = "Line Select mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSM_A {
    #[doc = "0: Interface captures all received lines"]
    All = 0,
    #[doc = "1: Interface captures one line out of two"]
    Half = 1,
}
impl From<LSM_A> for bool {
    #[inline(always)]
    fn from(variant: LSM_A) -> Self {
        variant as u8 != 0
    }
}
impl LSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSM_A {
        match self.bits {
            false => LSM_A::All,
            true => LSM_A::Half,
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == LSM_A::All
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == LSM_A::Half
    }
}
#[doc = "Field `LSM` writer - Line Select mode"]
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LSM_A, O>;
impl<'a, const O: u8> LSM_W<'a, O> {
    #[doc = "Interface captures all received lines"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(LSM_A::All)
    }
    #[doc = "Interface captures one line out of two"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(LSM_A::Half)
    }
}
#[doc = "Field `OELS` reader - Odd/Even Line Select (Line Select Start)"]
pub type OELS_R = crate::BitReader<OELS_A>;
#[doc = "Odd/Even Line Select (Line Select Start)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OELS_A {
    #[doc = "0: Interface captures first line after the frame start, second one being dropped"]
    Odd = 0,
    #[doc = "1: Interface captures second line from the frame start, first one being dropped"]
    Even = 1,
}
impl From<OELS_A> for bool {
    #[inline(always)]
    fn from(variant: OELS_A) -> Self {
        variant as u8 != 0
    }
}
impl OELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OELS_A {
        match self.bits {
            false => OELS_A::Odd,
            true => OELS_A::Even,
        }
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == OELS_A::Odd
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == OELS_A::Even
    }
}
#[doc = "Field `OELS` writer - Odd/Even Line Select (Line Select Start)"]
pub type OELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OELS_A, O>;
impl<'a, const O: u8> OELS_W<'a, O> {
    #[doc = "Interface captures first line after the frame start, second one being dropped"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(OELS_A::Odd)
    }
    #[doc = "Interface captures second line from the frame start, first one being dropped"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(OELS_A::Even)
    }
}
impl R {
    #[doc = "Bit 0 - Capture enable"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronization select"]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame capture rate control"]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DCMI enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Byte Select mode"]
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start)"]
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start)"]
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture enable"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<1> {
        CM_W::new(self)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<2> {
        CROP_W::new(self)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<3> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 4 - Embedded synchronization select"]
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<4> {
        ESS_W::new(self)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<5> {
        PCKPOL_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<6> {
        HSPOL_W::new(self)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame capture rate control"]
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<8> {
        FCRC_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bit 14 - DCMI enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Byte Select mode"]
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<16> {
        BSM_W::new(self)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start)"]
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<18> {
        OEBS_W::new(self)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<19> {
        LSM_W::new(self)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start)"]
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<20> {
        OELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
