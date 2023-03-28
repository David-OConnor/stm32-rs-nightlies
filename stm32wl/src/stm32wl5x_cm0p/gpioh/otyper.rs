#[doc = "Register `OTYPER` reader"]
pub struct R(crate::R<OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTYPER` writer"]
pub struct W(crate::W<OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTYPER_SPEC>;
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
impl From<crate::W<OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub type OT3_R = crate::BitReader<OT3_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT3_A {
    #[doc = "0: Output push-pull (reset state)"]
    PushPull = 0,
    #[doc = "1: Output open-drain"]
    OpenDrain = 1,
}
impl From<OT3_A> for bool {
    #[inline(always)]
    fn from(variant: OT3_A) -> Self {
        variant as u8 != 0
    }
}
impl OT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OT3_A {
        match self.bits {
            false => OT3_A::PushPull,
            true => OT3_A::OpenDrain,
        }
    }
    #[doc = "Checks if the value of the field is `PushPull`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OT3_A::PushPull
    }
    #[doc = "Checks if the value of the field is `OpenDrain`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OT3_A::OpenDrain
    }
}
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub type OT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, OT3_A, O>;
impl<'a, const O: u8> OT3_W<'a, O> {
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT3_A::PushPull)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT3_A::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<3> {
        OT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](index.html) module"]
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otyper::R](R) reader structure"]
impl crate::Readable for OTYPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otyper::W](W) writer structure"]
impl crate::Writable for OTYPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
