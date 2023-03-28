#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    Disabled = 0,
    #[doc = "1: DMA enabled"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAENÂ =Â 1. For more details, refer to pageÂ 403 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
#[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAENÂ =Â 1. For more details, refer to pageÂ 403 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    #[doc = "0: DMA one shot mode selected"]
    OneShot = 0,
    #[doc = "1: DMA circular mode selected"]
    Circular = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::OneShot,
            true => DMACFG_A::Circular,
        }
    }
    #[doc = "Checks if the value of the field is `OneShot`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG_A::OneShot
    }
    #[doc = "Checks if the value of the field is `Circular`"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG_A::Circular
    }
}
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAENÂ =Â 1. For more details, refer to pageÂ 403 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DMACFG_A, O>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::OneShot)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::Circular)
    }
}
#[doc = "Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_R = crate::BitReader<SCANDIR_A>;
#[doc = "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR_A {
    #[doc = "0: Upward scan (from CHSEL0 to CHSEL17)"]
    Upward = 0,
    #[doc = "1: Backward scan (from CHSEL17 to CHSEL0)"]
    Backward = 1,
}
impl From<SCANDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SCANDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANDIR_A {
        match self.bits {
            false => SCANDIR_A::Upward,
            true => SCANDIR_A::Backward,
        }
    }
    #[doc = "Checks if the value of the field is `Upward`"]
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == SCANDIR_A::Upward
    }
    #[doc = "Checks if the value of the field is `Backward`"]
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        *self == SCANDIR_A::Backward
    }
}
#[doc = "Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SCANDIR_A, O>;
impl<'a, const O: u8> SCANDIR_W<'a, O> {
    #[doc = "Upward scan (from CHSEL0 to CHSEL17)"]
    #[inline(always)]
    pub fn upward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Upward)
    }
    #[doc = "Backward scan (from CHSEL17 to CHSEL0)"]
    #[inline(always)]
    pub fn backward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Backward)
    }
}
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADENÂ =Â 0."]
pub type RES_R = crate::FieldReader<u8, RES_A>;
#[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADENÂ =Â 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12 bits"]
    Bits12 = 0,
    #[doc = "1: 10 bits"]
    Bits10 = 1,
    #[doc = "2: 8 bits"]
    Bits8 = 2,
    #[doc = "3: 6 bits"]
    Bits6 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::Bits12,
            1 => RES_A::Bits10,
            2 => RES_A::Bits8,
            3 => RES_A::Bits6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES_A::Bits12
    }
    #[doc = "Checks if the value of the field is `Bits10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES_A::Bits10
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits6`"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES_A::Bits6
    }
}
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADENÂ =Â 0."]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(RES_A::Bits12)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(RES_A::Bits10)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(RES_A::Bits8)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(RES_A::Bits6)
    }
}
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on pageÂ 401 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on pageÂ 401 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::Right,
            true => ALIGN_A::Left,
        }
    }
    #[doc = "Checks if the value of the field is `Right`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::Right
    }
    #[doc = "Checks if the value of the field is `Left`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::Left
    }
}
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on pageÂ 401 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::Left)
    }
}
#[doc = "Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
#[doc = "External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 1 TRGO event"]
    Tim1Trgo = 0,
    #[doc = "1: Timer 1 CC4 event"]
    Tim1Cc4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    Tim2Trgo = 2,
    #[doc = "3: Timer 2 CH4 event"]
    Tim2Ch4 = 3,
    #[doc = "5: Timer 2 CH3 event"]
    Tim2Ch3 = 5,
    #[doc = "7: EXTI line 11 event"]
    ExtiLine11 = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::Tim1Trgo),
            1 => Some(EXTSEL_A::Tim1Cc4),
            2 => Some(EXTSEL_A::Tim2Trgo),
            3 => Some(EXTSEL_A::Tim2Ch4),
            5 => Some(EXTSEL_A::Tim2Ch3),
            7 => Some(EXTSEL_A::ExtiLine11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tim1Trgo`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo
    }
    #[doc = "Checks if the value of the field is `Tim1Cc4`"]
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc4
    }
    #[doc = "Checks if the value of the field is `Tim2Trgo`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim2Trgo
    }
    #[doc = "Checks if the value of the field is `Tim2Ch4`"]
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == EXTSEL_A::Tim2Ch4
    }
    #[doc = "Checks if the value of the field is `Tim2Ch3`"]
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == EXTSEL_A::Tim2Ch3
    }
    #[doc = "Checks if the value of the field is `ExtiLine11`"]
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        *self == EXTSEL_A::ExtiLine11
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, EXTSEL_A, 3, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Trgo)
    }
    #[doc = "Timer 2 CH4 event"]
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Ch4)
    }
    #[doc = "Timer 2 CH3 event"]
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Ch3)
    }
    #[doc = "EXTI line 11 event"]
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut W {
        self.variant(EXTSEL_A::ExtiLine11)
    }
}
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
#[doc = "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Hardware trigger detection disabled"]
    Disabled = 0,
    #[doc = "1: Hardware trigger detection on the rising edge"]
    RisingEdge = 1,
    #[doc = "2: Hardware trigger detection on the falling edge"]
    FallingEdge = 2,
    #[doc = "3: Hardware trigger detection on both the rising and falling edges"]
    BothEdges = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::Disabled,
            1 => EXTEN_A::RisingEdge,
            2 => EXTEN_A::FallingEdge,
            3 => EXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BothEdges
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, EXTEN_A, 2, O>;
impl<'a, const O: u8> EXTEN_W<'a, O> {
    #[doc = "Hardware trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::Disabled)
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RisingEdge)
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FallingEdge)
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BothEdges)
    }
}
#[doc = "Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
#[doc = "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    #[doc = "0: ADC_DR register is preserved with the old data when an overrun is detected"]
    Preserve = 0,
    #[doc = "1: ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
    Overwrite = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::Preserve,
            true => OVRMOD_A::Overwrite,
        }
    }
    #[doc = "Checks if the value of the field is `Preserve`"]
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD_A::Preserve
    }
    #[doc = "Checks if the value of the field is `Overwrite`"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD_A::Overwrite
    }
}
#[doc = "Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, OVRMOD_A, O>;
impl<'a, const O: u8> OVRMOD_W<'a, O> {
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected"]
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::Preserve)
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::Overwrite)
    }
}
#[doc = "Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::Single,
            true => CONT_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::Single
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
#[doc = "Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
#[doc = "Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_R = crate::BitReader<WAIT_A>;
#[doc = "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    #[doc = "0: Wait conversion mode off"]
    Disabled = 0,
    #[doc = "1: Wait conversion mode on"]
    Enabled = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::Disabled,
            true => WAIT_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAIT_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAIT_A::Enabled
    }
}
#[doc = "Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, WAIT_A, O>;
impl<'a, const O: u8> WAIT_W<'a, O> {
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAIT_A::Disabled)
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAIT_A::Enabled)
    }
}
#[doc = "Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_R = crate::BitReader<AUTOFF_A>;
#[doc = "Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF_A {
    #[doc = "0: Auto-off mode disabled"]
    Disabled = 0,
    #[doc = "1: Auto-off mode enabled"]
    Enabled = 1,
}
impl From<AUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOFF_A {
        match self.bits {
            false => AUTOFF_A::Disabled,
            true => AUTOFF_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOFF_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOFF_A::Enabled
    }
}
#[doc = "Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AUTOFF_A, O>;
impl<'a, const O: u8> AUTOFF_W<'a, O> {
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::Disabled)
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::Enabled)
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
#[doc = "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode enabled"]
    Enabled = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::Disabled,
            true => DISCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::Enabled
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DISCEN_A, O>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    #[doc = "Discontinuous mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Disabled)
    }
    #[doc = "Discontinuous mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Enabled)
    }
}
#[doc = "Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_R = crate::BitReader<CHSELRMOD_A>;
#[doc = "Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELRMOD_A {
    #[doc = "0: Each bit of the ADC_CHSELR register enables an input"]
    BitPerInput = 0,
    #[doc = "1: ADC_CHSELR register is able to sequence up to 8 channels"]
    Sequence = 1,
}
impl From<CHSELRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSELRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELRMOD_A {
        match self.bits {
            false => CHSELRMOD_A::BitPerInput,
            true => CHSELRMOD_A::Sequence,
        }
    }
    #[doc = "Checks if the value of the field is `BitPerInput`"]
    #[inline(always)]
    pub fn is_bit_per_input(&self) -> bool {
        *self == CHSELRMOD_A::BitPerInput
    }
    #[doc = "Checks if the value of the field is `Sequence`"]
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == CHSELRMOD_A::Sequence
    }
}
#[doc = "Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CHSELRMOD_A, O>;
impl<'a, const O: u8> CHSELRMOD_W<'a, O> {
    #[doc = "Each bit of the ADC_CHSELR register enables an input"]
    #[inline(always)]
    pub fn bit_per_input(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::BitPerInput)
    }
    #[doc = "ADC_CHSELR register is able to sequence up to 8 channels"]
    #[inline(always)]
    pub fn sequence(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::Sequence)
    }
}
#[doc = "Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
#[doc = "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    AllChannels = 0,
    #[doc = "1: Analog watchdog 1 enabled on a single channel"]
    SingleChannel = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::AllChannels,
            true => AWD1SGL_A::SingleChannel,
        }
    }
    #[doc = "Checks if the value of the field is `AllChannels`"]
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWD1SGL_A::AllChannels
    }
    #[doc = "Checks if the value of the field is `SingleChannel`"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWD1SGL_A::SingleChannel
    }
}
#[doc = "Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWD1SGL_A, O>;
impl<'a, const O: u8> AWD1SGL_W<'a, O> {
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWD1SGL_A::AllChannels)
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWD1SGL_A::SingleChannel)
    }
}
#[doc = "Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
#[doc = "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog 1 enabled"]
    Enabled = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::Disabled,
            true => AWD1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN_A::Enabled
    }
}
#[doc = "Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWD1EN_A, O>;
impl<'a, const O: u8> AWD1EN_W<'a, O> {
    #[doc = "Analog watchdog 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Disabled)
    }
    #[doc = "Analog watchdog 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Enabled)
    }
}
#[doc = "Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAENÂ =Â 1. For more details, refer to pageÂ 403 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADENÂ =Â 0."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on pageÂ 401 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAENÂ =Â 1. For more details, refer to pageÂ 403 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<2> {
        SCANDIR_W::new(self)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADENÂ =Â 0."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on pageÂ 401 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<14> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<15> {
        AUTOFF_W::new(self)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCENÂ =Â 1 and CONTÂ =Â 1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<21> {
        CHSELRMOD_W::new(self)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<26> {
        AWD1CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
