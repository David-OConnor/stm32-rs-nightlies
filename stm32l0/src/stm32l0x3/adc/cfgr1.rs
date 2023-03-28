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
#[doc = "Field `DMAEN` reader - Direct memory access enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "Direct memory access enable\n\nValue on reset: 0"]
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
#[doc = "Field `DMAEN` writer - Direct memory access enable"]
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
#[doc = "Field `DMACFG` reader - Direct memery access configuration"]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
#[doc = "Direct memery access configuration\n\nValue on reset: 0"]
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
#[doc = "Field `DMACFG` writer - Direct memery access configuration"]
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
#[doc = "Field `SCANDIR` reader - Scan sequence direction"]
pub type SCANDIR_R = crate::BitReader<SCANDIR_A>;
#[doc = "Scan sequence direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR_A {
    #[doc = "0: Upward scan (from CHSEL0 to CHSEL18)"]
    Upward = 0,
    #[doc = "1: Backward scan (from CHSEL18 to CHSEL0)"]
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
#[doc = "Field `SCANDIR` writer - Scan sequence direction"]
pub type SCANDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SCANDIR_A, O>;
impl<'a, const O: u8> SCANDIR_W<'a, O> {
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn upward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Upward)
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn backward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Backward)
    }
}
#[doc = "Field `RES` reader - Data resolution"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
#[doc = "Data resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12 bits"]
    TwelveBit = 0,
    #[doc = "1: 10 bits"]
    TenBit = 1,
    #[doc = "2: 8 bits"]
    EightBit = 2,
    #[doc = "3: 6 bits"]
    SixBit = 3,
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
            0 => RES_A::TwelveBit,
            1 => RES_A::TenBit,
            2 => RES_A::EightBit,
            3 => RES_A::SixBit,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TwelveBit`"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES_A::TwelveBit
    }
    #[doc = "Checks if the value of the field is `TenBit`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES_A::TenBit
    }
    #[doc = "Checks if the value of the field is `EightBit`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES_A::EightBit
    }
    #[doc = "Checks if the value of the field is `SixBit`"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == RES_A::SixBit
    }
}
#[doc = "Field `RES` writer - Data resolution"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RES_A::TwelveBit)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RES_A::TenBit)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RES_A::EightBit)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(RES_A::SixBit)
    }
}
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Data alignment\n\nValue on reset: 0"]
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
#[doc = "Field `ALIGN` writer - Data alignment"]
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
#[doc = "Field `EXTSEL` reader - External trigger selection"]
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
#[doc = "External trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 6 TRGO event"]
    Tim6Trgo = 0,
    #[doc = "1: Timer 21 CH2 event"]
    Tim21Ch2 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    Tim2Trgo = 2,
    #[doc = "3: Timer 2 CH4 event"]
    Tim2Ch4 = 3,
    #[doc = "4: Timer 22 TRGO, Timer 21 TRGO event"]
    Tim22Trgo = 4,
    #[doc = "5: Timer 2 CH3 event"]
    Tim2Ch3 = 5,
    #[doc = "6: Timer 3 TRGO event"]
    Tim3Trgo = 6,
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
    pub fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::Tim6Trgo,
            1 => EXTSEL_A::Tim21Ch2,
            2 => EXTSEL_A::Tim2Trgo,
            3 => EXTSEL_A::Tim2Ch4,
            4 => EXTSEL_A::Tim22Trgo,
            5 => EXTSEL_A::Tim2Ch3,
            6 => EXTSEL_A::Tim3Trgo,
            7 => EXTSEL_A::ExtiLine11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Tim6Trgo`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim6Trgo
    }
    #[doc = "Checks if the value of the field is `Tim21Ch2`"]
    #[inline(always)]
    pub fn is_tim21_ch2(&self) -> bool {
        *self == EXTSEL_A::Tim21Ch2
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
    #[doc = "Checks if the value of the field is `Tim22Trgo`"]
    #[inline(always)]
    pub fn is_tim22_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim22Trgo
    }
    #[doc = "Checks if the value of the field is `Tim2Ch3`"]
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == EXTSEL_A::Tim2Ch3
    }
    #[doc = "Checks if the value of the field is `Tim3Trgo`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim3Trgo
    }
    #[doc = "Checks if the value of the field is `ExtiLine11`"]
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        *self == EXTSEL_A::ExtiLine11
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection"]
pub type EXTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, EXTSEL_A, 3, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim6Trgo)
    }
    #[doc = "Timer 21 CH2 event"]
    #[inline(always)]
    pub fn tim21_ch2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim21Ch2)
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
    #[doc = "Timer 22 TRGO, Timer 21 TRGO event"]
    #[inline(always)]
    pub fn tim22_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim22Trgo)
    }
    #[doc = "Timer 2 CH3 event"]
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Ch3)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3Trgo)
    }
    #[doc = "EXTI line 11 event"]
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut W {
        self.variant(EXTSEL_A::ExtiLine11)
    }
}
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection"]
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
#[doc = "External trigger enable and polarity selection\n\nValue on reset: 0"]
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
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection"]
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
#[doc = "Field `OVRMOD` reader - Overrun management mode"]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
#[doc = "Overrun management mode\n\nValue on reset: 0"]
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
#[doc = "Field `OVRMOD` writer - Overrun management mode"]
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
#[doc = "Field `CONT` reader - Single / continuous conversion mode"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Single / continuous conversion mode\n\nValue on reset: 0"]
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
#[doc = "Field `CONT` writer - Single / continuous conversion mode"]
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
#[doc = "Field `WAIT` reader - Auto-delayed conversion mode"]
pub type WAIT_R = crate::BitReader<WAIT_A>;
#[doc = "Auto-delayed conversion mode\n\nValue on reset: 0"]
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
#[doc = "Field `WAIT` writer - Auto-delayed conversion mode"]
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
#[doc = "Field `AUTOFF` reader - Auto-off mode"]
pub type AUTOFF_R = crate::BitReader<AUTOFF_A>;
#[doc = "Auto-off mode\n\nValue on reset: 0"]
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
#[doc = "Field `AUTOFF` writer - Auto-off mode"]
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
#[doc = "Field `DISCEN` reader - Discontinuous mode"]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
#[doc = "Discontinuous mode\n\nValue on reset: 0"]
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
#[doc = "Field `DISCEN` writer - Discontinuous mode"]
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
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel or on all channels"]
pub type AWDSGL_R = crate::BitReader<AWDSGL_A>;
#[doc = "Enable the watchdog on a single channel or on all channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDSGL_A {
    #[doc = "0: Analog watchdog enabled on all channels"]
    AllChannels = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    SingleChannel = 1,
}
impl From<AWDSGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDSGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDSGL_A {
        match self.bits {
            false => AWDSGL_A::AllChannels,
            true => AWDSGL_A::SingleChannel,
        }
    }
    #[doc = "Checks if the value of the field is `AllChannels`"]
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGL_A::AllChannels
    }
    #[doc = "Checks if the value of the field is `SingleChannel`"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGL_A::SingleChannel
    }
}
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel or on all channels"]
pub type AWDSGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWDSGL_A, O>;
impl<'a, const O: u8> AWDSGL_W<'a, O> {
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWDSGL_A::AllChannels)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWDSGL_A::SingleChannel)
    }
}
#[doc = "Field `AWDEN` reader - Analog watchdog enable"]
pub type AWDEN_R = crate::BitReader<AWDEN_A>;
#[doc = "Analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDEN_A {
    #[doc = "0: Analog watchdog disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled"]
    Enabled = 1,
}
impl From<AWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AWDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDEN_A {
        match self.bits {
            false => AWDEN_A::Disabled,
            true => AWDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN_A::Enabled
    }
}
#[doc = "Field `AWDEN` writer - Analog watchdog enable"]
pub type AWDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWDEN_A, O>;
impl<'a, const O: u8> AWDEN_W<'a, O> {
    #[doc = "Analog watchdog disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDEN_A::Disabled)
    }
    #[doc = "Analog watchdog enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDEN_A::Enabled)
    }
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AWDCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Direct memory access enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memery access configuration"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto-delayed conversion mode"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode"]
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Direct memery access configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<2> {
        SCANDIR_W::new(self)
    }
    #[doc = "Bits 3:4 - Data resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bit 5 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 6:8 - External trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    #[doc = "Bit 12 - Overrun management mode"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 14 - Auto-delayed conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<14> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 15 - Auto-off mode"]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<15> {
        AUTOFF_W::new(self)
    }
    #[doc = "Bit 16 - Discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels"]
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<22> {
        AWDSGL_W::new(self)
    }
    #[doc = "Bit 23 - Analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<23> {
        AWDEN_W::new(self)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<26> {
        AWDCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
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
