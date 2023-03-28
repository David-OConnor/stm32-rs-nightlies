#[doc = "Register `MMCCR` reader"]
pub struct R(crate::R<MMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCCR` writer"]
pub struct W(crate::W<MMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCR_SPEC>;
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
impl From<crate::W<MMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - Counter reset"]
pub type CR_R = crate::BitReader<CR_A>;
#[doc = "Counter reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR_A {
    #[doc = "1: Reset all counters. Cleared automatically"]
    Reset = 1,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            true => Some(CR_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR_A::Reset
    }
}
#[doc = "Field `CR` writer - Counter reset"]
pub type CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, CR_A, O>;
impl<'a, const O: u8> CR_W<'a, O> {
    #[doc = "Reset all counters. Cleared automatically"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR_A::Reset)
    }
}
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CSR_R = crate::BitReader<CSR_A>;
#[doc = "Counter stop rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR_A {
    #[doc = "0: Counters roll over to zero after reaching the maximum value"]
    Disabled = 0,
    #[doc = "1: Counters do not roll over to zero after reaching the maximum value"]
    Enabled = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::Disabled,
            true => CSR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSR_A::Enabled
    }
}
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, CSR_A, O>;
impl<'a, const O: u8> CSR_W<'a, O> {
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSR_A::Disabled)
    }
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSR_A::Enabled)
    }
}
#[doc = "Field `ROR` reader - Reset on read"]
pub type ROR_R = crate::BitReader<ROR_A>;
#[doc = "Reset on read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROR_A {
    #[doc = "0: MMC counters do not reset on read"]
    Disabled = 0,
    #[doc = "1: MMC counters reset to zero after read"]
    Enabled = 1,
}
impl From<ROR_A> for bool {
    #[inline(always)]
    fn from(variant: ROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROR_A {
        match self.bits {
            false => ROR_A::Disabled,
            true => ROR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROR_A::Enabled
    }
}
#[doc = "Field `ROR` writer - Reset on read"]
pub type ROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, ROR_A, O>;
impl<'a, const O: u8> ROR_W<'a, O> {
    #[doc = "MMC counters do not reset on read"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROR_A::Disabled)
    }
    #[doc = "MMC counters reset to zero after read"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROR_A::Enabled)
    }
}
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type MCF_R = crate::BitReader<MCF_A>;
#[doc = "MMC counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCF_A {
    #[doc = "0: All MMC counters update normally"]
    Unfrozen = 0,
    #[doc = "1: All MMC counters frozen to their current value"]
    Frozen = 1,
}
impl From<MCF_A> for bool {
    #[inline(always)]
    fn from(variant: MCF_A) -> Self {
        variant as u8 != 0
    }
}
impl MCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCF_A {
        match self.bits {
            false => MCF_A::Unfrozen,
            true => MCF_A::Frozen,
        }
    }
    #[doc = "Checks if the value of the field is `Unfrozen`"]
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCF_A::Unfrozen
    }
    #[doc = "Checks if the value of the field is `Frozen`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == MCF_A::Frozen
    }
}
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type MCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, MCF_A, O>;
impl<'a, const O: u8> MCF_W<'a, O> {
    #[doc = "All MMC counters update normally"]
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut W {
        self.variant(MCF_A::Unfrozen)
    }
    #[doc = "All MMC counters frozen to their current value"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(MCF_A::Frozen)
    }
}
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<1> {
        CSR_W::new(self)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<2> {
        ROR_W::new(self)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<31> {
        MCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC control register (ETH_MMCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmccr](index.html) module"]
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmccr::R](R) reader structure"]
impl crate::Readable for MMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmccr::W](W) writer structure"]
impl crate::Writable for MMCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
