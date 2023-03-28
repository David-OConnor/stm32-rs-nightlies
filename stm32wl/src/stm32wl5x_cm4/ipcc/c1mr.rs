#[doc = "Register `C1MR` reader"]
pub struct R(crate::R<C1MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1MR` writer"]
pub struct W(crate::W<C1MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1MR_SPEC>;
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
impl From<crate::W<C1MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1OM` reader - CH1OM"]
pub type CH1OM_R = crate::BitReader<CH1OM_A>;
#[doc = "CH1OM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OM_A {
    #[doc = "0: Receive channel n occupied interrupt not masked"]
    Unmasked = 0,
    #[doc = "1: Receive channel n occupied interrupt masked"]
    Masked = 1,
}
impl From<CH1OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OM_A {
        match self.bits {
            false => CH1OM_A::Unmasked,
            true => CH1OM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1OM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1OM_A::Masked
    }
}
#[doc = "Field `CH1OM` writer - CH1OM"]
pub type CH1OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, CH1OM_A, O>;
impl<'a, const O: u8> CH1OM_W<'a, O> {
    #[doc = "Receive channel n occupied interrupt not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH1OM_A::Unmasked)
    }
    #[doc = "Receive channel n occupied interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH1OM_A::Masked)
    }
}
#[doc = "Field `CH2OM` reader - CH2OM"]
pub use CH1OM_R as CH2OM_R;
#[doc = "Field `CH3OM` reader - CH3OM"]
pub use CH1OM_R as CH3OM_R;
#[doc = "Field `CH4OM` reader - CH4OM"]
pub use CH1OM_R as CH4OM_R;
#[doc = "Field `CH5OM` reader - CH5OM"]
pub use CH1OM_R as CH5OM_R;
#[doc = "Field `CH6OM` reader - CH6OM"]
pub use CH1OM_R as CH6OM_R;
#[doc = "Field `CH2OM` writer - CH2OM"]
pub use CH1OM_W as CH2OM_W;
#[doc = "Field `CH3OM` writer - CH3OM"]
pub use CH1OM_W as CH3OM_W;
#[doc = "Field `CH4OM` writer - CH4OM"]
pub use CH1OM_W as CH4OM_W;
#[doc = "Field `CH5OM` writer - CH5OM"]
pub use CH1OM_W as CH5OM_W;
#[doc = "Field `CH6OM` writer - CH6OM"]
pub use CH1OM_W as CH6OM_W;
#[doc = "Field `CH1FM` reader - CH1FM"]
pub type CH1FM_R = crate::BitReader<CH1FM_A>;
#[doc = "CH1FM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1FM_A {
    #[doc = "0: Transmit channel n free interrupt not masked"]
    Unmasked = 0,
    #[doc = "1: Transmit channel n free interrupt masked"]
    Masked = 1,
}
impl From<CH1FM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1FM_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1FM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1FM_A {
        match self.bits {
            false => CH1FM_A::Unmasked,
            true => CH1FM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1FM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1FM_A::Masked
    }
}
#[doc = "Field `CH1FM` writer - CH1FM"]
pub type CH1FM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1MR_SPEC, CH1FM_A, O>;
impl<'a, const O: u8> CH1FM_W<'a, O> {
    #[doc = "Transmit channel n free interrupt not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH1FM_A::Unmasked)
    }
    #[doc = "Transmit channel n free interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH1FM_A::Masked)
    }
}
#[doc = "Field `CH2FM` reader - CH2FM"]
pub use CH1FM_R as CH2FM_R;
#[doc = "Field `CH3FM` reader - CH3FM"]
pub use CH1FM_R as CH3FM_R;
#[doc = "Field `CH4FM` reader - CH4FM"]
pub use CH1FM_R as CH4FM_R;
#[doc = "Field `CH5FM` reader - CH5FM"]
pub use CH1FM_R as CH5FM_R;
#[doc = "Field `CH6FM` reader - CH6FM"]
pub use CH1FM_R as CH6FM_R;
#[doc = "Field `CH2FM` writer - CH2FM"]
pub use CH1FM_W as CH2FM_W;
#[doc = "Field `CH3FM` writer - CH3FM"]
pub use CH1FM_W as CH3FM_W;
#[doc = "Field `CH4FM` writer - CH4FM"]
pub use CH1FM_W as CH4FM_W;
#[doc = "Field `CH5FM` writer - CH5FM"]
pub use CH1FM_W as CH5FM_W;
#[doc = "Field `CH6FM` writer - CH6FM"]
pub use CH1FM_W as CH6FM_W;
impl R {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH4OM"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH5OM"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6OM"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH3FM"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH4FM"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH5FM"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH6FM"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> CH1OM_W<0> {
        CH1OM_W::new(self)
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<1> {
        CH2OM_W::new(self)
    }
    #[doc = "Bit 2 - CH3OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<2> {
        CH3OM_W::new(self)
    }
    #[doc = "Bit 3 - CH4OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om(&mut self) -> CH4OM_W<3> {
        CH4OM_W::new(self)
    }
    #[doc = "Bit 4 - CH5OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om(&mut self) -> CH5OM_W<4> {
        CH5OM_W::new(self)
    }
    #[doc = "Bit 5 - CH6OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om(&mut self) -> CH6OM_W<5> {
        CH6OM_W::new(self)
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fm(&mut self) -> CH1FM_W<16> {
        CH1FM_W::new(self)
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fm(&mut self) -> CH2FM_W<17> {
        CH2FM_W::new(self)
    }
    #[doc = "Bit 18 - CH3FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fm(&mut self) -> CH3FM_W<18> {
        CH3FM_W::new(self)
    }
    #[doc = "Bit 19 - CH4FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch4fm(&mut self) -> CH4FM_W<19> {
        CH4FM_W::new(self)
    }
    #[doc = "Bit 20 - CH5FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch5fm(&mut self) -> CH5FM_W<20> {
        CH5FM_W::new(self)
    }
    #[doc = "Bit 21 - CH6FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch6fm(&mut self) -> CH6FM_W<21> {
        CH6FM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 1 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1mr](index.html) module"]
pub struct C1MR_SPEC;
impl crate::RegisterSpec for C1MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1mr::R](R) reader structure"]
impl crate::Readable for C1MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1mr::W](W) writer structure"]
impl crate::Writable for C1MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1MR to value 0xffff_ffff"]
impl crate::Resettable for C1MR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
