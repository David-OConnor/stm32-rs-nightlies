#[doc = "Register `MODER` reader"]
pub struct R(crate::R<MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODER` writer"]
pub struct W(crate::W<MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODER_SPEC>;
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
impl From<crate::W<MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODER3` reader - Port x configuration bits (y = 0..15)"]
pub type MODER3_R = crate::FieldReader<u8, MODER3_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER3_A {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: General purpose output mode"]
    Output = 1,
    #[doc = "2: Alternate function mode"]
    Alternate = 2,
    #[doc = "3: Analog mode"]
    Analog = 3,
}
impl From<MODER3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODER3_A) -> Self {
        variant as _
    }
}
impl MODER3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODER3_A {
        match self.bits {
            0 => MODER3_A::Input,
            1 => MODER3_A::Output,
            2 => MODER3_A::Alternate,
            3 => MODER3_A::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER3_A::Input
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER3_A::Output
    }
    #[doc = "Checks if the value of the field is `Alternate`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODER3_A::Alternate
    }
    #[doc = "Checks if the value of the field is `Analog`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER3_A::Analog
    }
}
#[doc = "Field `MODER3` writer - Port x configuration bits (y = 0..15)"]
pub type MODER3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODER_SPEC, u8, MODER3_A, 2, O>;
impl<'a, const O: u8> MODER3_W<'a, O> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER3_A::Input)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER3_A::Output)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER3_A::Alternate)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER3_A::Analog)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<6> {
        MODER3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](index.html) module"]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moder::R](R) reader structure"]
impl crate::Readable for MODER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moder::W](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODER to value 0xc0"]
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
