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
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt"]
pub type ADRDYIE_R = crate::BitReader<ADRDYIE_A>;
#[doc = "ADC ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE_A {
    #[doc = "0: ADRDY interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
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
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt"]
pub type ADRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ADRDYIE_A, O>;
impl<'a, const O: u8> ADRDYIE_W<'a, O> {
    #[doc = "ADRDY interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Disabled)
    }
    #[doc = "ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Enabled)
    }
}
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
pub type EOSMPIE_R = crate::BitReader<EOSMPIE_A>;
#[doc = "ADC group regular end of sampling interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE_A {
    #[doc = "0: EOSMP interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
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
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub type EOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSMPIE_A, O>;
impl<'a, const O: u8> EOSMPIE_W<'a, O> {
    #[doc = "EOSMP interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Disabled)
    }
    #[doc = "EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Enabled)
    }
}
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
#[doc = "ADC group regular end of unitary conversion interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: EOC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
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
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOCIE_A, O>;
impl<'a, const O: u8> EOCIE_W<'a, O> {
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Disabled)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Enabled)
    }
}
#[doc = "Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_R = crate::BitReader<EOSIE_A>;
#[doc = "ADC group regular end of sequence conversions interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE_A {
    #[doc = "0: EOS interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
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
#[doc = "Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt"]
pub type EOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSIE_A, O>;
impl<'a, const O: u8> EOSIE_W<'a, O> {
    #[doc = "EOS interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSIE_A::Disabled)
    }
    #[doc = "EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSIE_A::Enabled)
    }
}
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
#[doc = "ADC group regular overrun interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    #[doc = "0: Overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
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
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVRIE_A, O>;
impl<'a, const O: u8> OVRIE_W<'a, O> {
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Disabled)
    }
    #[doc = "Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Enabled)
    }
}
#[doc = "Field `AWD1IE` reader - ADC analog watchdog 1 interrupt"]
pub type AWD1IE_R = crate::BitReader<AWD1IE_A>;
#[doc = "ADC analog watchdog 1 interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `AWD1IE` writer - ADC analog watchdog 1 interrupt"]
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
#[doc = "Field `AWD2IE` reader - ADC analog watchdog 2 interrupt"]
pub use AWD1IE_R as AWD2IE_R;
#[doc = "Field `AWD3IE` reader - ADC analog watchdog 3 interrupt"]
pub use AWD1IE_R as AWD3IE_R;
#[doc = "Field `AWD2IE` writer - ADC analog watchdog 2 interrupt"]
pub use AWD1IE_W as AWD2IE_W;
#[doc = "Field `AWD3IE` writer - ADC analog watchdog 3 interrupt"]
pub use AWD1IE_W as AWD3IE_W;
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable"]
pub type EOCALIE_R = crate::BitReader<EOCALIE_A>;
#[doc = "End of calibration interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALIE_A {
    #[doc = "0: End of calibration interrupt disabled"]
    Disabled = 0,
    #[doc = "1: End of calibration interrupt enabled"]
    Enabled = 1,
}
impl From<EOCALIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCALIE_A {
        match self.bits {
            false => EOCALIE_A::Disabled,
            true => EOCALIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCALIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCALIE_A::Enabled
    }
}
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable"]
pub type EOCALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOCALIE_A, O>;
impl<'a, const O: u8> EOCALIE_W<'a, O> {
    #[doc = "End of calibration interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCALIE_A::Disabled)
    }
    #[doc = "End of calibration interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCALIE_A::Enabled)
    }
}
#[doc = "Field `CCRDYIE` reader - Channel Configuration Ready Interrupt enable"]
pub type CCRDYIE_R = crate::BitReader<CCRDYIE_A>;
#[doc = "Channel Configuration Ready Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDYIE_A {
    #[doc = "0: Channel configuration ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Channel configuration ready interrupt enabled"]
    Enabled = 1,
}
impl From<CCRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRDYIE_A {
        match self.bits {
            false => CCRDYIE_A::Disabled,
            true => CCRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCRDYIE_A::Enabled
    }
}
#[doc = "Field `CCRDYIE` writer - Channel Configuration Ready Interrupt enable"]
pub type CCRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CCRDYIE_A, O>;
impl<'a, const O: u8> CCRDYIE_W<'a, O> {
    #[doc = "Channel configuration ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCRDYIE_A::Disabled)
    }
    #[doc = "Channel configuration ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCRDYIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable"]
    #[inline(always)]
    pub fn ccrdyie(&self) -> CCRDYIE_R {
        CCRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<0> {
        ADRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<1> {
        EOSMPIE_W::new(self)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<2> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<3> {
        EOSIE_W::new(self)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<4> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<7> {
        AWD1IE_W::new(self)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<8> {
        AWD2IE_W::new(self)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<9> {
        AWD3IE_W::new(self)
    }
    #[doc = "Bit 11 - End of calibration interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EOCALIE_W<11> {
        EOCALIE_W::new(self)
    }
    #[doc = "Bit 13 - Channel Configuration Ready Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccrdyie(&mut self) -> CCRDYIE_W<13> {
        CCRDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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
