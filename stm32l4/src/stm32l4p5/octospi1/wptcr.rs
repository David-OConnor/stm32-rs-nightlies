#[doc = "Register `WPTCR` reader"]
pub struct R(crate::R<WPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPTCR` writer"]
pub struct W(crate::W<WPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPTCR_SPEC>;
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
impl From<crate::W<WPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCYC` reader - Number of dummy cycles"]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - Number of dummy cycles"]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WPTCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader<DHQC_A>;
#[doc = "Delay hold quarter cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHQC_A {
    #[doc = "0: No delay hold"]
    NoDelay = 0,
    #[doc = "1: 1/4 cycle hold"]
    QuarterCycleHold = 1,
}
impl From<DHQC_A> for bool {
    #[inline(always)]
    fn from(variant: DHQC_A) -> Self {
        variant as u8 != 0
    }
}
impl DHQC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DHQC_A {
        match self.bits {
            false => DHQC_A::NoDelay,
            true => DHQC_A::QuarterCycleHold,
        }
    }
    #[doc = "Checks if the value of the field is `NoDelay`"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == DHQC_A::NoDelay
    }
    #[doc = "Checks if the value of the field is `QuarterCycleHold`"]
    #[inline(always)]
    pub fn is_quarter_cycle_hold(&self) -> bool {
        *self == DHQC_A::QuarterCycleHold
    }
}
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPTCR_SPEC, DHQC_A, O>;
impl<'a, const O: u8> DHQC_W<'a, O> {
    #[doc = "No delay hold"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut W {
        self.variant(DHQC_A::NoDelay)
    }
    #[doc = "1/4 cycle hold"]
    #[inline(always)]
    pub fn quarter_cycle_hold(self) -> &'a mut W {
        self.variant(DHQC_A::QuarterCycleHold)
    }
}
#[doc = "Field `SSHIFT` reader - Sample shift"]
pub type SSHIFT_R = crate::BitReader<SSHIFT_A>;
#[doc = "Sample shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSHIFT_A {
    #[doc = "0: No shift"]
    NoShift = 0,
    #[doc = "1: 1/2 cycle shift"]
    HalfCycleShift = 1,
}
impl From<SSHIFT_A> for bool {
    #[inline(always)]
    fn from(variant: SSHIFT_A) -> Self {
        variant as u8 != 0
    }
}
impl SSHIFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSHIFT_A {
        match self.bits {
            false => SSHIFT_A::NoShift,
            true => SSHIFT_A::HalfCycleShift,
        }
    }
    #[doc = "Checks if the value of the field is `NoShift`"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == SSHIFT_A::NoShift
    }
    #[doc = "Checks if the value of the field is `HalfCycleShift`"]
    #[inline(always)]
    pub fn is_half_cycle_shift(&self) -> bool {
        *self == SSHIFT_A::HalfCycleShift
    }
}
#[doc = "Field `SSHIFT` writer - Sample shift"]
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPTCR_SPEC, SSHIFT_A, O>;
impl<'a, const O: u8> SSHIFT_W<'a, O> {
    #[doc = "No shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(SSHIFT_A::NoShift)
    }
    #[doc = "1/2 cycle shift"]
    #[inline(always)]
    pub fn half_cycle_shift(self) -> &'a mut W {
        self.variant(SSHIFT_A::HalfCycleShift)
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<0> {
        DCYC_W::new(self)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<28> {
        DHQC_W::new(self)
    }
    #[doc = "Bit 30 - Sample shift"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<30> {
        SSHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wrap timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wptcr](index.html) module"]
pub struct WPTCR_SPEC;
impl crate::RegisterSpec for WPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wptcr::R](R) reader structure"]
impl crate::Readable for WPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wptcr::W](W) writer structure"]
impl crate::Writable for WPTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPTCR to value 0"]
impl crate::Resettable for WPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
