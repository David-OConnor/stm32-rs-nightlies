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
#[doc = "Field `FWDIS` reader - Firewall disable"]
pub type FWDIS_R = crate::BitReader<FWDIS_A>;
#[doc = "Firewall disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDIS_A {
    #[doc = "0: Firewall protection enabled"]
    Enabled = 0,
    #[doc = "1: Firewall protection disabled"]
    Disabled = 1,
}
impl From<FWDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FWDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FWDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDIS_A {
        match self.bits {
            false => FWDIS_A::Enabled,
            true => FWDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWDIS_A::Disabled
    }
}
#[doc = "Field `FWDIS` writer - Firewall disable"]
pub type FWDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FWDIS_A, O>;
impl<'a, const O: u8> FWDIS_W<'a, O> {
    #[doc = "Firewall protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWDIS_A::Enabled)
    }
    #[doc = "Firewall protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWDIS_A::Disabled)
    }
}
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub type BOOSTEN_R = crate::BitReader<BOOSTEN_A>;
#[doc = "I/O analog switch voltage booster enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOSTEN_A {
    #[doc = "0: I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation"]
    Disabled = 0,
    #[doc = "1: I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation"]
    Enabled = 1,
}
impl From<BOOSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOSTEN_A {
        match self.bits {
            false => BOOSTEN_A::Disabled,
            true => BOOSTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOOSTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOOSTEN_A::Enabled
    }
}
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub type BOOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, BOOSTEN_A, O>;
impl<'a, const O: u8> BOOSTEN_W<'a, O> {
    #[doc = "I/O analog switches are supplied by VDDA voltage. This is the recommended configuration when using the ADC in high VDDA voltage operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOOSTEN_A::Disabled)
    }
    #[doc = "I/O analog switches are supplied by a dedicated voltage booster (supplied by VDD). This is the recommended configuration when using the ADC in low VDDA voltage operation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOOSTEN_A::Enabled)
    }
}
#[doc = "Field `ANASWVDD` reader - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)"]
pub type ANASWVDD_R = crate::BitReader<ANASWVDD_A>;
#[doc = "GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANASWVDD_A {
    #[doc = "0: I/O analog switches supplied by VDDA or booster when booster is ON"]
    Vdda = 0,
    #[doc = "1: I/O analog switches supplied by VDD"]
    Vdd = 1,
}
impl From<ANASWVDD_A> for bool {
    #[inline(always)]
    fn from(variant: ANASWVDD_A) -> Self {
        variant as u8 != 0
    }
}
impl ANASWVDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANASWVDD_A {
        match self.bits {
            false => ANASWVDD_A::Vdda,
            true => ANASWVDD_A::Vdd,
        }
    }
    #[doc = "Checks if the value of the field is `Vdda`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == ANASWVDD_A::Vdda
    }
    #[doc = "Checks if the value of the field is `Vdd`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == ANASWVDD_A::Vdd
    }
}
#[doc = "Field `ANASWVDD` writer - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)"]
pub type ANASWVDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, ANASWVDD_A, O>;
impl<'a, const O: u8> ANASWVDD_W<'a, O> {
    #[doc = "I/O analog switches supplied by VDDA or booster when booster is ON"]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(ANASWVDD_A::Vdda)
    }
    #[doc = "I/O analog switches supplied by VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(ANASWVDD_A::Vdd)
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP_A>;
#[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP_A {
    #[doc = "0: PBx pin operates in standard mode"]
    Disabled = 0,
    #[doc = "1: Fm+ mode enabled on PB7 pin, and the Speed control is bypassed"]
    Enabled = 1,
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
            false => I2C_PB6_FMP_A::Disabled,
            true => I2C_PB6_FMP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C_PB6_FMP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C_PB6_FMP_A::Enabled
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB6"]
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB6_FMP_A, O>;
impl<'a, const O: u8> I2C_PB6_FMP_W<'a, O> {
    #[doc = "PBx pin operates in standard mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Disabled)
    }
    #[doc = "Fm+ mode enabled on PB7 pin, and the Speed control is bypassed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Enabled)
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub use I2C_PB6_FMP_R as I2C_PB7_FMP_R;
#[doc = "Field `I2C_PB8_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub use I2C_PB6_FMP_R as I2C_PB8_FMP_R;
#[doc = "Field `I2C_PB9_FMP` reader - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub use I2C_PB6_FMP_R as I2C_PB9_FMP_R;
#[doc = "Field `I2C_PB7_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB7"]
pub use I2C_PB6_FMP_W as I2C_PB7_FMP_W;
#[doc = "Field `I2C_PB8_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB8"]
pub use I2C_PB6_FMP_W as I2C_PB8_FMP_W;
#[doc = "Field `I2C_PB9_FMP` writer - Fast-mode Plus (Fm+) driving capability activation on PB9"]
pub use I2C_PB6_FMP_W as I2C_PB9_FMP_W;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP_A>;
#[doc = "I2C1 Fast-mode Plus driving capability activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP_A {
    #[doc = "0: Fm+ mode is not enabled on I2Cx pins selected through AF selection bits"]
    Disabled = 0,
    #[doc = "1: Fm+ mode is enabled on I2Cx pins selected through AF selection bits"]
    Enabled = 1,
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
            false => I2C1_FMP_A::Disabled,
            true => I2C1_FMP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1_FMP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1_FMP_A::Enabled
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C1_FMP_A, O>;
impl<'a, const O: u8> I2C1_FMP_W<'a, O> {
    #[doc = "Fm+ mode is not enabled on I2Cx pins selected through AF selection bits"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Disabled)
    }
    #[doc = "Fm+ mode is enabled on I2Cx pins selected through AF selection bits"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Enabled)
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_R as I2C2_FMP_R;
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_R as I2C3_FMP_R;
#[doc = "Field `I2C4_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_R as I2C4_FMP_R;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_W as I2C2_FMP_W;
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_W as I2C3_FMP_W;
#[doc = "Field `I2C4_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub use I2C1_FMP_W as I2C4_FMP_W;
#[doc = "Field `FPU_IE0` reader - Invalid operation interrupt enable"]
pub type FPU_IE0_R = crate::BitReader<FPU_IE0_A>;
#[doc = "Invalid operation interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<FPU_IE0_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE0_A {
        match self.bits {
            false => FPU_IE0_A::Disabled,
            true => FPU_IE0_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0_A::Enabled
    }
}
#[doc = "Field `FPU_IE0` writer - Invalid operation interrupt enable"]
pub type FPU_IE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE0_A, O>;
impl<'a, const O: u8> FPU_IE0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Enabled)
    }
}
#[doc = "Field `FPU_IE1` reader - Divide-by-zero interrupt enable"]
pub use FPU_IE0_R as FPU_IE1_R;
#[doc = "Field `FPU_IE2` reader - Underflow interrupt enable"]
pub use FPU_IE0_R as FPU_IE2_R;
#[doc = "Field `FPU_IE3` reader - Overflow interrupt enable"]
pub use FPU_IE0_R as FPU_IE3_R;
#[doc = "Field `FPU_IE4` reader - Input denormal interrupt enable"]
pub use FPU_IE0_R as FPU_IE4_R;
#[doc = "Field `FPU_IE5` reader - Inexact interrupt enable"]
pub use FPU_IE0_R as FPU_IE5_R;
#[doc = "Field `FPU_IE1` writer - Divide-by-zero interrupt enable"]
pub use FPU_IE0_W as FPU_IE1_W;
#[doc = "Field `FPU_IE2` writer - Underflow interrupt enable"]
pub use FPU_IE0_W as FPU_IE2_W;
#[doc = "Field `FPU_IE3` writer - Overflow interrupt enable"]
pub use FPU_IE0_W as FPU_IE3_W;
#[doc = "Field `FPU_IE4` writer - Input denormal interrupt enable"]
pub use FPU_IE0_W as FPU_IE4_W;
#[doc = "Field `FPU_IE5` writer - Inexact interrupt enable"]
pub use FPU_IE0_W as FPU_IE5_W;
impl R {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)"]
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Divide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable"]
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<0> {
        FWDIS_W::new(self)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<8> {
        BOOSTEN_W::new(self)
    }
    #[doc = "Bit 9 - GPIO analog switch control voltage selection when at least one analog peripheral supplied by VDDA is enabled (COMP, OPAMP, VREFBUF, ADC,...)"]
    #[inline(always)]
    #[must_use]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<9> {
        ANASWVDD_W::new(self)
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<16> {
        I2C_PB6_FMP_W::new(self)
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<17> {
        I2C_PB7_FMP_W::new(self)
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<18> {
        I2C_PB8_FMP_W::new(self)
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<19> {
        I2C_PB9_FMP_W::new(self)
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<20> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<21> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<22> {
        I2C3_FMP_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<23> {
        I2C4_FMP_W::new(self)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<26> {
        FPU_IE0_W::new(self)
    }
    #[doc = "Bit 27 - Divide-by-zero interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<27> {
        FPU_IE1_W::new(self)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<28> {
        FPU_IE2_W::new(self)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<29> {
        FPU_IE3_W::new(self)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<30> {
        FPU_IE4_W::new(self)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<31> {
        FPU_IE5_W::new(self)
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
#[doc = "`reset()` method sets CFGR1 to value 0x7c00_0001"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x7c00_0001;
}
