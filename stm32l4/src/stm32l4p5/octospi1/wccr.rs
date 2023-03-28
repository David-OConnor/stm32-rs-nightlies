#[doc = "Register `WCCR` reader"]
pub struct R(crate::R<WCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCCR` writer"]
pub struct W(crate::W<WCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCCR_SPEC>;
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
impl From<crate::W<WCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMODE` reader - Instruction mode"]
pub type IMODE_R = crate::FieldReader<u8, IMODE_A>;
#[doc = "Instruction mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMODE_A {
    #[doc = "0: No instruction"]
    NoInstruction = 0,
    #[doc = "1: Instruction on a single line"]
    SingleLine = 1,
    #[doc = "2: Instruction on two lines"]
    TwoLines = 2,
    #[doc = "3: Instruction on four lines"]
    FourLines = 3,
    #[doc = "4: Instruction on eight lines"]
    EightLines = 4,
}
impl From<IMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IMODE_A) -> Self {
        variant as _
    }
}
impl IMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMODE_A> {
        match self.bits {
            0 => Some(IMODE_A::NoInstruction),
            1 => Some(IMODE_A::SingleLine),
            2 => Some(IMODE_A::TwoLines),
            3 => Some(IMODE_A::FourLines),
            4 => Some(IMODE_A::EightLines),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoInstruction`"]
    #[inline(always)]
    pub fn is_no_instruction(&self) -> bool {
        *self == IMODE_A::NoInstruction
    }
    #[doc = "Checks if the value of the field is `SingleLine`"]
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == IMODE_A::SingleLine
    }
    #[doc = "Checks if the value of the field is `TwoLines`"]
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == IMODE_A::TwoLines
    }
    #[doc = "Checks if the value of the field is `FourLines`"]
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == IMODE_A::FourLines
    }
    #[doc = "Checks if the value of the field is `EightLines`"]
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == IMODE_A::EightLines
    }
}
#[doc = "Field `IMODE` writer - Instruction mode"]
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, IMODE_A, 3, O>;
impl<'a, const O: u8> IMODE_W<'a, O> {
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn no_instruction(self) -> &'a mut W {
        self.variant(IMODE_A::NoInstruction)
    }
    #[doc = "Instruction on a single line"]
    #[inline(always)]
    pub fn single_line(self) -> &'a mut W {
        self.variant(IMODE_A::SingleLine)
    }
    #[doc = "Instruction on two lines"]
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut W {
        self.variant(IMODE_A::TwoLines)
    }
    #[doc = "Instruction on four lines"]
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut W {
        self.variant(IMODE_A::FourLines)
    }
    #[doc = "Instruction on eight lines"]
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut W {
        self.variant(IMODE_A::EightLines)
    }
}
#[doc = "Field `IDTR` reader - Instruction double transfer rate"]
pub type IDTR_R = crate::BitReader<IDTR_A>;
#[doc = "Instruction double transfer rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDTR_A {
    #[doc = "0: DTR mode disabled for instruction phase"]
    Disabled = 0,
    #[doc = "1: DTR mode enabled for instruction phase"]
    Enabled = 1,
}
impl From<IDTR_A> for bool {
    #[inline(always)]
    fn from(variant: IDTR_A) -> Self {
        variant as u8 != 0
    }
}
impl IDTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDTR_A {
        match self.bits {
            false => IDTR_A::Disabled,
            true => IDTR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDTR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDTR_A::Enabled
    }
}
#[doc = "Field `IDTR` writer - Instruction double transfer rate"]
pub type IDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, IDTR_A, O>;
impl<'a, const O: u8> IDTR_W<'a, O> {
    #[doc = "DTR mode disabled for instruction phase"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDTR_A::Disabled)
    }
    #[doc = "DTR mode enabled for instruction phase"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDTR_A::Enabled)
    }
}
#[doc = "Field `ISIZE` reader - Instruction size"]
pub type ISIZE_R = crate::FieldReader<u8, ISIZE_A>;
#[doc = "Instruction size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISIZE_A {
    #[doc = "0: 8-bit instruction"]
    Bits8 = 0,
    #[doc = "1: 16-bit instruction"]
    Bits16 = 1,
    #[doc = "2: 24-bit instruction"]
    Bits24 = 2,
    #[doc = "3: 32-bit instruction"]
    Bits32 = 3,
}
impl From<ISIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ISIZE_A) -> Self {
        variant as _
    }
}
impl ISIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISIZE_A {
        match self.bits {
            0 => ISIZE_A::Bits8,
            1 => ISIZE_A::Bits16,
            2 => ISIZE_A::Bits24,
            3 => ISIZE_A::Bits32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ISIZE_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ISIZE_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits24`"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ISIZE_A::Bits24
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ISIZE_A::Bits32
    }
}
#[doc = "Field `ISIZE` writer - Instruction size"]
pub type ISIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WCCR_SPEC, u8, ISIZE_A, 2, O>;
impl<'a, const O: u8> ISIZE_W<'a, O> {
    #[doc = "8-bit instruction"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(ISIZE_A::Bits8)
    }
    #[doc = "16-bit instruction"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ISIZE_A::Bits16)
    }
    #[doc = "24-bit instruction"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut W {
        self.variant(ISIZE_A::Bits24)
    }
    #[doc = "32-bit instruction"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ISIZE_A::Bits32)
    }
}
#[doc = "Field `ADMODE` reader - Address mode"]
pub type ADMODE_R = crate::FieldReader<u8, ADMODE_A>;
#[doc = "Address mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADMODE_A {
    #[doc = "0: No address"]
    NoAddress = 0,
    #[doc = "1: Address on a single line"]
    SingleLine = 1,
    #[doc = "2: Address on two lines"]
    TwoLines = 2,
    #[doc = "3: Address on four lines"]
    FourLines = 3,
    #[doc = "4: Address on eight lines"]
    EightLines = 4,
}
impl From<ADMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADMODE_A) -> Self {
        variant as _
    }
}
impl ADMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADMODE_A> {
        match self.bits {
            0 => Some(ADMODE_A::NoAddress),
            1 => Some(ADMODE_A::SingleLine),
            2 => Some(ADMODE_A::TwoLines),
            3 => Some(ADMODE_A::FourLines),
            4 => Some(ADMODE_A::EightLines),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoAddress`"]
    #[inline(always)]
    pub fn is_no_address(&self) -> bool {
        *self == ADMODE_A::NoAddress
    }
    #[doc = "Checks if the value of the field is `SingleLine`"]
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ADMODE_A::SingleLine
    }
    #[doc = "Checks if the value of the field is `TwoLines`"]
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ADMODE_A::TwoLines
    }
    #[doc = "Checks if the value of the field is `FourLines`"]
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ADMODE_A::FourLines
    }
    #[doc = "Checks if the value of the field is `EightLines`"]
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == ADMODE_A::EightLines
    }
}
#[doc = "Field `ADMODE` writer - Address mode"]
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, ADMODE_A, 3, O>;
impl<'a, const O: u8> ADMODE_W<'a, O> {
    #[doc = "No address"]
    #[inline(always)]
    pub fn no_address(self) -> &'a mut W {
        self.variant(ADMODE_A::NoAddress)
    }
    #[doc = "Address on a single line"]
    #[inline(always)]
    pub fn single_line(self) -> &'a mut W {
        self.variant(ADMODE_A::SingleLine)
    }
    #[doc = "Address on two lines"]
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut W {
        self.variant(ADMODE_A::TwoLines)
    }
    #[doc = "Address on four lines"]
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut W {
        self.variant(ADMODE_A::FourLines)
    }
    #[doc = "Address on eight lines"]
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut W {
        self.variant(ADMODE_A::EightLines)
    }
}
#[doc = "Field `ADDTR` reader - Address double transfer rate"]
pub type ADDTR_R = crate::BitReader<ADDTR_A>;
#[doc = "Address double transfer rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDTR_A {
    #[doc = "0: DTR mode disabled for address phase"]
    Disabled = 0,
    #[doc = "1: DTR mode enabled for address phase"]
    Enabled = 1,
}
impl From<ADDTR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDTR_A {
        match self.bits {
            false => ADDTR_A::Disabled,
            true => ADDTR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDTR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDTR_A::Enabled
    }
}
#[doc = "Field `ADDTR` writer - Address double transfer rate"]
pub type ADDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, ADDTR_A, O>;
impl<'a, const O: u8> ADDTR_W<'a, O> {
    #[doc = "DTR mode disabled for address phase"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDTR_A::Disabled)
    }
    #[doc = "DTR mode enabled for address phase"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDTR_A::Enabled)
    }
}
#[doc = "Field `ADSIZE` reader - Address size"]
pub type ADSIZE_R = crate::FieldReader<u8, ADSIZE_A>;
#[doc = "Address size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADSIZE_A {
    #[doc = "0: 8-bit address"]
    Bits8 = 0,
    #[doc = "1: 16-bit address"]
    Bits16 = 1,
    #[doc = "2: 24-bit address"]
    Bits24 = 2,
    #[doc = "3: 32-bit address"]
    Bits32 = 3,
}
impl From<ADSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSIZE_A) -> Self {
        variant as _
    }
}
impl ADSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSIZE_A {
        match self.bits {
            0 => ADSIZE_A::Bits8,
            1 => ADSIZE_A::Bits16,
            2 => ADSIZE_A::Bits24,
            3 => ADSIZE_A::Bits32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ADSIZE_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ADSIZE_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits24`"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ADSIZE_A::Bits24
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ADSIZE_A::Bits32
    }
}
#[doc = "Field `ADSIZE` writer - Address size"]
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WCCR_SPEC, u8, ADSIZE_A, 2, O>;
impl<'a, const O: u8> ADSIZE_W<'a, O> {
    #[doc = "8-bit address"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(ADSIZE_A::Bits8)
    }
    #[doc = "16-bit address"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ADSIZE_A::Bits16)
    }
    #[doc = "24-bit address"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut W {
        self.variant(ADSIZE_A::Bits24)
    }
    #[doc = "32-bit address"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ADSIZE_A::Bits32)
    }
}
#[doc = "Field `ABMODE` reader - Alternate byte mode"]
pub type ABMODE_R = crate::FieldReader<u8, ABMODE_A>;
#[doc = "Alternate byte mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABMODE_A {
    #[doc = "0: No alternate bytes"]
    NoAlternateBytes = 0,
    #[doc = "1: Alternate bytes on a single line"]
    SingleLine = 1,
    #[doc = "2: Alternate bytes on two lines"]
    TwoLines = 2,
    #[doc = "3: Alternate bytes on four lines"]
    FourLines = 3,
    #[doc = "4: Alternate bytes on eight lines"]
    EightLines = 4,
}
impl From<ABMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ABMODE_A) -> Self {
        variant as _
    }
}
impl ABMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABMODE_A> {
        match self.bits {
            0 => Some(ABMODE_A::NoAlternateBytes),
            1 => Some(ABMODE_A::SingleLine),
            2 => Some(ABMODE_A::TwoLines),
            3 => Some(ABMODE_A::FourLines),
            4 => Some(ABMODE_A::EightLines),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoAlternateBytes`"]
    #[inline(always)]
    pub fn is_no_alternate_bytes(&self) -> bool {
        *self == ABMODE_A::NoAlternateBytes
    }
    #[doc = "Checks if the value of the field is `SingleLine`"]
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ABMODE_A::SingleLine
    }
    #[doc = "Checks if the value of the field is `TwoLines`"]
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ABMODE_A::TwoLines
    }
    #[doc = "Checks if the value of the field is `FourLines`"]
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ABMODE_A::FourLines
    }
    #[doc = "Checks if the value of the field is `EightLines`"]
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == ABMODE_A::EightLines
    }
}
#[doc = "Field `ABMODE` writer - Alternate byte mode"]
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, ABMODE_A, 3, O>;
impl<'a, const O: u8> ABMODE_W<'a, O> {
    #[doc = "No alternate bytes"]
    #[inline(always)]
    pub fn no_alternate_bytes(self) -> &'a mut W {
        self.variant(ABMODE_A::NoAlternateBytes)
    }
    #[doc = "Alternate bytes on a single line"]
    #[inline(always)]
    pub fn single_line(self) -> &'a mut W {
        self.variant(ABMODE_A::SingleLine)
    }
    #[doc = "Alternate bytes on two lines"]
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut W {
        self.variant(ABMODE_A::TwoLines)
    }
    #[doc = "Alternate bytes on four lines"]
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut W {
        self.variant(ABMODE_A::FourLines)
    }
    #[doc = "Alternate bytes on eight lines"]
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut W {
        self.variant(ABMODE_A::EightLines)
    }
}
#[doc = "Field `ABDTR` reader - Alternate bytes double transfer rate"]
pub type ABDTR_R = crate::BitReader<ABDTR_A>;
#[doc = "Alternate bytes double transfer rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABDTR_A {
    #[doc = "0: DTR mode disabled for alternate bytes phase"]
    Disabled = 0,
    #[doc = "1: DTR mode enabled for alternate bytes phase"]
    Enabled = 1,
}
impl From<ABDTR_A> for bool {
    #[inline(always)]
    fn from(variant: ABDTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABDTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABDTR_A {
        match self.bits {
            false => ABDTR_A::Disabled,
            true => ABDTR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABDTR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABDTR_A::Enabled
    }
}
#[doc = "Field `ABDTR` writer - Alternate bytes double transfer rate"]
pub type ABDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, ABDTR_A, O>;
impl<'a, const O: u8> ABDTR_W<'a, O> {
    #[doc = "DTR mode disabled for alternate bytes phase"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABDTR_A::Disabled)
    }
    #[doc = "DTR mode enabled for alternate bytes phase"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABDTR_A::Enabled)
    }
}
#[doc = "Field `ABSIZE` reader - Alternate bytes size"]
pub type ABSIZE_R = crate::FieldReader<u8, ABSIZE_A>;
#[doc = "Alternate bytes size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABSIZE_A {
    #[doc = "0: 8-bit alternate bytes"]
    Bits8 = 0,
    #[doc = "1: 16-bit alternate bytes"]
    Bits16 = 1,
    #[doc = "2: 24-bit alternate bytes"]
    Bits24 = 2,
    #[doc = "3: 32-bit alternate bytes"]
    Bits32 = 3,
}
impl From<ABSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ABSIZE_A) -> Self {
        variant as _
    }
}
impl ABSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABSIZE_A {
        match self.bits {
            0 => ABSIZE_A::Bits8,
            1 => ABSIZE_A::Bits16,
            2 => ABSIZE_A::Bits24,
            3 => ABSIZE_A::Bits32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == ABSIZE_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ABSIZE_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits24`"]
    #[inline(always)]
    pub fn is_bits24(&self) -> bool {
        *self == ABSIZE_A::Bits24
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ABSIZE_A::Bits32
    }
}
#[doc = "Field `ABSIZE` writer - Alternate bytes size"]
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WCCR_SPEC, u8, ABSIZE_A, 2, O>;
impl<'a, const O: u8> ABSIZE_W<'a, O> {
    #[doc = "8-bit alternate bytes"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(ABSIZE_A::Bits8)
    }
    #[doc = "16-bit alternate bytes"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ABSIZE_A::Bits16)
    }
    #[doc = "24-bit alternate bytes"]
    #[inline(always)]
    pub fn bits24(self) -> &'a mut W {
        self.variant(ABSIZE_A::Bits24)
    }
    #[doc = "32-bit alternate bytes"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ABSIZE_A::Bits32)
    }
}
#[doc = "Field `DMODE` reader - Data mode"]
pub type DMODE_R = crate::FieldReader<u8, DMODE_A>;
#[doc = "Data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMODE_A {
    #[doc = "0: No data"]
    NoData = 0,
    #[doc = "1: Data on a single line"]
    SingleLine = 1,
    #[doc = "2: Data on two lines"]
    TwoLines = 2,
    #[doc = "3: Data on four lines"]
    FourLines = 3,
    #[doc = "4: Data on eight lines"]
    EightLines = 4,
}
impl From<DMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMODE_A) -> Self {
        variant as _
    }
}
impl DMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMODE_A> {
        match self.bits {
            0 => Some(DMODE_A::NoData),
            1 => Some(DMODE_A::SingleLine),
            2 => Some(DMODE_A::TwoLines),
            3 => Some(DMODE_A::FourLines),
            4 => Some(DMODE_A::EightLines),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoData`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == DMODE_A::NoData
    }
    #[doc = "Checks if the value of the field is `SingleLine`"]
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == DMODE_A::SingleLine
    }
    #[doc = "Checks if the value of the field is `TwoLines`"]
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == DMODE_A::TwoLines
    }
    #[doc = "Checks if the value of the field is `FourLines`"]
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == DMODE_A::FourLines
    }
    #[doc = "Checks if the value of the field is `EightLines`"]
    #[inline(always)]
    pub fn is_eight_lines(&self) -> bool {
        *self == DMODE_A::EightLines
    }
}
#[doc = "Field `DMODE` writer - Data mode"]
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WCCR_SPEC, u8, DMODE_A, 3, O>;
impl<'a, const O: u8> DMODE_W<'a, O> {
    #[doc = "No data"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(DMODE_A::NoData)
    }
    #[doc = "Data on a single line"]
    #[inline(always)]
    pub fn single_line(self) -> &'a mut W {
        self.variant(DMODE_A::SingleLine)
    }
    #[doc = "Data on two lines"]
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut W {
        self.variant(DMODE_A::TwoLines)
    }
    #[doc = "Data on four lines"]
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut W {
        self.variant(DMODE_A::FourLines)
    }
    #[doc = "Data on eight lines"]
    #[inline(always)]
    pub fn eight_lines(self) -> &'a mut W {
        self.variant(DMODE_A::EightLines)
    }
}
#[doc = "Field `DDTR` reader - alternate bytes double transfer rate"]
pub type DDTR_R = crate::BitReader<DDTR_A>;
#[doc = "alternate bytes double transfer rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDTR_A {
    #[doc = "0: DTR mode disabled for data phase"]
    Disabled = 0,
    #[doc = "1: DTR mode enabled for data phase"]
    Enabled = 1,
}
impl From<DDTR_A> for bool {
    #[inline(always)]
    fn from(variant: DDTR_A) -> Self {
        variant as u8 != 0
    }
}
impl DDTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDTR_A {
        match self.bits {
            false => DDTR_A::Disabled,
            true => DDTR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDTR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDTR_A::Enabled
    }
}
#[doc = "Field `DDTR` writer - alternate bytes double transfer rate"]
pub type DDTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, DDTR_A, O>;
impl<'a, const O: u8> DDTR_W<'a, O> {
    #[doc = "DTR mode disabled for data phase"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDTR_A::Disabled)
    }
    #[doc = "DTR mode enabled for data phase"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DDTR_A::Enabled)
    }
}
#[doc = "Field `DQSE` reader - DQS enable"]
pub type DQSE_R = crate::BitReader<DQSE_A>;
#[doc = "DQS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DQSE_A {
    #[doc = "0: DQS disabled"]
    Disabled = 0,
    #[doc = "1: DQS enabled"]
    Enabled = 1,
}
impl From<DQSE_A> for bool {
    #[inline(always)]
    fn from(variant: DQSE_A) -> Self {
        variant as u8 != 0
    }
}
impl DQSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSE_A {
        match self.bits {
            false => DQSE_A::Disabled,
            true => DQSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DQSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DQSE_A::Enabled
    }
}
#[doc = "Field `DQSE` writer - DQS enable"]
pub type DQSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, DQSE_A, O>;
impl<'a, const O: u8> DQSE_W<'a, O> {
    #[doc = "DQS disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DQSE_A::Disabled)
    }
    #[doc = "DQS enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DQSE_A::Enabled)
    }
}
#[doc = "Field `SIOO` reader - Send instruction only once mode"]
pub type SIOO_R = crate::BitReader<bool>;
#[doc = "Field `SIOO` writer - Send instruction only once mode"]
pub type SIOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Instruction mode"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Instruction double transfer rate"]
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Instruction size"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Address mode"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Address double transfer rate"]
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Alternate byte mode"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate"]
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Alternate bytes size"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Data mode"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - alternate bytes double transfer rate"]
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DQS enable"]
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Send instruction only once mode"]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Instruction mode"]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<0> {
        IMODE_W::new(self)
    }
    #[doc = "Bit 3 - Instruction double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn idtr(&mut self) -> IDTR_W<3> {
        IDTR_W::new(self)
    }
    #[doc = "Bits 4:5 - Instruction size"]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<4> {
        ISIZE_W::new(self)
    }
    #[doc = "Bits 8:10 - Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<8> {
        ADMODE_W::new(self)
    }
    #[doc = "Bit 11 - Address double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn addtr(&mut self) -> ADDTR_W<11> {
        ADDTR_W::new(self)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Alternate byte mode"]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<16> {
        ABMODE_W::new(self)
    }
    #[doc = "Bit 19 - Alternate bytes double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn abdtr(&mut self) -> ABDTR_W<19> {
        ABDTR_W::new(self)
    }
    #[doc = "Bits 20:21 - Alternate bytes size"]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<20> {
        ABSIZE_W::new(self)
    }
    #[doc = "Bits 24:26 - Data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    #[doc = "Bit 27 - alternate bytes double transfer rate"]
    #[inline(always)]
    #[must_use]
    pub fn ddtr(&mut self) -> DDTR_W<27> {
        DDTR_W::new(self)
    }
    #[doc = "Bit 29 - DQS enable"]
    #[inline(always)]
    #[must_use]
    pub fn dqse(&mut self) -> DQSE_W<29> {
        DQSE_W::new(self)
    }
    #[doc = "Bit 31 - Send instruction only once mode"]
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<31> {
        SIOO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wccr](index.html) module"]
pub struct WCCR_SPEC;
impl crate::RegisterSpec for WCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wccr::R](R) reader structure"]
impl crate::Readable for WCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wccr::W](W) writer structure"]
impl crate::Writable for WCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCCR to value 0"]
impl crate::Resettable for WCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
