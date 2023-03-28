#[doc = "Register `AWSCDR` reader"]
pub struct R(crate::R<AWSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWSCDR` writer"]
pub struct W(crate::W<AWSCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWSCDR_SPEC>;
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
impl From<crate::W<AWSCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWSCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDT` reader - SCDT"]
pub type SCDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCDT` writer - SCDT"]
pub type SCDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWSCDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKSCD` reader - BKSCD"]
pub type BKSCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKSCD` writer - BKSCD"]
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWSCDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWFOSR` reader - AWFOSR"]
pub type AWFOSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWFOSR` writer - AWFOSR"]
pub type AWFOSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWSCDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `AWFORD` reader - AWFORD"]
pub type AWFORD_R = crate::FieldReader<u8, AWFORD_A>;
#[doc = "AWFORD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWFORD_A {
    #[doc = "0: FastSinc filter type"]
    FastSinc = 0,
    #[doc = "1: Sinc1 filter type"]
    Sinc1 = 1,
    #[doc = "2: Sinc2 filter type"]
    Sinc2 = 2,
    #[doc = "3: Sinc3 filter type"]
    Sinc3 = 3,
}
impl From<AWFORD_A> for u8 {
    #[inline(always)]
    fn from(variant: AWFORD_A) -> Self {
        variant as _
    }
}
impl AWFORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWFORD_A {
        match self.bits {
            0 => AWFORD_A::FastSinc,
            1 => AWFORD_A::Sinc1,
            2 => AWFORD_A::Sinc2,
            3 => AWFORD_A::Sinc3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FastSinc`"]
    #[inline(always)]
    pub fn is_fast_sinc(&self) -> bool {
        *self == AWFORD_A::FastSinc
    }
    #[doc = "Checks if the value of the field is `Sinc1`"]
    #[inline(always)]
    pub fn is_sinc1(&self) -> bool {
        *self == AWFORD_A::Sinc1
    }
    #[doc = "Checks if the value of the field is `Sinc2`"]
    #[inline(always)]
    pub fn is_sinc2(&self) -> bool {
        *self == AWFORD_A::Sinc2
    }
    #[doc = "Checks if the value of the field is `Sinc3`"]
    #[inline(always)]
    pub fn is_sinc3(&self) -> bool {
        *self == AWFORD_A::Sinc3
    }
}
#[doc = "Field `AWFORD` writer - AWFORD"]
pub type AWFORD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AWSCDR_SPEC, u8, AWFORD_A, 2, O>;
impl<'a, const O: u8> AWFORD_W<'a, O> {
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn fast_sinc(self) -> &'a mut W {
        self.variant(AWFORD_A::FastSinc)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn sinc1(self) -> &'a mut W {
        self.variant(AWFORD_A::Sinc1)
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn sinc2(self) -> &'a mut W {
        self.variant(AWFORD_A::Sinc2)
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn sinc3(self) -> &'a mut W {
        self.variant(AWFORD_A::Sinc3)
    }
}
impl R {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<0> {
        SCDT_W::new(self)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<12> {
        BKSCD_W::new(self)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<16> {
        AWFOSR_W::new(self)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<22> {
        AWFORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awscdr](index.html) module"]
pub struct AWSCDR_SPEC;
impl crate::RegisterSpec for AWSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awscdr::R](R) reader structure"]
impl crate::Readable for AWSCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awscdr::W](W) writer structure"]
impl crate::Writable for AWSCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWSCDR to value 0"]
impl crate::Resettable for AWSCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
