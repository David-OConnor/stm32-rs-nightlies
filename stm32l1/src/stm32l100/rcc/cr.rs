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
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HSION_R = crate::BitReader<HSION_A>;
#[doc = "Internal high-speed clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::Disabled,
            true => HSION_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION_A::Enabled
    }
}
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSION_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSION_A::Enabled)
    }
}
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub type HSIRDY_R = crate::BitReader<HSIRDYR_A>;
#[doc = "Internal high-speed clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR_A {
    #[doc = "0: Oscillator is not stable"]
    NotReady = 0,
    #[doc = "1: Oscillator is stable"]
    Ready = 1,
}
impl From<HSIRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYR_A {
        match self.bits {
            false => HSIRDYR_A::NotReady,
            true => HSIRDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR_A::Ready
    }
}
#[doc = "Field `MSION` reader - MSI clock enable"]
pub use HSION_R as MSION_R;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub use HSION_R as HSEON_R;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub use HSION_W as MSION_W;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub use HSION_W as HSEON_W;
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub use HSIRDY_R as MSIRDY_R;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub use HSIRDY_R as HSERDY_R;
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
#[doc = "HSE clock bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HSE oscillator bypassed"]
    Bypassed = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NotBypassed,
            true => HSEBYP_A::Bypassed,
        }
    }
    #[doc = "Checks if the value of the field is `NotBypassed`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NotBypassed
    }
    #[doc = "Checks if the value of the field is `Bypassed`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::Bypassed
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYP_A, O>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    #[doc = "HSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NotBypassed)
    }
    #[doc = "HSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::Bypassed)
    }
}
#[doc = "Field `PLLON` reader - PLL enable"]
pub use HSION_R as PLLON_R;
#[doc = "Field `PLLON` writer - PLL enable"]
pub use HSION_W as PLLON_W;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader<PLLRDYR_A>;
#[doc = "PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYR_A {
    #[doc = "0: PLL unlocked"]
    Unlocked = 0,
    #[doc = "1: PLL locked"]
    Locked = 1,
}
impl From<PLLRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYR_A {
        match self.bits {
            false => PLLRDYR_A::Unlocked,
            true => PLLRDYR_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDYR_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDYR_A::Locked
    }
}
#[doc = "Field `CSSON` reader - Clock security system enable"]
pub use HSION_R as CSSON_R;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub use HSION_W as CSSON_W;
#[doc = "Field `RTCPRE` reader - TC/LCD prescaler"]
pub type RTCPRE_R = crate::FieldReader<u8, RTCPRE_A>;
#[doc = "TC/LCD prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPRE_A {
    #[doc = "0: HSE divided by 2"]
    Div2 = 0,
    #[doc = "1: HSE divided by 4"]
    Div4 = 1,
    #[doc = "2: HSE divided by 8"]
    Div8 = 2,
    #[doc = "3: HSE divided by 16"]
    Div16 = 3,
}
impl From<RTCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE_A) -> Self {
        variant as _
    }
}
impl RTCPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPRE_A {
        match self.bits {
            0 => RTCPRE_A::Div2,
            1 => RTCPRE_A::Div4,
            2 => RTCPRE_A::Div8,
            3 => RTCPRE_A::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTCPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTCPRE_A::Div16
    }
}
#[doc = "Field `RTCPRE` writer - TC/LCD prescaler"]
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, RTCPRE_A, 2, O>;
impl<'a, const O: u8> RTCPRE_W<'a, O> {
    #[doc = "HSE divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div2)
    }
    #[doc = "HSE divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div4)
    }
    #[doc = "HSE divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div8)
    }
    #[doc = "HSE divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div16)
    }
}
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - TC/LCD prescaler"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<0> {
        HSION_W::new(self)
    }
    #[doc = "Bit 8 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<8> {
        MSION_W::new(self)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    #[doc = "Bit 28 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<28> {
        CSSON_W::new(self)
    }
    #[doc = "Bits 29:30 - TC/LCD prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<29> {
        RTCPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
