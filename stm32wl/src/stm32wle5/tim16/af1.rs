#[doc = "Register `AF1` reader"]
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF1` writer"]
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader<BKINE_A>;
#[doc = "BRK BKIN input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINE_A {
    #[doc = "0: BKIN input disabled"]
    Disabled = 0,
    #[doc = "1: BKIN input enabled"]
    Enabled = 1,
}
impl From<BKINE_A> for bool {
    #[inline(always)]
    fn from(variant: BKINE_A) -> Self {
        variant as u8 != 0
    }
}
impl BKINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKINE_A {
        match self.bits {
            false => BKINE_A::Disabled,
            true => BKINE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKINE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKINE_A::Enabled
    }
}
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKINE_A, O>;
impl<'a, const O: u8> BKINE_W<'a, O> {
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKINE_A::Disabled)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKINE_A::Enabled)
    }
}
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type BKCMP1E_R = crate::BitReader<BKCMP1E_A>;
#[doc = "BRK COMP1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP1 input enabled"]
    Enabled = 1,
}
impl From<BKCMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E_A) -> Self {
        variant as u8 != 0
    }
}
impl BKCMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1E_A {
        match self.bits {
            false => BKCMP1E_A::Disabled,
            true => BKCMP1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKCMP1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKCMP1E_A::Enabled
    }
}
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type BKCMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKCMP1E_A, O>;
impl<'a, const O: u8> BKCMP1E_W<'a, O> {
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKCMP1E_A::Disabled)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKCMP1E_A::Enabled)
    }
}
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type BKCMP2E_R = crate::BitReader<BKCMP2E_A>;
#[doc = "BRK COMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP2 input enabled"]
    Enabled = 1,
}
impl From<BKCMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2E_A) -> Self {
        variant as u8 != 0
    }
}
impl BKCMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2E_A {
        match self.bits {
            false => BKCMP2E_A::Disabled,
            true => BKCMP2E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKCMP2E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKCMP2E_A::Enabled
    }
}
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type BKCMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKCMP2E_A, O>;
impl<'a, const O: u8> BKCMP2E_W<'a, O> {
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKCMP2E_A::Disabled)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKCMP2E_A::Enabled)
    }
}
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BKINP_R = crate::BitReader<BKINP_A>;
#[doc = "BRK BKIN input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINP_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKINP_A> for bool {
    #[inline(always)]
    fn from(variant: BKINP_A) -> Self {
        variant as u8 != 0
    }
}
impl BKINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKINP_A {
        match self.bits {
            false => BKINP_A::NotInverted,
            true => BKINP_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKINP_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKINP_A::Inverted
    }
}
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKINP_A, O>;
impl<'a, const O: u8> BKINP_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BKINP_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BKINP_A::Inverted)
    }
}
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type BKCMP1P_R = crate::BitReader<BKCMP1P_A>;
#[doc = "BRK COMP1 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1P_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKCMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P_A) -> Self {
        variant as u8 != 0
    }
}
impl BKCMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1P_A {
        match self.bits {
            false => BKCMP1P_A::NotInverted,
            true => BKCMP1P_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKCMP1P_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKCMP1P_A::Inverted
    }
}
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type BKCMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKCMP1P_A, O>;
impl<'a, const O: u8> BKCMP1P_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BKCMP1P_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BKCMP1P_A::Inverted)
    }
}
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type BKCMP2P_R = crate::BitReader<BKCMP2P_A>;
#[doc = "BRK COMP2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2P_A {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKCMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2P_A) -> Self {
        variant as u8 != 0
    }
}
impl BKCMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2P_A {
        match self.bits {
            false => BKCMP2P_A::NotInverted,
            true => BKCMP2P_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKCMP2P_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKCMP2P_A::Inverted
    }
}
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type BKCMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, BKCMP2P_A, O>;
impl<'a, const O: u8> BKCMP2P_W<'a, O> {
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BKCMP2P_A::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BKCMP2P_A::Inverted)
    }
}
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<1> {
        BKCMP1E_W::new(self)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<2> {
        BKCMP2E_W::new(self)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<10> {
        BKCMP1P_W::new(self)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<11> {
        BKCMP2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16 alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af1](index.html) module"]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af1::R](R) reader structure"]
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af1::W](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
