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
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader<EN1_A>;
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: DAC Channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X enabled"]
    Enabled = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::Disabled,
            true => EN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1_A::Enabled
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::Disabled)
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::Enabled)
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<TEN1_A>;
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1_A {
    #[doc = "0: DAC Channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X trigger enabled"]
    Enabled = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::Disabled,
            true => TEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1_A::Enabled
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEN1_A, O>;
impl<'a, const O: u8> TEN1_W<'a, O> {
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::Disabled)
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::Enabled)
    }
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<u8, TSEL1_A>;
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: SWTRIG1"]
    Swtrig = 0,
    #[doc = "1: dac_chx_trg1"]
    Tim1Trgo = 1,
    #[doc = "2: dac_chx_trg2"]
    Tim2Trgo = 2,
    #[doc = "3: dac_chx_trg3"]
    Trg3 = 3,
    #[doc = "4: dac_chx_trg4"]
    Trg4 = 4,
    #[doc = "5: dac_chx_trg5"]
    Trg5 = 5,
    #[doc = "6: dac_chx_trg6"]
    Trg6 = 6,
    #[doc = "7: dac_chx_trg7"]
    Trg7 = 7,
    #[doc = "8: dac_chx_trg8"]
    Trg8 = 8,
    #[doc = "9: dac_chx_trg9"]
    Trg9 = 9,
    #[doc = "10: dac_chx_trg10"]
    Trg10 = 10,
    #[doc = "11: dac_chx_trg11"]
    Lptim1Out = 11,
    #[doc = "12: dac_chx_trg12"]
    Lptim2Out = 12,
    #[doc = "13: dac_chx_trg13"]
    Lptim3Out = 13,
    #[doc = "14: dac_chx_trg14"]
    Exti9 = 14,
    #[doc = "15: dac_chx_trg15"]
    Trg15 = 15,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEL1_A {
        match self.bits {
            0 => TSEL1_A::Swtrig,
            1 => TSEL1_A::Tim1Trgo,
            2 => TSEL1_A::Tim2Trgo,
            3 => TSEL1_A::Trg3,
            4 => TSEL1_A::Trg4,
            5 => TSEL1_A::Trg5,
            6 => TSEL1_A::Trg6,
            7 => TSEL1_A::Trg7,
            8 => TSEL1_A::Trg8,
            9 => TSEL1_A::Trg9,
            10 => TSEL1_A::Trg10,
            11 => TSEL1_A::Lptim1Out,
            12 => TSEL1_A::Lptim2Out,
            13 => TSEL1_A::Lptim3Out,
            14 => TSEL1_A::Exti9,
            15 => TSEL1_A::Trg15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Swtrig`"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == TSEL1_A::Swtrig
    }
    #[doc = "Checks if the value of the field is `Tim1Trgo`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == TSEL1_A::Tim1Trgo
    }
    #[doc = "Checks if the value of the field is `Tim2Trgo`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1_A::Tim2Trgo
    }
    #[doc = "Checks if the value of the field is `Trg3`"]
    #[inline(always)]
    pub fn is_trg3(&self) -> bool {
        *self == TSEL1_A::Trg3
    }
    #[doc = "Checks if the value of the field is `Trg4`"]
    #[inline(always)]
    pub fn is_trg4(&self) -> bool {
        *self == TSEL1_A::Trg4
    }
    #[doc = "Checks if the value of the field is `Trg5`"]
    #[inline(always)]
    pub fn is_trg5(&self) -> bool {
        *self == TSEL1_A::Trg5
    }
    #[doc = "Checks if the value of the field is `Trg6`"]
    #[inline(always)]
    pub fn is_trg6(&self) -> bool {
        *self == TSEL1_A::Trg6
    }
    #[doc = "Checks if the value of the field is `Trg7`"]
    #[inline(always)]
    pub fn is_trg7(&self) -> bool {
        *self == TSEL1_A::Trg7
    }
    #[doc = "Checks if the value of the field is `Trg8`"]
    #[inline(always)]
    pub fn is_trg8(&self) -> bool {
        *self == TSEL1_A::Trg8
    }
    #[doc = "Checks if the value of the field is `Trg9`"]
    #[inline(always)]
    pub fn is_trg9(&self) -> bool {
        *self == TSEL1_A::Trg9
    }
    #[doc = "Checks if the value of the field is `Trg10`"]
    #[inline(always)]
    pub fn is_trg10(&self) -> bool {
        *self == TSEL1_A::Trg10
    }
    #[doc = "Checks if the value of the field is `Lptim1Out`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == TSEL1_A::Lptim1Out
    }
    #[doc = "Checks if the value of the field is `Lptim2Out`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == TSEL1_A::Lptim2Out
    }
    #[doc = "Checks if the value of the field is `Lptim3Out`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == TSEL1_A::Lptim3Out
    }
    #[doc = "Checks if the value of the field is `Exti9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1_A::Exti9
    }
    #[doc = "Checks if the value of the field is `Trg15`"]
    #[inline(always)]
    pub fn is_trg15(&self) -> bool {
        *self == TSEL1_A::Trg15
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, TSEL1_A, 4, O>;
impl<'a, const O: u8> TSEL1_W<'a, O> {
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut W {
        self.variant(TSEL1_A::Swtrig)
    }
    #[doc = "dac_chx_trg1"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim1Trgo)
    }
    #[doc = "dac_chx_trg2"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim2Trgo)
    }
    #[doc = "dac_chx_trg3"]
    #[inline(always)]
    pub fn trg3(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg3)
    }
    #[doc = "dac_chx_trg4"]
    #[inline(always)]
    pub fn trg4(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg4)
    }
    #[doc = "dac_chx_trg5"]
    #[inline(always)]
    pub fn trg5(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg5)
    }
    #[doc = "dac_chx_trg6"]
    #[inline(always)]
    pub fn trg6(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg6)
    }
    #[doc = "dac_chx_trg7"]
    #[inline(always)]
    pub fn trg7(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg7)
    }
    #[doc = "dac_chx_trg8"]
    #[inline(always)]
    pub fn trg8(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg8)
    }
    #[doc = "dac_chx_trg9"]
    #[inline(always)]
    pub fn trg9(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg9)
    }
    #[doc = "dac_chx_trg10"]
    #[inline(always)]
    pub fn trg10(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg10)
    }
    #[doc = "dac_chx_trg11"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(TSEL1_A::Lptim1Out)
    }
    #[doc = "dac_chx_trg12"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(TSEL1_A::Lptim2Out)
    }
    #[doc = "dac_chx_trg13"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(TSEL1_A::Lptim3Out)
    }
    #[doc = "dac_chx_trg14"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::Exti9)
    }
    #[doc = "dac_chx_trg15"]
    #[inline(always)]
    pub fn trg15(self) -> &'a mut W {
        self.variant(TSEL1_A::Trg15)
    }
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::FieldReader<u8, WAVE1_A>;
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: Wave generation disabled"]
    Disabled = 0,
    #[doc = "1: Noise wave generation enabled"]
    Noise = 1,
    #[doc = "2: Triangle wave generation enabled"]
    Triangle = 2,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
impl WAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::Disabled),
            1 => Some(WAVE1_A::Noise),
            2 => Some(WAVE1_A::Triangle),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Noise`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1_A::Noise
    }
    #[doc = "Checks if the value of the field is `Triangle`"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1_A::Triangle
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, WAVE1_A, 2, O>;
impl<'a, const O: u8> WAVE1_W<'a, O> {
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1_A::Disabled)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1_A::Noise)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1_A::Triangle)
    }
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader<u8, MAMP1_A>;
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP1_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    Amp1 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    Amp3 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    Amp7 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    Amp15 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    Amp31 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    Amp63 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    Amp127 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    Amp255 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    Amp511 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    Amp1023 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    Amp2047 = 10,
    #[doc = "11: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    Amp4095 = 11,
}
impl From<MAMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1_A) -> Self {
        variant as _
    }
}
impl MAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAMP1_A> {
        match self.bits {
            0 => Some(MAMP1_A::Amp1),
            1 => Some(MAMP1_A::Amp3),
            2 => Some(MAMP1_A::Amp7),
            3 => Some(MAMP1_A::Amp15),
            4 => Some(MAMP1_A::Amp31),
            5 => Some(MAMP1_A::Amp63),
            6 => Some(MAMP1_A::Amp127),
            7 => Some(MAMP1_A::Amp255),
            8 => Some(MAMP1_A::Amp511),
            9 => Some(MAMP1_A::Amp1023),
            10 => Some(MAMP1_A::Amp2047),
            11 => Some(MAMP1_A::Amp4095),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Amp1`"]
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        *self == MAMP1_A::Amp1
    }
    #[doc = "Checks if the value of the field is `Amp3`"]
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        *self == MAMP1_A::Amp3
    }
    #[doc = "Checks if the value of the field is `Amp7`"]
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        *self == MAMP1_A::Amp7
    }
    #[doc = "Checks if the value of the field is `Amp15`"]
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        *self == MAMP1_A::Amp15
    }
    #[doc = "Checks if the value of the field is `Amp31`"]
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        *self == MAMP1_A::Amp31
    }
    #[doc = "Checks if the value of the field is `Amp63`"]
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        *self == MAMP1_A::Amp63
    }
    #[doc = "Checks if the value of the field is `Amp127`"]
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        *self == MAMP1_A::Amp127
    }
    #[doc = "Checks if the value of the field is `Amp255`"]
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        *self == MAMP1_A::Amp255
    }
    #[doc = "Checks if the value of the field is `Amp511`"]
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        *self == MAMP1_A::Amp511
    }
    #[doc = "Checks if the value of the field is `Amp1023`"]
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        *self == MAMP1_A::Amp1023
    }
    #[doc = "Checks if the value of the field is `Amp2047`"]
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        *self == MAMP1_A::Amp2047
    }
    #[doc = "Checks if the value of the field is `Amp4095`"]
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        *self == MAMP1_A::Amp4095
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MAMP1_A, 4, O>;
impl<'a, const O: u8> MAMP1_W<'a, O> {
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn amp1(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp1)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn amp3(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp3)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn amp7(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp7)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn amp15(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp15)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn amp31(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp31)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn amp63(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp63)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn amp127(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp127)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn amp255(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp255)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn amp511(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp511)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp1023)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp2047)
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp4095)
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader<DMAEN1_A>;
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC Channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::Disabled,
            true => DMAEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1_A::Enabled
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN1_A, O>;
impl<'a, const O: u8> DMAEN1_W<'a, O> {
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Disabled)
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Enabled)
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1_A>;
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC Channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::Disabled,
            true => DMAUDRIE1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1_A::Enabled
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAUDRIE1_A, O>;
impl<'a, const O: u8> DMAUDRIE1_W<'a, O> {
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Disabled)
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Enabled)
    }
}
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable"]
pub type CEN1_R = crate::BitReader<CEN1_A>;
#[doc = "DAC Channel 1 calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN1_A {
    #[doc = "0: DAC Channel X Normal operating mode"]
    Normal = 0,
    #[doc = "1: DAC Channel X calibration mode"]
    Calibration = 1,
}
impl From<CEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN1_A {
        match self.bits {
            false => CEN1_A::Normal,
            true => CEN1_A::Calibration,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CEN1_A::Normal
    }
    #[doc = "Checks if the value of the field is `Calibration`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == CEN1_A::Calibration
    }
}
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable"]
pub type CEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CEN1_A, O>;
impl<'a, const O: u8> CEN1_W<'a, O> {
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CEN1_A::Normal)
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(CEN1_A::Calibration)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<1> {
        TEN1_W::new(self)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<2> {
        TSEL1_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<6> {
        WAVE1_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<8> {
        MAMP1_W::new(self)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<13> {
        DMAUDRIE1_W::new(self)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen1(&mut self) -> CEN1_W<14> {
        CEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
