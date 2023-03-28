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
#[doc = "Field `RNGEN` reader - True random number generator enable"]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
#[doc = "True random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    #[doc = "0: Random number generator is disabled"]
    Disabled = 0,
    #[doc = "1: Random number generator is enabled"]
    Enabled = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::Disabled,
            true => RNGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN_A::Enabled
    }
}
#[doc = "Field `RNGEN` writer - True random number generator enable"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RNGEN_A, O>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Disabled)
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Enabled)
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<IE_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE_A {
    #[doc = "0: RNG interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: RNG interrupt is enabled"]
    Enabled = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::Disabled,
            true => IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE_A::Enabled
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, IE_A, O>;
impl<'a, const O: u8> IE_W<'a, O> {
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IE_A::Disabled)
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IE_A::Enabled)
    }
}
#[doc = "Field `CED` reader - Interrupt Enable"]
pub type CED_R = crate::BitReader<CED_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CED_A {
    #[doc = "0: Clock error detection is enabled"]
    Enabled = 0,
    #[doc = "1: Clock error detection is disabled"]
    Disabled = 1,
}
impl From<CED_A> for bool {
    #[inline(always)]
    fn from(variant: CED_A) -> Self {
        variant as u8 != 0
    }
}
impl CED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CED_A {
        match self.bits {
            false => CED_A::Enabled,
            true => CED_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CED_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CED_A::Disabled
    }
}
#[doc = "Field `CED` writer - Interrupt Enable"]
pub type CED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CED_A, O>;
impl<'a, const O: u8> CED_W<'a, O> {
    #[doc = "Clock error detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CED_A::Enabled)
    }
    #[doc = "Clock error detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CED_A::Disabled)
    }
}
#[doc = "Field `RNG_CONFIG3` reader - RNG_CONFIG3"]
pub type RNG_CONFIG3_R = crate::FieldReader<u8, RNG_CONFIG3_A>;
#[doc = "RNG_CONFIG3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG3_A {
    #[doc = "0: Recommended value for config B (not NIST certifiable)"]
    ConfigB = 0,
    #[doc = "13: Recommended value for config A (NIST certifiable)"]
    ConfigA = 13,
}
impl From<RNG_CONFIG3_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG3_A) -> Self {
        variant as _
    }
}
impl RNG_CONFIG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG3_A> {
        match self.bits {
            0 => Some(RNG_CONFIG3_A::ConfigB),
            13 => Some(RNG_CONFIG3_A::ConfigA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ConfigB`"]
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG3_A::ConfigB
    }
    #[doc = "Checks if the value of the field is `ConfigA`"]
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG3_A::ConfigA
    }
}
#[doc = "Field `RNG_CONFIG3` writer - RNG_CONFIG3"]
pub type RNG_CONFIG3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CR_SPEC, u8, RNG_CONFIG3_A, 4, O>;
impl<'a, const O: u8> RNG_CONFIG3_W<'a, O> {
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn config_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG3_A::ConfigB)
    }
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn config_a(self) -> &'a mut W {
        self.variant(RNG_CONFIG3_A::ConfigA)
    }
}
#[doc = "Field `NISTC` reader - NISTC"]
pub type NISTC_R = crate::BitReader<NISTC_A>;
#[doc = "NISTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISTC_A {
    #[doc = "0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
    Default = 0,
    #[doc = "1: Custom values for NIST compliant RNG"]
    Custom = 1,
}
impl From<NISTC_A> for bool {
    #[inline(always)]
    fn from(variant: NISTC_A) -> Self {
        variant as u8 != 0
    }
}
impl NISTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NISTC_A {
        match self.bits {
            false => NISTC_A::Default,
            true => NISTC_A::Custom,
        }
    }
    #[doc = "Checks if the value of the field is `Default`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == NISTC_A::Default
    }
    #[doc = "Checks if the value of the field is `Custom`"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == NISTC_A::Custom
    }
}
#[doc = "Field `NISTC` writer - NISTC"]
pub type NISTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, NISTC_A, O>;
impl<'a, const O: u8> NISTC_W<'a, O> {
    #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(NISTC_A::Default)
    }
    #[doc = "Custom values for NIST compliant RNG"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(NISTC_A::Custom)
    }
}
#[doc = "Field `RNG_CONFIG2` reader - RNG_CONFIG2"]
pub type RNG_CONFIG2_R = crate::FieldReader<u8, RNG_CONFIG2_A>;
#[doc = "RNG_CONFIG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG2_A {
    #[doc = "0: Recommended value for config A and B"]
    ConfigAB = 0,
}
impl From<RNG_CONFIG2_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG2_A) -> Self {
        variant as _
    }
}
impl RNG_CONFIG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG2_A> {
        match self.bits {
            0 => Some(RNG_CONFIG2_A::ConfigAB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ConfigAB`"]
    #[inline(always)]
    pub fn is_config_a_b(&self) -> bool {
        *self == RNG_CONFIG2_A::ConfigAB
    }
}
#[doc = "Field `RNG_CONFIG2` writer - RNG_CONFIG2"]
pub type RNG_CONFIG2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CR_SPEC, u8, RNG_CONFIG2_A, 3, O>;
impl<'a, const O: u8> RNG_CONFIG2_W<'a, O> {
    #[doc = "Recommended value for config A and B"]
    #[inline(always)]
    pub fn config_a_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG2_A::ConfigAB)
    }
}
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<u8, CLKDIV_A>;
#[doc = "CLKDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: Internal RNG clock after divider is similar to incoming RNG clock"]
    NoDiv = 0,
    #[doc = "1: Divide RNG clock by 2^1"]
    Div21 = 1,
    #[doc = "2: Divide RNG clock by 2^2"]
    Div22 = 2,
    #[doc = "3: Divide RNG clock by 2^3"]
    Div23 = 3,
    #[doc = "4: Divide RNG clock by 2^4"]
    Div24 = 4,
    #[doc = "5: Divide RNG clock by 2^5"]
    Div25 = 5,
    #[doc = "6: Divide RNG clock by 2^6"]
    Div26 = 6,
    #[doc = "7: Divide RNG clock by 2^7"]
    Div27 = 7,
    #[doc = "8: Divide RNG clock by 2^8"]
    Div28 = 8,
    #[doc = "9: Divide RNG clock by 2^9"]
    Div29 = 9,
    #[doc = "10: Divide RNG clock by 2^10"]
    Div210 = 10,
    #[doc = "11: Divide RNG clock by 2^11"]
    Div211 = 11,
    #[doc = "12: Divide RNG clock by 2^12"]
    Div212 = 12,
    #[doc = "13: Divide RNG clock by 2^13"]
    Div213 = 13,
    #[doc = "14: Divide RNG clock by 2^14"]
    Div214 = 14,
    #[doc = "15: Divide RNG clock by 2^15"]
    Div215 = 15,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIV_A {
        match self.bits {
            0 => CLKDIV_A::NoDiv,
            1 => CLKDIV_A::Div21,
            2 => CLKDIV_A::Div22,
            3 => CLKDIV_A::Div23,
            4 => CLKDIV_A::Div24,
            5 => CLKDIV_A::Div25,
            6 => CLKDIV_A::Div26,
            7 => CLKDIV_A::Div27,
            8 => CLKDIV_A::Div28,
            9 => CLKDIV_A::Div29,
            10 => CLKDIV_A::Div210,
            11 => CLKDIV_A::Div211,
            12 => CLKDIV_A::Div212,
            13 => CLKDIV_A::Div213,
            14 => CLKDIV_A::Div214,
            15 => CLKDIV_A::Div215,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoDiv`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == CLKDIV_A::NoDiv
    }
    #[doc = "Checks if the value of the field is `Div21`"]
    #[inline(always)]
    pub fn is_div_2_1(&self) -> bool {
        *self == CLKDIV_A::Div21
    }
    #[doc = "Checks if the value of the field is `Div22`"]
    #[inline(always)]
    pub fn is_div_2_2(&self) -> bool {
        *self == CLKDIV_A::Div22
    }
    #[doc = "Checks if the value of the field is `Div23`"]
    #[inline(always)]
    pub fn is_div_2_3(&self) -> bool {
        *self == CLKDIV_A::Div23
    }
    #[doc = "Checks if the value of the field is `Div24`"]
    #[inline(always)]
    pub fn is_div_2_4(&self) -> bool {
        *self == CLKDIV_A::Div24
    }
    #[doc = "Checks if the value of the field is `Div25`"]
    #[inline(always)]
    pub fn is_div_2_5(&self) -> bool {
        *self == CLKDIV_A::Div25
    }
    #[doc = "Checks if the value of the field is `Div26`"]
    #[inline(always)]
    pub fn is_div_2_6(&self) -> bool {
        *self == CLKDIV_A::Div26
    }
    #[doc = "Checks if the value of the field is `Div27`"]
    #[inline(always)]
    pub fn is_div_2_7(&self) -> bool {
        *self == CLKDIV_A::Div27
    }
    #[doc = "Checks if the value of the field is `Div28`"]
    #[inline(always)]
    pub fn is_div_2_8(&self) -> bool {
        *self == CLKDIV_A::Div28
    }
    #[doc = "Checks if the value of the field is `Div29`"]
    #[inline(always)]
    pub fn is_div_2_9(&self) -> bool {
        *self == CLKDIV_A::Div29
    }
    #[doc = "Checks if the value of the field is `Div210`"]
    #[inline(always)]
    pub fn is_div_2_10(&self) -> bool {
        *self == CLKDIV_A::Div210
    }
    #[doc = "Checks if the value of the field is `Div211`"]
    #[inline(always)]
    pub fn is_div_2_11(&self) -> bool {
        *self == CLKDIV_A::Div211
    }
    #[doc = "Checks if the value of the field is `Div212`"]
    #[inline(always)]
    pub fn is_div_2_12(&self) -> bool {
        *self == CLKDIV_A::Div212
    }
    #[doc = "Checks if the value of the field is `Div213`"]
    #[inline(always)]
    pub fn is_div_2_13(&self) -> bool {
        *self == CLKDIV_A::Div213
    }
    #[doc = "Checks if the value of the field is `Div214`"]
    #[inline(always)]
    pub fn is_div_2_14(&self) -> bool {
        *self == CLKDIV_A::Div214
    }
    #[doc = "Checks if the value of the field is `Div215`"]
    #[inline(always)]
    pub fn is_div_2_15(&self) -> bool {
        *self == CLKDIV_A::Div215
    }
}
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, CLKDIV_A, 4, O>;
impl<'a, const O: u8> CLKDIV_W<'a, O> {
    #[doc = "Internal RNG clock after divider is similar to incoming RNG clock"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(CLKDIV_A::NoDiv)
    }
    #[doc = "Divide RNG clock by 2^1"]
    #[inline(always)]
    pub fn div_2_1(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div21)
    }
    #[doc = "Divide RNG clock by 2^2"]
    #[inline(always)]
    pub fn div_2_2(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div22)
    }
    #[doc = "Divide RNG clock by 2^3"]
    #[inline(always)]
    pub fn div_2_3(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div23)
    }
    #[doc = "Divide RNG clock by 2^4"]
    #[inline(always)]
    pub fn div_2_4(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div24)
    }
    #[doc = "Divide RNG clock by 2^5"]
    #[inline(always)]
    pub fn div_2_5(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div25)
    }
    #[doc = "Divide RNG clock by 2^6"]
    #[inline(always)]
    pub fn div_2_6(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div26)
    }
    #[doc = "Divide RNG clock by 2^7"]
    #[inline(always)]
    pub fn div_2_7(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div27)
    }
    #[doc = "Divide RNG clock by 2^8"]
    #[inline(always)]
    pub fn div_2_8(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div28)
    }
    #[doc = "Divide RNG clock by 2^9"]
    #[inline(always)]
    pub fn div_2_9(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div29)
    }
    #[doc = "Divide RNG clock by 2^10"]
    #[inline(always)]
    pub fn div_2_10(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div210)
    }
    #[doc = "Divide RNG clock by 2^11"]
    #[inline(always)]
    pub fn div_2_11(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div211)
    }
    #[doc = "Divide RNG clock by 2^12"]
    #[inline(always)]
    pub fn div_2_12(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div212)
    }
    #[doc = "Divide RNG clock by 2^13"]
    #[inline(always)]
    pub fn div_2_13(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div213)
    }
    #[doc = "Divide RNG clock by 2^14"]
    #[inline(always)]
    pub fn div_2_14(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div214)
    }
    #[doc = "Divide RNG clock by 2^15"]
    #[inline(always)]
    pub fn div_2_15(self) -> &'a mut W {
        self.variant(CLKDIV_A::Div215)
    }
}
#[doc = "Field `RNG_CONFIG1` reader - RNG_CONFIG1"]
pub type RNG_CONFIG1_R = crate::FieldReader<u8, RNG_CONFIG1_A>;
#[doc = "RNG_CONFIG1\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNG_CONFIG1_A {
    #[doc = "15: Recommended value for config A (NIST certifiable)"]
    ConfigA = 15,
    #[doc = "24: Recommended value for config B (not NIST certifiable)"]
    ConfigB = 24,
}
impl From<RNG_CONFIG1_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG1_A) -> Self {
        variant as _
    }
}
impl RNG_CONFIG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG1_A> {
        match self.bits {
            15 => Some(RNG_CONFIG1_A::ConfigA),
            24 => Some(RNG_CONFIG1_A::ConfigB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ConfigA`"]
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        *self == RNG_CONFIG1_A::ConfigA
    }
    #[doc = "Checks if the value of the field is `ConfigB`"]
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        *self == RNG_CONFIG1_A::ConfigB
    }
}
#[doc = "Field `RNG_CONFIG1` writer - RNG_CONFIG1"]
pub type RNG_CONFIG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CR_SPEC, u8, RNG_CONFIG1_A, 6, O>;
impl<'a, const O: u8> RNG_CONFIG1_W<'a, O> {
    #[doc = "Recommended value for config A (NIST certifiable)"]
    #[inline(always)]
    pub fn config_a(self) -> &'a mut W {
        self.variant(RNG_CONFIG1_A::ConfigA)
    }
    #[doc = "Recommended value for config B (not NIST certifiable)"]
    #[inline(always)]
    pub fn config_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG1_A::ConfigB)
    }
}
#[doc = "Field `CONDRST` reader - Conditioning soft reset"]
pub type CONDRST_R = crate::BitReader<bool>;
#[doc = "Field `CONDRST` writer - Conditioning soft reset"]
pub type CONDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CONFIGLOCK` reader - CONFIGLOCK"]
pub type CONFIGLOCK_R = crate::BitReader<CONFIGLOCK_A>;
#[doc = "CONFIGLOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFIGLOCK_A {
    #[doc = "0: Writes to the RNG_CR configuration bits \\[29:4\\]
are allowed"]
    Enabled = 0,
    #[doc = "1: Writes to the RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset"]
    Disabled = 1,
}
impl From<CONFIGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: CONFIGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl CONFIGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFIGLOCK_A {
        match self.bits {
            false => CONFIGLOCK_A::Enabled,
            true => CONFIGLOCK_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONFIGLOCK_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONFIGLOCK_A::Disabled
    }
}
#[doc = "Field `CONFIGLOCK` writer - CONFIGLOCK"]
pub type CONFIGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CONFIGLOCK_A, O>;
impl<'a, const O: u8> CONFIGLOCK_W<'a, O> {
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are allowed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONFIGLOCK_A::Enabled)
    }
    #[doc = "Writes to the RNG_CR configuration bits \\[29:4\\]
are ignored until the next RNG reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONFIGLOCK_A::Disabled)
    }
}
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable"]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RNG_CONFIG3"]
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - NISTC"]
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RNG_CONFIG2"]
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - RNG_CONFIG1"]
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CONFIGLOCK"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<2> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<3> {
        IE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<5> {
        CED_W::new(self)
    }
    #[doc = "Bits 8:11 - RNG_CONFIG3"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<8> {
        RNG_CONFIG3_W::new(self)
    }
    #[doc = "Bit 12 - NISTC"]
    #[inline(always)]
    #[must_use]
    pub fn nistc(&mut self) -> NISTC_W<12> {
        NISTC_W::new(self)
    }
    #[doc = "Bits 13:15 - RNG_CONFIG2"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<13> {
        RNG_CONFIG2_W::new(self)
    }
    #[doc = "Bits 16:19 - CLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 20:25 - RNG_CONFIG1"]
    #[inline(always)]
    #[must_use]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<20> {
        RNG_CONFIG1_W::new(self)
    }
    #[doc = "Bit 30 - Conditioning soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn condrst(&mut self) -> CONDRST_W<30> {
        CONDRST_W::new(self)
    }
    #[doc = "Bit 31 - CONFIGLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<31> {
        CONFIGLOCK_W::new(self)
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
#[doc = "`reset()` method sets CR to value 0x0080_0000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
