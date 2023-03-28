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
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<u8, MEM_MODE_A>;
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SystemFlash = 1,
    #[doc = "2: Main Flash memory mapped at 0x0000_0000"]
    MainFlash2 = 2,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
impl MEM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_MODE_A {
        match self.bits {
            0 => MEM_MODE_A::MainFlash,
            1 => MEM_MODE_A::SystemFlash,
            2 => MEM_MODE_A::MainFlash2,
            3 => MEM_MODE_A::Sram,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MainFlash`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MainFlash
    }
    #[doc = "Checks if the value of the field is `SystemFlash`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SystemFlash
    }
    #[doc = "Checks if the value of the field is `MainFlash2`"]
    #[inline(always)]
    pub fn is_main_flash2(&self) -> bool {
        *self == MEM_MODE_A::MainFlash2
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::Sram
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, MEM_MODE_A, 2, O>;
impl<'a, const O: u8> MEM_MODE_W<'a, O> {
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SystemFlash)
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash2(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash2)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Sram)
    }
}
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
pub type PA11_PA12_RMP_R = crate::BitReader<PA11_PA12_RMP_A>;
#[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA11_PA12_RMP_A {
    #[doc = "0: Pin pair PA9/PA10 mapped on the pins"]
    NotRemapped = 0,
    #[doc = "1: Pin pair PA11/PA12 mapped instead of PA9/PA10"]
    Remapped = 1,
}
impl From<PA11_PA12_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: PA11_PA12_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PA11_PA12_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PA11_PA12_RMP_A {
        match self.bits {
            false => PA11_PA12_RMP_A::NotRemapped,
            true => PA11_PA12_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == PA11_PA12_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == PA11_PA12_RMP_A::Remapped
    }
}
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
pub type PA11_PA12_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, PA11_PA12_RMP_A, O>;
impl<'a, const O: u8> PA11_PA12_RMP_W<'a, O> {
    #[doc = "Pin pair PA9/PA10 mapped on the pins"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(PA11_PA12_RMP_A::NotRemapped)
    }
    #[doc = "Pin pair PA11/PA12 mapped instead of PA9/PA10"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(PA11_PA12_RMP_A::Remapped)
    }
}
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection"]
pub type IR_MOD_R = crate::FieldReader<u8, IR_MOD_A>;
#[doc = "IR Modulation Envelope signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IR_MOD_A {
    #[doc = "0: TIM16 selected"]
    Tim16 = 0,
    #[doc = "1: USART1 selected"]
    Usart1 = 1,
    #[doc = "2: USART4 selected"]
    Usart4 = 2,
}
impl From<IR_MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: IR_MOD_A) -> Self {
        variant as _
    }
}
impl IR_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IR_MOD_A> {
        match self.bits {
            0 => Some(IR_MOD_A::Tim16),
            1 => Some(IR_MOD_A::Usart1),
            2 => Some(IR_MOD_A::Usart4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tim16`"]
    #[inline(always)]
    pub fn is_tim16(&self) -> bool {
        *self == IR_MOD_A::Tim16
    }
    #[doc = "Checks if the value of the field is `Usart1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == IR_MOD_A::Usart1
    }
    #[doc = "Checks if the value of the field is `Usart4`"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == IR_MOD_A::Usart4
    }
}
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection"]
pub type IR_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, IR_MOD_A, 2, O>;
impl<'a, const O: u8> IR_MOD_W<'a, O> {
    #[doc = "TIM16 selected"]
    #[inline(always)]
    pub fn tim16(self) -> &'a mut W {
        self.variant(IR_MOD_A::Tim16)
    }
    #[doc = "USART1 selected"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(IR_MOD_A::Usart1)
    }
    #[doc = "USART4 selected"]
    #[inline(always)]
    pub fn usart4(self) -> &'a mut W {
        self.variant(IR_MOD_A::Usart4)
    }
}
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA remapping bit"]
pub type ADC_DMA_RMP_R = crate::BitReader<ADC_DMA_RMP_A>;
#[doc = "ADC DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DMA_RMP_A {
    #[doc = "0: ADC DMA request mapped on DMA channel 1"]
    NotRemapped = 0,
    #[doc = "1: ADC DMA request mapped on DMA channel 2"]
    Remapped = 1,
}
impl From<ADC_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DMA_RMP_A {
        match self.bits {
            false => ADC_DMA_RMP_A::NotRemapped,
            true => ADC_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == ADC_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == ADC_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA remapping bit"]
pub type ADC_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, ADC_DMA_RMP_A, O>;
impl<'a, const O: u8> ADC_DMA_RMP_W<'a, O> {
    #[doc = "ADC DMA request mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::NotRemapped)
    }
    #[doc = "ADC DMA request mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(ADC_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `USART1_TX_DMA_RMP` reader - USART1_TX DMA remapping bit"]
pub type USART1_TX_DMA_RMP_R = crate::BitReader<USART1_TX_DMA_RMP_A>;
#[doc = "USART1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1_TX_DMA_RMP_A {
    #[doc = "0: USART1_TX DMA request mapped on DMA channel 2"]
    NotRemapped = 0,
    #[doc = "1: USART1_TX DMA request mapped on DMA channel 4"]
    Remapped = 1,
}
impl From<USART1_TX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_TX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1_TX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_TX_DMA_RMP_A {
        match self.bits {
            false => USART1_TX_DMA_RMP_A::NotRemapped,
            true => USART1_TX_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_TX_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `USART1_TX_DMA_RMP` writer - USART1_TX DMA remapping bit"]
pub type USART1_TX_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, USART1_TX_DMA_RMP_A, O>;
impl<'a, const O: u8> USART1_TX_DMA_RMP_W<'a, O> {
    #[doc = "USART1_TX DMA request mapped on DMA channel 2"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART1_TX_DMA_RMP_A::NotRemapped)
    }
    #[doc = "USART1_TX DMA request mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART1_TX_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `USART1_RX_DMA_RMP` reader - USART1_RX DMA request remapping bit"]
pub type USART1_RX_DMA_RMP_R = crate::BitReader<USART1_RX_DMA_RMP_A>;
#[doc = "USART1_RX DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1_RX_DMA_RMP_A {
    #[doc = "0: USART1_RX DMA request mapped on DMA channel 3"]
    NotRemapped = 0,
    #[doc = "1: USART1_RX DMA request mapped on DMA channel 5"]
    Remapped = 1,
}
impl From<USART1_RX_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_RX_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1_RX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_RX_DMA_RMP_A {
        match self.bits {
            false => USART1_RX_DMA_RMP_A::NotRemapped,
            true => USART1_RX_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART1_RX_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `USART1_RX_DMA_RMP` writer - USART1_RX DMA request remapping bit"]
pub type USART1_RX_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, USART1_RX_DMA_RMP_A, O>;
impl<'a, const O: u8> USART1_RX_DMA_RMP_W<'a, O> {
    #[doc = "USART1_RX DMA request mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART1_RX_DMA_RMP_A::NotRemapped)
    }
    #[doc = "USART1_RX DMA request mapped on DMA channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART1_RX_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_R = crate::BitReader<TIM16_DMA_RMP_A>;
#[doc = "TIM16 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP_A {
    #[doc = "0: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3"]
    NotRemapped = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
    Remapped = 1,
}
impl From<TIM16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP_A {
        match self.bits {
            false => TIM16_DMA_RMP_A::NotRemapped,
            true => TIM16_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM16_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM16_DMA_RMP_W<'a, O> {
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_R = crate::BitReader<TIM17_DMA_RMP_A>;
#[doc = "TIM17 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP_A {
    #[doc = "0: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1"]
    NotRemapped = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
    Remapped = 1,
}
impl From<TIM17_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP_A {
        match self.bits {
            false => TIM17_DMA_RMP_A::NotRemapped,
            true => TIM17_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM17_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM17_DMA_RMP_W<'a, O> {
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM16_DMA_RMP2` reader - TIM16 alternate DMA request remapping bit"]
pub type TIM16_DMA_RMP2_R = crate::BitReader<TIM16_DMA_RMP2_A>;
#[doc = "TIM16 alternate DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP2_A {
    #[doc = "0: TIM16 DMA request mapped according to TIM16_DMA_RMP bit"]
    NotAlternateRemapped = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
    AlternateRemapped = 1,
}
impl From<TIM16_DMA_RMP2_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16_DMA_RMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP2_A {
        match self.bits {
            false => TIM16_DMA_RMP2_A::NotAlternateRemapped,
            true => TIM16_DMA_RMP2_A::AlternateRemapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotAlternateRemapped`"]
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2_A::NotAlternateRemapped
    }
    #[doc = "Checks if the value of the field is `AlternateRemapped`"]
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP2_A::AlternateRemapped
    }
}
#[doc = "Field `TIM16_DMA_RMP2` writer - TIM16 alternate DMA request remapping bit"]
pub type TIM16_DMA_RMP2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM16_DMA_RMP2_A, O>;
impl<'a, const O: u8> TIM16_DMA_RMP2_W<'a, O> {
    #[doc = "TIM16 DMA request mapped according to TIM16_DMA_RMP bit"]
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP2_A::NotAlternateRemapped)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP2_A::AlternateRemapped)
    }
}
#[doc = "Field `TIM17_DMA_RMP2` reader - TIM17 alternate DMA request remapping bit"]
pub type TIM17_DMA_RMP2_R = crate::BitReader<TIM17_DMA_RMP2_A>;
#[doc = "TIM17 alternate DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP2_A {
    #[doc = "0: TIM17 DMA request mapped according to TIM16_DMA_RMP bit"]
    NotAlternateRemapped = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
    AlternateRemapped = 1,
}
impl From<TIM17_DMA_RMP2_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17_DMA_RMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP2_A {
        match self.bits {
            false => TIM17_DMA_RMP2_A::NotAlternateRemapped,
            true => TIM17_DMA_RMP2_A::AlternateRemapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotAlternateRemapped`"]
    #[inline(always)]
    pub fn is_not_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2_A::NotAlternateRemapped
    }
    #[doc = "Checks if the value of the field is `AlternateRemapped`"]
    #[inline(always)]
    pub fn is_alternate_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP2_A::AlternateRemapped
    }
}
#[doc = "Field `TIM17_DMA_RMP2` writer - TIM17 alternate DMA request remapping bit"]
pub type TIM17_DMA_RMP2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM17_DMA_RMP2_A, O>;
impl<'a, const O: u8> TIM17_DMA_RMP2_W<'a, O> {
    #[doc = "TIM17 DMA request mapped according to TIM16_DMA_RMP bit"]
    #[inline(always)]
    pub fn not_alternate_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP2_A::NotAlternateRemapped)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
    #[inline(always)]
    pub fn alternate_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP2_A::AlternateRemapped)
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM plus) driving capability activation bits."]
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP_A>;
#[doc = "Fast Mode Plus (FM plus) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP_A {
    #[doc = "0: PB6 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB6_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB6_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB6_FMP_A {
        match self.bits {
            false => I2C_PB6_FMP_A::Standard,
            true => I2C_PB6_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM plus) driving capability activation bits."]
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB6_FMP_A, O>;
impl<'a, const O: u8> I2C_PB6_FMP_W<'a, O> {
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP_A>;
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP_A {
    #[doc = "0: PB7 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB7_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB7_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB7_FMP_A {
        match self.bits {
            false => I2C_PB7_FMP_A::Standard,
            true => I2C_PB7_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB7_FMP_A, O>;
impl<'a, const O: u8> I2C_PB7_FMP_W<'a, O> {
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP_A>;
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP_A {
    #[doc = "0: PB8 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB8_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB8_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB8_FMP_A {
        match self.bits {
            false => I2C_PB8_FMP_A::Standard,
            true => I2C_PB8_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB8_FMP_A, O>;
impl<'a, const O: u8> I2C_PB8_FMP_W<'a, O> {
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP_A>;
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP_A {
    #[doc = "0: PB9 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB9_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB9_FMP_A {
        match self.bits {
            false => I2C_PB9_FMP_A::Standard,
            true => I2C_PB9_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB9_FMP_A, O>;
impl<'a, const O: u8> I2C_PB9_FMP_W<'a, O> {
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1"]
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP_A>;
#[doc = "FM+ driving capability activation for I2C1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    Fmp = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_FMP_A {
        match self.bits {
            false => I2C1_FMP_A::Standard,
            true => I2C1_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP_A::Fmp
    }
}
#[doc = "Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C1_FMP_A, O>;
impl<'a, const O: u8> I2C1_FMP_W<'a, O> {
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2"]
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP_A>;
#[doc = "FM+ driving capability activation for I2C2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    Fmp = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_FMP_A {
        match self.bits {
            false => I2C2_FMP_A::Standard,
            true => I2C2_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP_A::Fmp
    }
}
#[doc = "Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2"]
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C2_FMP_A, O>;
impl<'a, const O: u8> I2C2_FMP_W<'a, O> {
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PA9_FMP_R = crate::BitReader<I2C_PA9_FMP_A>;
#[doc = "Fast Mode Plus (FM+) driving capability activation bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA9_FMP_A {
    #[doc = "0: PA9 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PA9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PA9_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PA9_FMP_A {
        match self.bits {
            false => I2C_PA9_FMP_A::Standard,
            true => I2C_PA9_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA9_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA9_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PA9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PA9_FMP_A, O>;
impl<'a, const O: u8> I2C_PA9_FMP_W<'a, O> {
    #[doc = "PA9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PA9_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PA9_FMP_A::Fmp)
    }
}
#[doc = "Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PA10_FMP_R = crate::BitReader<I2C_PA10_FMP_A>;
#[doc = "Fast Mode Plus (FM+) driving capability activation bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA10_FMP_A {
    #[doc = "0: PA10 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PA10_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA10_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PA10_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PA10_FMP_A {
        match self.bits {
            false => I2C_PA10_FMP_A::Standard,
            true => I2C_PA10_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PA10_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PA10_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub type I2C_PA10_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PA10_FMP_A, O>;
impl<'a, const O: u8> I2C_PA10_FMP_W<'a, O> {
    #[doc = "PA10 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PA10_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PA10_FMP_A::Fmp)
    }
}
#[doc = "Field `SPI2_DMA_RMP` reader - SPI2 DMA request remapping bit"]
pub type SPI2_DMA_RMP_R = crate::BitReader<SPI2_DMA_RMP_A>;
#[doc = "SPI2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2_DMA_RMP_A {
    #[doc = "0: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively"]
    NotRemapped = 0,
    #[doc = "1: SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    Remapped = 1,
}
impl From<SPI2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2_DMA_RMP_A {
        match self.bits {
            false => SPI2_DMA_RMP_A::NotRemapped,
            true => SPI2_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == SPI2_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `SPI2_DMA_RMP` writer - SPI2 DMA request remapping bit"]
pub type SPI2_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SPI2_DMA_RMP_A, O>;
impl<'a, const O: u8> SPI2_DMA_RMP_W<'a, O> {
    #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(SPI2_DMA_RMP_A::NotRemapped)
    }
    #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(SPI2_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `USART2_DMA_RMP` reader - USART2 DMA request remapping bit"]
pub type USART2_DMA_RMP_R = crate::BitReader<USART2_DMA_RMP_A>;
#[doc = "USART2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2_DMA_RMP_A {
    #[doc = "0: USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively"]
    NotRemapped = 0,
    #[doc = "1: USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    Remapped = 1,
}
impl From<USART2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2_DMA_RMP_A {
        match self.bits {
            false => USART2_DMA_RMP_A::NotRemapped,
            true => USART2_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART2_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART2_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `USART2_DMA_RMP` writer - USART2 DMA request remapping bit"]
pub type USART2_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, USART2_DMA_RMP_A, O>;
impl<'a, const O: u8> USART2_DMA_RMP_W<'a, O> {
    #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART2_DMA_RMP_A::NotRemapped)
    }
    #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART2_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `USART3_DMA_RMP` reader - USART3 DMA request remapping bit"]
pub type USART3_DMA_RMP_R = crate::BitReader<USART3_DMA_RMP_A>;
#[doc = "USART3 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART3_DMA_RMP_A {
    #[doc = "0: USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)"]
    NotRemapped = 0,
    #[doc = "1: USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    Remapped = 1,
}
impl From<USART3_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USART3_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USART3_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3_DMA_RMP_A {
        match self.bits {
            false => USART3_DMA_RMP_A::NotRemapped,
            true => USART3_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USART3_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USART3_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `USART3_DMA_RMP` writer - USART3 DMA request remapping bit"]
pub type USART3_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, USART3_DMA_RMP_A, O>;
impl<'a, const O: u8> USART3_DMA_RMP_W<'a, O> {
    #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USART3_DMA_RMP_A::NotRemapped)
    }
    #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USART3_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `I2C1_DMA_RMP` reader - I2C1 DMA request remapping bit"]
pub type I2C1_DMA_RMP_R = crate::BitReader<I2C1_DMA_RMP_A>;
#[doc = "I2C1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_DMA_RMP_A {
    #[doc = "0: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    NotRemapped = 0,
    #[doc = "1: I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
    Remapped = 1,
}
impl From<I2C1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_DMA_RMP_A {
        match self.bits {
            false => I2C1_DMA_RMP_A::NotRemapped,
            true => I2C1_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == I2C1_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `I2C1_DMA_RMP` writer - I2C1 DMA request remapping bit"]
pub type I2C1_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C1_DMA_RMP_A, O>;
impl<'a, const O: u8> I2C1_DMA_RMP_W<'a, O> {
    #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(I2C1_DMA_RMP_A::NotRemapped)
    }
    #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(I2C1_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM1_DMA_RMP` reader - TIM1 DMA request remapping bit"]
pub type TIM1_DMA_RMP_R = crate::BitReader<TIM1_DMA_RMP_A>;
#[doc = "TIM1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_DMA_RMP_A {
    #[doc = "0: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively"]
    NotRemapped = 0,
    #[doc = "1: TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
    Remapped = 1,
}
impl From<TIM1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1_DMA_RMP_A {
        match self.bits {
            false => TIM1_DMA_RMP_A::NotRemapped,
            true => TIM1_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM1_DMA_RMP` writer - TIM1 DMA request remapping bit"]
pub type TIM1_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, TIM1_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM1_DMA_RMP_W<'a, O> {
    #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM1_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM1_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM2_DMA_RMP` reader - TIM2 DMA request remapping bit"]
pub type TIM2_DMA_RMP_R = crate::BitReader<TIM2_DMA_RMP_A>;
#[doc = "TIM2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2_DMA_RMP_A {
    #[doc = "0: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively"]
    NotRemapped = 0,
    #[doc = "1: TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
    Remapped = 1,
}
impl From<TIM2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2_DMA_RMP_A {
        match self.bits {
            false => TIM2_DMA_RMP_A::NotRemapped,
            true => TIM2_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM2_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM2_DMA_RMP` writer - TIM2 DMA request remapping bit"]
pub type TIM2_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, TIM2_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM2_DMA_RMP_W<'a, O> {
    #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM2_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM2_DMA_RMP_A::Remapped)
    }
}
#[doc = "Field `TIM3_DMA_RMP` reader - TIM3 DMA request remapping bit"]
pub type TIM3_DMA_RMP_R = crate::BitReader<TIM3_DMA_RMP_A>;
#[doc = "TIM3 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3_DMA_RMP_A {
    #[doc = "0: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4"]
    NotRemapped = 0,
    #[doc = "1: TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
    Remapped = 1,
}
impl From<TIM3_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM3_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM3_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM3_DMA_RMP_A {
        match self.bits {
            false => TIM3_DMA_RMP_A::NotRemapped,
            true => TIM3_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM3_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM3_DMA_RMP` writer - TIM3 DMA request remapping bit"]
pub type TIM3_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, TIM3_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM3_DMA_RMP_W<'a, O> {
    #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM3_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM3_DMA_RMP_A::Remapped)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection"]
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&self) -> USART1_TX_DMA_RMP_R {
        USART1_TX_DMA_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&self) -> USART1_RX_DMA_RMP_R {
        USART1_RX_DMA_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM16 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp2(&self) -> TIM16_DMA_RMP2_R {
        TIM16_DMA_RMP2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM17 alternate DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp2(&self) -> TIM17_DMA_RMP2_R {
        TIM17_DMA_RMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    pub fn spi2_dma_rmp(&self) -> SPI2_DMA_RMP_R {
        SPI2_DMA_RMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart2_dma_rmp(&self) -> USART2_DMA_RMP_R {
        USART2_DMA_RMP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart3_dma_rmp(&self) -> USART3_DMA_RMP_R {
        USART3_DMA_RMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    pub fn i2c1_dma_rmp(&self) -> I2C1_DMA_RMP_R {
        I2C1_DMA_RMP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim1_dma_rmp(&self) -> TIM1_DMA_RMP_R {
        TIM1_DMA_RMP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim2_dma_rmp(&self) -> TIM2_DMA_RMP_R {
        TIM2_DMA_RMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim3_dma_rmp(&self) -> TIM3_DMA_RMP_R {
        TIM3_DMA_RMP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W<4> {
        PA11_PA12_RMP_W::new(self)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_mod(&mut self) -> IR_MOD_W<6> {
        IR_MOD_W::new(self)
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W<8> {
        ADC_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_tx_dma_rmp(&mut self) -> USART1_TX_DMA_RMP_W<9> {
        USART1_TX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_rx_dma_rmp(&mut self) -> USART1_RX_DMA_RMP_W<10> {
        USART1_RX_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<11> {
        TIM16_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<12> {
        TIM17_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 13 - TIM16 alternate DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim16_dma_rmp2(&mut self) -> TIM16_DMA_RMP2_W<13> {
        TIM16_DMA_RMP2_W::new(self)
    }
    #[doc = "Bit 14 - TIM17 alternate DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim17_dma_rmp2(&mut self) -> TIM17_DMA_RMP2_W<14> {
        TIM17_DMA_RMP2_W::new(self)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<16> {
        I2C_PB6_FMP_W::new(self)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<17> {
        I2C_PB7_FMP_W::new(self)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<18> {
        I2C_PB8_FMP_W::new(self)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<19> {
        I2C_PB9_FMP_W::new(self)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<20> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<21> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Bit 22 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<22> {
        I2C_PA9_FMP_W::new(self)
    }
    #[doc = "Bit 23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<23> {
        I2C_PA10_FMP_W::new(self)
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_dma_rmp(&mut self) -> SPI2_DMA_RMP_W<24> {
        SPI2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_dma_rmp(&mut self) -> USART2_DMA_RMP_W<25> {
        USART2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_dma_rmp(&mut self) -> USART3_DMA_RMP_W<26> {
        USART3_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_dma_rmp(&mut self) -> I2C1_DMA_RMP_W<27> {
        I2C1_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_dma_rmp(&mut self) -> TIM1_DMA_RMP_W<28> {
        TIM1_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_dma_rmp(&mut self) -> TIM2_DMA_RMP_W<29> {
        TIM2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_dma_rmp(&mut self) -> TIM3_DMA_RMP_W<30> {
        TIM3_DMA_RMP_W::new(self)
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
