#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub type JEOCIE_R = crate::BitReader<JEOCIE_A>;
#[doc = "Injected end of conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE_A {
    #[doc = "0: Injected end of conversion interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Injected end of conversion interrupt is enabled"]
    Enabled = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::Disabled,
            true => JEOCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE_A::Enabled
    }
}
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JEOCIE_A, O>;
impl<'a, const O: u8> JEOCIE_W<'a, O> {
    #[doc = "Injected end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Disabled)
    }
    #[doc = "Injected end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Enabled)
    }
}
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub type REOCIE_R = crate::BitReader<REOCIE_A>;
#[doc = "Regular end of conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REOCIE_A {
    #[doc = "0: Regular end of conversion interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Regular end of conversion interrupt is enabled"]
    Enabled = 1,
}
impl From<REOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: REOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REOCIE_A {
        match self.bits {
            false => REOCIE_A::Disabled,
            true => REOCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REOCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REOCIE_A::Enabled
    }
}
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub type REOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, REOCIE_A, O>;
impl<'a, const O: u8> REOCIE_W<'a, O> {
    #[doc = "Regular end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REOCIE_A::Disabled)
    }
    #[doc = "Regular end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REOCIE_A::Enabled)
    }
}
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub type JOVRIE_R = crate::BitReader<JOVRIE_A>;
#[doc = "Injected data overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVRIE_A {
    #[doc = "0: Injected data overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Injected data overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<JOVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JOVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVRIE_A {
        match self.bits {
            false => JOVRIE_A::Disabled,
            true => JOVRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVRIE_A::Enabled
    }
}
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub type JOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JOVRIE_A, O>;
impl<'a, const O: u8> JOVRIE_W<'a, O> {
    #[doc = "Injected data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVRIE_A::Disabled)
    }
    #[doc = "Injected data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVRIE_A::Enabled)
    }
}
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub type ROVRIE_R = crate::BitReader<ROVRIE_A>;
#[doc = "Regular data overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVRIE_A {
    #[doc = "0: Regular data overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Regular data overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<ROVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVRIE_A {
        match self.bits {
            false => ROVRIE_A::Disabled,
            true => ROVRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVRIE_A::Enabled
    }
}
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub type ROVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ROVRIE_A, O>;
impl<'a, const O: u8> ROVRIE_W<'a, O> {
    #[doc = "Regular data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVRIE_A::Disabled)
    }
    #[doc = "Regular data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVRIE_A::Enabled)
    }
}
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader<AWDIE_A>;
#[doc = "Analog watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE_A {
    #[doc = "0: Analog watchdog interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt is enabled"]
    Enabled = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::Disabled,
            true => AWDIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE_A::Enabled
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, AWDIE_A, O>;
impl<'a, const O: u8> AWDIE_W<'a, O> {
    #[doc = "Analog watchdog interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Disabled)
    }
    #[doc = "Analog watchdog interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Enabled)
    }
}
#[doc = "Field `SCDIE` reader - Short-circuit detector interrupt enable"]
pub type SCDIE_R = crate::BitReader<SCDIE_A>;
#[doc = "Short-circuit detector interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDIE_A {
    #[doc = "0: Short-circuit detector interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Short-circuit detector interrupt is enabled"]
    Enabled = 1,
}
impl From<SCDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SCDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDIE_A {
        match self.bits {
            false => SCDIE_A::Disabled,
            true => SCDIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDIE_A::Enabled
    }
}
#[doc = "Field `SCDIE` writer - Short-circuit detector interrupt enable"]
pub type SCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SCDIE_A, O>;
impl<'a, const O: u8> SCDIE_W<'a, O> {
    #[doc = "Short-circuit detector interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCDIE_A::Disabled)
    }
    #[doc = "Short-circuit detector interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCDIE_A::Enabled)
    }
}
#[doc = "Field `CKABIE` reader - Clock absence interrupt enable"]
pub type CKABIE_R = crate::BitReader<CKABIE_A>;
#[doc = "Clock absence interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABIE_A {
    #[doc = "0: Detection of channel input clock absence interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Detection of channel input clock absence interrupt is enabled"]
    Enabled = 1,
}
impl From<CKABIE_A> for bool {
    #[inline(always)]
    fn from(variant: CKABIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CKABIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKABIE_A {
        match self.bits {
            false => CKABIE_A::Disabled,
            true => CKABIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABIE_A::Enabled
    }
}
#[doc = "Field `CKABIE` writer - Clock absence interrupt enable"]
pub type CKABIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CKABIE_A, O>;
impl<'a, const O: u8> CKABIE_W<'a, O> {
    #[doc = "Detection of channel input clock absence interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CKABIE_A::Disabled)
    }
    #[doc = "Detection of channel input clock absence interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CKABIE_A::Enabled)
    }
}
#[doc = "Field `EXCH` reader - Extremes detector channel selection"]
pub type EXCH_R = crate::FieldReader<u8, EXCH_A>;
#[doc = "Extremes detector channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXCH_A {
    #[doc = "0: Extremes detector does not accept data from channel y"]
    Disabled = 0,
    #[doc = "1: Extremes detector accepts data from channel y"]
    Enabled = 1,
}
impl From<EXCH_A> for u8 {
    #[inline(always)]
    fn from(variant: EXCH_A) -> Self {
        variant as _
    }
}
impl EXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXCH_A> {
        match self.bits {
            0 => Some(EXCH_A::Disabled),
            1 => Some(EXCH_A::Enabled),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXCH_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXCH_A::Enabled
    }
}
#[doc = "Field `EXCH` writer - Extremes detector channel selection"]
pub type EXCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, EXCH_A, 8, O>;
impl<'a, const O: u8> EXCH_W<'a, O> {
    #[doc = "Extremes detector does not accept data from channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXCH_A::Disabled)
    }
    #[doc = "Extremes detector accepts data from channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXCH_A::Enabled)
    }
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AWDCH_R = crate::FieldReader<u8, AWDCH_A>;
#[doc = "Analog watchdog channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWDCH_A {
    #[doc = "0: Analog watchdog is disabled on channel y"]
    Disabled = 0,
    #[doc = "1: Analog watchdog is enabled on channel y"]
    Enabled = 1,
}
impl From<AWDCH_A> for u8 {
    #[inline(always)]
    fn from(variant: AWDCH_A) -> Self {
        variant as _
    }
}
impl AWDCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AWDCH_A> {
        match self.bits {
            0 => Some(AWDCH_A::Disabled),
            1 => Some(AWDCH_A::Enabled),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDCH_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDCH_A::Enabled
    }
}
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AWDCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, AWDCH_A, 8, O>;
impl<'a, const O: u8> AWDCH_W<'a, O> {
    #[doc = "Analog watchdog is disabled on channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDCH_A::Disabled)
    }
    #[doc = "Analog watchdog is enabled on channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDCH_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<0> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<1> {
        REOCIE_W::new(self)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<2> {
        JOVRIE_W::new(self)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<3> {
        ROVRIE_W::new(self)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<4> {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<5> {
        SCDIE_W::new(self)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<6> {
        CKABIE_W::new(self)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn exch(&mut self) -> EXCH_W<8> {
        EXCH_W::new(self)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<16> {
        AWDCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
