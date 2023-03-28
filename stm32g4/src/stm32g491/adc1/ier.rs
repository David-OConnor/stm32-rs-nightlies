#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt enable"]
pub type ADRDYIE_R = crate::BitReader<ADRDYIE_A>;
#[doc = "ADC ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE_A {
    #[doc = "0: ADC ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ADC ready interrupt enabled"]
    Enabled = 1,
}
impl From<ADRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDYIE_A {
        match self.bits {
            false => ADRDYIE_A::Disabled,
            true => ADRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRDYIE_A::Enabled
    }
}
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt enable"]
pub type ADRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ADRDYIE_A, O>;
impl<'a, const O: u8> ADRDYIE_W<'a, O> {
    #[doc = "ADC ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Disabled)
    }
    #[doc = "ADC ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Enabled)
    }
}
#[doc = "Field `EOSMPIE` reader - End of sampling flag interrupt enable for regular conversions"]
pub type EOSMPIE_R = crate::BitReader<EOSMPIE_A>;
#[doc = "End of sampling flag interrupt enable for regular conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE_A {
    #[doc = "0: End of regular conversion sampling phase interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular conversion sampling phase interrupt enabled"]
    Enabled = 1,
}
impl From<EOSMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMPIE_A {
        match self.bits {
            false => EOSMPIE_A::Disabled,
            true => EOSMPIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE_A::Enabled
    }
}
#[doc = "Field `EOSMPIE` writer - End of sampling flag interrupt enable for regular conversions"]
pub type EOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSMPIE_A, O>;
impl<'a, const O: u8> EOSMPIE_W<'a, O> {
    #[doc = "End of regular conversion sampling phase interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Disabled)
    }
    #[doc = "End of regular conversion sampling phase interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Enabled)
    }
}
#[doc = "Field `EOCIE` reader - End of regular conversion interrupt enable"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
#[doc = "End of regular conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: End of regular conversion interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular conversion interrupt enabled"]
    Enabled = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::Disabled,
            true => EOCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::Enabled
    }
}
#[doc = "Field `EOCIE` writer - End of regular conversion interrupt enable"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOCIE_A, O>;
impl<'a, const O: u8> EOCIE_W<'a, O> {
    #[doc = "End of regular conversion interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Disabled)
    }
    #[doc = "End of regular conversion interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Enabled)
    }
}
#[doc = "Field `EOSIE` reader - End of regular sequence of conversions interrupt enable"]
pub type EOSIE_R = crate::BitReader<EOSIE_A>;
#[doc = "End of regular sequence of conversions interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE_A {
    #[doc = "0: End of regular sequence interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of regular sequence interrupt enabled"]
    Enabled = 1,
}
impl From<EOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSIE_A {
        match self.bits {
            false => EOSIE_A::Disabled,
            true => EOSIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSIE_A::Enabled
    }
}
#[doc = "Field `EOSIE` writer - End of regular sequence of conversions interrupt enable"]
pub type EOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSIE_A, O>;
impl<'a, const O: u8> EOSIE_W<'a, O> {
    #[doc = "End of regular sequence interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSIE_A::Disabled)
    }
    #[doc = "End of regular sequence interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSIE_A::Enabled)
    }
}
#[doc = "Field `OVRIE` reader - Overrun interrupt enable"]
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
#[doc = "Overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    #[doc = "0: Overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Overrun interrupt enabled"]
    Enabled = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::Disabled,
            true => OVRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE_A::Enabled
    }
}
#[doc = "Field `OVRIE` writer - Overrun interrupt enable"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVRIE_A, O>;
impl<'a, const O: u8> OVRIE_W<'a, O> {
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Disabled)
    }
    #[doc = "Overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Enabled)
    }
}
#[doc = "Field `JEOCIE` reader - End of injected conversion interrupt enable"]
pub type JEOCIE_R = crate::BitReader<JEOCIE_A>;
#[doc = "End of injected conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE_A {
    #[doc = "0: End of injected conversion interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of injected conversion interrupt enabled"]
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
#[doc = "Field `JEOCIE` writer - End of injected conversion interrupt enable"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, JEOCIE_A, O>;
impl<'a, const O: u8> JEOCIE_W<'a, O> {
    #[doc = "End of injected conversion interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Disabled)
    }
    #[doc = "End of injected conversion interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::Enabled)
    }
}
#[doc = "Field `JEOSIE` reader - End of injected sequence of conversions interrupt enable"]
pub type JEOSIE_R = crate::BitReader<JEOSIE_A>;
#[doc = "End of injected sequence of conversions interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSIE_A {
    #[doc = "0: End of injected sequence interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of injected sequence interrupt enabled"]
    Enabled = 1,
}
impl From<JEOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOSIE_A {
        match self.bits {
            false => JEOSIE_A::Disabled,
            true => JEOSIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOSIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOSIE_A::Enabled
    }
}
#[doc = "Field `JEOSIE` writer - End of injected sequence of conversions interrupt enable"]
pub type JEOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, JEOSIE_A, O>;
impl<'a, const O: u8> JEOSIE_W<'a, O> {
    #[doc = "End of injected sequence interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOSIE_A::Disabled)
    }
    #[doc = "End of injected sequence interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOSIE_A::Enabled)
    }
}
#[doc = "Field `AWD1IE` reader - Analog watchdog 1 interrupt enable"]
pub type AWD1IE_R = crate::BitReader<AWD1IE_A>;
#[doc = "Analog watchdog 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE_A {
    #[doc = "0: Analog watchdog interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    Enabled = 1,
}
impl From<AWD1IE_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1IE_A {
        match self.bits {
            false => AWD1IE_A::Disabled,
            true => AWD1IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1IE_A::Enabled
    }
}
#[doc = "Field `AWD1IE` writer - Analog watchdog 1 interrupt enable"]
pub type AWD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, AWD1IE_A, O>;
impl<'a, const O: u8> AWD1IE_W<'a, O> {
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1IE_A::Disabled)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1IE_A::Enabled)
    }
}
#[doc = "Field `AWD2IE` reader - Analog watchdog 2 interrupt enable"]
pub use AWD1IE_R as AWD2IE_R;
#[doc = "Field `AWD3IE` reader - Analog watchdog 3 interrupt enable"]
pub use AWD1IE_R as AWD3IE_R;
#[doc = "Field `AWD2IE` writer - Analog watchdog 2 interrupt enable"]
pub use AWD1IE_W as AWD2IE_W;
#[doc = "Field `AWD3IE` writer - Analog watchdog 3 interrupt enable"]
pub use AWD1IE_W as AWD3IE_W;
#[doc = "Field `JQOVFIE` reader - Injected context queue overflow interrupt enable"]
pub type JQOVFIE_R = crate::BitReader<JQOVFIE_A>;
#[doc = "Injected context queue overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFIE_A {
    #[doc = "0: Injected context queue overflow interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Injected context queue overflow interrupt enabled"]
    Enabled = 1,
}
impl From<JQOVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl JQOVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQOVFIE_A {
        match self.bits {
            false => JQOVFIE_A::Disabled,
            true => JQOVFIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQOVFIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQOVFIE_A::Enabled
    }
}
#[doc = "Field `JQOVFIE` writer - Injected context queue overflow interrupt enable"]
pub type JQOVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, JQOVFIE_A, O>;
impl<'a, const O: u8> JQOVFIE_W<'a, O> {
    #[doc = "Injected context queue overflow interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JQOVFIE_A::Disabled)
    }
    #[doc = "Injected context queue overflow interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JQOVFIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt enable"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<0> {
        ADRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<1> {
        EOSMPIE_W::new(self)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<2> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<3> {
        EOSIE_W::new(self)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<4> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<5> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<6> {
        JEOSIE_W::new(self)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<7> {
        AWD1IE_W::new(self)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<8> {
        AWD2IE_W::new(self)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<9> {
        AWD3IE_W::new(self)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<10> {
        JQOVFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
