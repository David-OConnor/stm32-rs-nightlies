#[doc = "Register `FLTINR1` reader"]
pub struct R(crate::R<FLTINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR1` writer"]
pub struct W(crate::W<FLTINR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR1_SPEC>;
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
impl From<crate::W<FLTINR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1E` reader - FLT1E"]
pub type FLT1E_R = crate::BitReader<FLT1E_A>;
#[doc = "FLT1E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1E_A {
    #[doc = "0: Fault input disabled"]
    Disabled = 0,
    #[doc = "1: Fault input enabled"]
    Enabled = 1,
}
impl From<FLT1E_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1E_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1E_A {
        match self.bits {
            false => FLT1E_A::Disabled,
            true => FLT1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT1E_A::Enabled
    }
}
#[doc = "Field `FLT1E` writer - FLT1E"]
pub type FLT1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR1_SPEC, FLT1E_A, O>;
impl<'a, const O: u8> FLT1E_W<'a, O> {
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1E_A::Disabled)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT1E_A::Enabled)
    }
}
#[doc = "Field `FLT1P` reader - FLT1P"]
pub type FLT1P_R = crate::BitReader<FLT1P_A>;
#[doc = "FLT1P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1P_A {
    #[doc = "0: Fault input is active low"]
    ActiveLow = 0,
    #[doc = "1: Fault input is active high"]
    ActiveHigh = 1,
}
impl From<FLT1P_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1P_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1P_A {
        match self.bits {
            false => FLT1P_A::ActiveLow,
            true => FLT1P_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT1P_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT1P_A::ActiveHigh
    }
}
#[doc = "Field `FLT1P` writer - FLT1P"]
pub type FLT1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR1_SPEC, FLT1P_A, O>;
impl<'a, const O: u8> FLT1P_W<'a, O> {
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT1P_A::ActiveLow)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT1P_A::ActiveHigh)
    }
}
#[doc = "Field `FLT1SRC` reader - FLT1SRC"]
pub type FLT1SRC_R = crate::BitReader<FLT1SRC_A>;
#[doc = "FLT1SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1SRC_A {
    #[doc = "0: Fault input is FLTx input pin"]
    Input = 0,
    #[doc = "1: Fault input is FLTn_Int signal"]
    Internal = 1,
}
impl From<FLT1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1SRC_A {
        match self.bits {
            false => FLT1SRC_A::Input,
            true => FLT1SRC_A::Internal,
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT1SRC_A::Input
    }
    #[doc = "Checks if the value of the field is `Internal`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT1SRC_A::Internal
    }
}
#[doc = "Field `FLT1SRC` writer - FLT1SRC"]
pub type FLT1SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR1_SPEC, FLT1SRC_A, O>;
impl<'a, const O: u8> FLT1SRC_W<'a, O> {
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT1SRC_A::Input)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT1SRC_A::Internal)
    }
}
#[doc = "Field `FLT1F` reader - FLT1F"]
pub type FLT1F_R = crate::FieldReader<u8, FLT1F_A>;
#[doc = "FLT1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT1F_A {
    #[doc = "0: No filter, FLTx acts asynchronously"]
    Disabled = 0,
    #[doc = "1: f_SAMPLING=f_HRTIM, N=2"]
    Div1N2 = 1,
    #[doc = "2: f_SAMPLING=f_HRTIM, N=4"]
    Div1N4 = 2,
    #[doc = "3: f_SAMPLING=f_HRTIM, N=8"]
    Div1N8 = 3,
    #[doc = "4: f_SAMPLING=f_HRTIM/2, N=6"]
    Div2N6 = 4,
    #[doc = "5: f_SAMPLING=f_HRTIM/2, N=8"]
    Div2N8 = 5,
    #[doc = "6: f_SAMPLING=f_HRTIM/4, N=6"]
    Div4N6 = 6,
    #[doc = "7: f_SAMPLING=f_HRTIM/4, N=8"]
    Div4N8 = 7,
    #[doc = "8: f_SAMPLING=f_HRTIM/8, N=6"]
    Div8N6 = 8,
    #[doc = "9: f_SAMPLING=f_HRTIM/8, N=8"]
    Div8N8 = 9,
    #[doc = "10: f_SAMPLING=f_HRTIM/16, N=5"]
    Div16N5 = 10,
    #[doc = "11: f_SAMPLING=f_HRTIM/16, N=6"]
    Div16N6 = 11,
    #[doc = "12: f_SAMPLING=f_HRTIM/16, N=8"]
    Div16N8 = 12,
    #[doc = "13: f_SAMPLING=f_HRTIM/32, N=5"]
    Div32N5 = 13,
    #[doc = "14: f_SAMPLING=f_HRTIM/32, N=6"]
    Div32N6 = 14,
    #[doc = "15: f_SAMPLING=f_HRTIM/32, N=8"]
    Div32N8 = 15,
}
impl From<FLT1F_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT1F_A) -> Self {
        variant as _
    }
}
impl FLT1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1F_A {
        match self.bits {
            0 => FLT1F_A::Disabled,
            1 => FLT1F_A::Div1N2,
            2 => FLT1F_A::Div1N4,
            3 => FLT1F_A::Div1N8,
            4 => FLT1F_A::Div2N6,
            5 => FLT1F_A::Div2N8,
            6 => FLT1F_A::Div4N6,
            7 => FLT1F_A::Div4N8,
            8 => FLT1F_A::Div8N6,
            9 => FLT1F_A::Div8N8,
            10 => FLT1F_A::Div16N5,
            11 => FLT1F_A::Div16N6,
            12 => FLT1F_A::Div16N8,
            13 => FLT1F_A::Div32N5,
            14 => FLT1F_A::Div32N6,
            15 => FLT1F_A::Div32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1F_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Div1N2`"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT1F_A::Div1N2
    }
    #[doc = "Checks if the value of the field is `Div1N4`"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT1F_A::Div1N4
    }
    #[doc = "Checks if the value of the field is `Div1N8`"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT1F_A::Div1N8
    }
    #[doc = "Checks if the value of the field is `Div2N6`"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT1F_A::Div2N6
    }
    #[doc = "Checks if the value of the field is `Div2N8`"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT1F_A::Div2N8
    }
    #[doc = "Checks if the value of the field is `Div4N6`"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT1F_A::Div4N6
    }
    #[doc = "Checks if the value of the field is `Div4N8`"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT1F_A::Div4N8
    }
    #[doc = "Checks if the value of the field is `Div8N6`"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT1F_A::Div8N6
    }
    #[doc = "Checks if the value of the field is `Div8N8`"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT1F_A::Div8N8
    }
    #[doc = "Checks if the value of the field is `Div16N5`"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT1F_A::Div16N5
    }
    #[doc = "Checks if the value of the field is `Div16N6`"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT1F_A::Div16N6
    }
    #[doc = "Checks if the value of the field is `Div16N8`"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT1F_A::Div16N8
    }
    #[doc = "Checks if the value of the field is `Div32N5`"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT1F_A::Div32N5
    }
    #[doc = "Checks if the value of the field is `Div32N6`"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT1F_A::Div32N6
    }
    #[doc = "Checks if the value of the field is `Div32N8`"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT1F_A::Div32N8
    }
}
#[doc = "Field `FLT1F` writer - FLT1F"]
pub type FLT1F_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTINR1_SPEC, u8, FLT1F_A, 4, O>;
impl<'a, const O: u8> FLT1F_W<'a, O> {
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1F_A::Disabled)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT1F_A::Div1N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT1F_A::Div1N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div1N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::Div2N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div2N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::Div4N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div4N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::Div8N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div8N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT1F_A::Div16N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::Div16N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div16N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT1F_A::Div32N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::Div32N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::Div32N8)
    }
}
#[doc = "Field `FLT1LCK` reader - FLT1LCK"]
pub type FLT1LCK_R = crate::BitReader<FLT1LCKR_A>;
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKR_A {
    #[doc = "0: Fault bits are read/write"]
    Unlocked = 0,
    #[doc = "1: Fault bits are read-only"]
    Locked = 1,
}
impl From<FLT1LCKR_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKR_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1LCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1LCKR_A {
        match self.bits {
            false => FLT1LCKR_A::Unlocked,
            true => FLT1LCKR_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT1LCKR_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT1LCKR_A::Locked
    }
}
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKW_AW {
    #[doc = "1: Lock corresponding fault bits"]
    Lock = 1,
}
impl From<FLT1LCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1LCK` writer - FLT1LCK"]
pub type FLT1LCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR1_SPEC, FLT1LCKW_AW, O>;
impl<'a, const O: u8> FLT1LCK_W<'a, O> {
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT1LCKW_AW::Lock)
    }
}
#[doc = "Field `FLT2E` reader - FLT2E"]
pub use FLT1E_R as FLT2E_R;
#[doc = "Field `FLT3E` reader - FLT3E"]
pub use FLT1E_R as FLT3E_R;
#[doc = "Field `FLT4E` reader - FLT4E"]
pub use FLT1E_R as FLT4E_R;
#[doc = "Field `FLT2E` writer - FLT2E"]
pub use FLT1E_W as FLT2E_W;
#[doc = "Field `FLT3E` writer - FLT3E"]
pub use FLT1E_W as FLT3E_W;
#[doc = "Field `FLT4E` writer - FLT4E"]
pub use FLT1E_W as FLT4E_W;
#[doc = "Field `FLT2F` reader - FLT2F"]
pub use FLT1F_R as FLT2F_R;
#[doc = "Field `FLT3F` reader - FLT3F"]
pub use FLT1F_R as FLT3F_R;
#[doc = "Field `FLT4F` reader - FLT4F"]
pub use FLT1F_R as FLT4F_R;
#[doc = "Field `FLT2F` writer - FLT2F"]
pub use FLT1F_W as FLT2F_W;
#[doc = "Field `FLT3F` writer - FLT3F"]
pub use FLT1F_W as FLT3F_W;
#[doc = "Field `FLT4F` writer - FLT4F"]
pub use FLT1F_W as FLT4F_W;
#[doc = "Field `FLT2LCK` reader - FLT2LCK"]
pub use FLT1LCK_R as FLT2LCK_R;
#[doc = "Field `FLT3LCK` reader - FLT3LCK"]
pub use FLT1LCK_R as FLT3LCK_R;
#[doc = "Field `FLT4LCK` reader - FLT4LCK"]
pub use FLT1LCK_R as FLT4LCK_R;
#[doc = "Field `FLT2LCK` writer - FLT2LCK"]
pub use FLT1LCK_W as FLT2LCK_W;
#[doc = "Field `FLT3LCK` writer - FLT3LCK"]
pub use FLT1LCK_W as FLT3LCK_W;
#[doc = "Field `FLT4LCK` writer - FLT4LCK"]
pub use FLT1LCK_W as FLT4LCK_W;
#[doc = "Field `FLT2P` reader - FLT2P"]
pub use FLT1P_R as FLT2P_R;
#[doc = "Field `FLT3P` reader - FLT3P"]
pub use FLT1P_R as FLT3P_R;
#[doc = "Field `FLT4P` reader - FLT4P"]
pub use FLT1P_R as FLT4P_R;
#[doc = "Field `FLT2P` writer - FLT2P"]
pub use FLT1P_W as FLT2P_W;
#[doc = "Field `FLT3P` writer - FLT3P"]
pub use FLT1P_W as FLT3P_W;
#[doc = "Field `FLT4P` writer - FLT4P"]
pub use FLT1P_W as FLT4P_W;
#[doc = "Field `FLT2SRC` reader - FLT2SRC"]
pub use FLT1SRC_R as FLT2SRC_R;
#[doc = "Field `FLT3SRC` reader - FLT3SRC"]
pub use FLT1SRC_R as FLT3SRC_R;
#[doc = "Field `FLT4SRC` reader - FLT4SRC"]
pub use FLT1SRC_R as FLT4SRC_R;
#[doc = "Field `FLT2SRC` writer - FLT2SRC"]
pub use FLT1SRC_W as FLT2SRC_W;
#[doc = "Field `FLT3SRC` writer - FLT3SRC"]
pub use FLT1SRC_W as FLT3SRC_W;
#[doc = "Field `FLT4SRC` writer - FLT4SRC"]
pub use FLT1SRC_W as FLT4SRC_W;
impl R {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&self) -> FLT1E_R {
        FLT1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&self) -> FLT1P_R {
        FLT1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&self) -> FLT1SRC_R {
        FLT1SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&self) -> FLT1F_R {
        FLT1F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&self) -> FLT2E_R {
        FLT2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&self) -> FLT2P_R {
        FLT2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&self) -> FLT2SRC_R {
        FLT2SRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&self) -> FLT2F_R {
        FLT2F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&self) -> FLT3E_R {
        FLT3E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&self) -> FLT3P_R {
        FLT3P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&self) -> FLT3SRC_R {
        FLT3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&self) -> FLT3F_R {
        FLT3F_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&self) -> FLT4E_R {
        FLT4E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&self) -> FLT4P_R {
        FLT4P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&self) -> FLT4SRC_R {
        FLT4SRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&self) -> FLT4F_R {
        FLT4F_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    #[must_use]
    pub fn flt1e(&mut self) -> FLT1E_W<0> {
        FLT1E_W::new(self)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    #[must_use]
    pub fn flt1p(&mut self) -> FLT1P_W<1> {
        FLT1P_W::new(self)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src(&mut self) -> FLT1SRC_W<2> {
        FLT1SRC_W::new(self)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    #[must_use]
    pub fn flt1f(&mut self) -> FLT1F_W<3> {
        FLT1F_W::new(self)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt1lck(&mut self) -> FLT1LCK_W<7> {
        FLT1LCK_W::new(self)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    #[must_use]
    pub fn flt2e(&mut self) -> FLT2E_W<8> {
        FLT2E_W::new(self)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    #[must_use]
    pub fn flt2p(&mut self) -> FLT2P_W<9> {
        FLT2P_W::new(self)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src(&mut self) -> FLT2SRC_W<10> {
        FLT2SRC_W::new(self)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    #[must_use]
    pub fn flt2f(&mut self) -> FLT2F_W<11> {
        FLT2F_W::new(self)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt2lck(&mut self) -> FLT2LCK_W<15> {
        FLT2LCK_W::new(self)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    #[must_use]
    pub fn flt3e(&mut self) -> FLT3E_W<16> {
        FLT3E_W::new(self)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    #[must_use]
    pub fn flt3p(&mut self) -> FLT3P_W<17> {
        FLT3P_W::new(self)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src(&mut self) -> FLT3SRC_W<18> {
        FLT3SRC_W::new(self)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    #[must_use]
    pub fn flt3f(&mut self) -> FLT3F_W<19> {
        FLT3F_W::new(self)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt3lck(&mut self) -> FLT3LCK_W<23> {
        FLT3LCK_W::new(self)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    #[must_use]
    pub fn flt4e(&mut self) -> FLT4E_W<24> {
        FLT4E_W::new(self)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    #[must_use]
    pub fn flt4p(&mut self) -> FLT4P_W<25> {
        FLT4P_W::new(self)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src(&mut self) -> FLT4SRC_W<26> {
        FLT4SRC_W::new(self)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    #[must_use]
    pub fn flt4f(&mut self) -> FLT4F_W<27> {
        FLT4F_W::new(self)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt4lck(&mut self) -> FLT4LCK_W<31> {
        FLT4LCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr1](index.html) module"]
pub struct FLTINR1_SPEC;
impl crate::RegisterSpec for FLTINR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr1::R](R) reader structure"]
impl crate::Readable for FLTINR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr1::W](W) writer structure"]
impl crate::Writable for FLTINR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINR1 to value 0"]
impl crate::Resettable for FLTINR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
