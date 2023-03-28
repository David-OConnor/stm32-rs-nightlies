#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETR_RMP` reader - External trigger remap"]
pub type ETR_RMP_R = crate::BitReader<ETR_RMP_A>;
#[doc = "External trigger remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETR_RMP_A {
    #[doc = "0: TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping"]
    Gpio = 0,
    #[doc = "1: LSE internal clock is connected to TIM2_ETR input"]
    Tim2Etr = 1,
}
impl From<ETR_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ETR_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ETR_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETR_RMP_A {
        match self.bits {
            false => ETR_RMP_A::Gpio,
            true => ETR_RMP_A::Tim2Etr,
        }
    }
    #[doc = "Checks if the value of the field is `Gpio`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP_A::Gpio
    }
    #[doc = "Checks if the value of the field is `Tim2Etr`"]
    #[inline(always)]
    pub fn is_tim2_etr(&self) -> bool {
        *self == ETR_RMP_A::Tim2Etr
    }
}
#[doc = "Field `ETR_RMP` writer - External trigger remap"]
pub type ETR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, ETR_RMP_A, O>;
impl<'a, const O: u8> ETR_RMP_W<'a, O> {
    #[doc = "TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Gpio)
    }
    #[doc = "LSE internal clock is connected to TIM2_ETR input"]
    #[inline(always)]
    pub fn tim2_etr(self) -> &'a mut W {
        self.variant(ETR_RMP_A::Tim2Etr)
    }
}
#[doc = "Field `TI4_RMP` reader - Input capture 4 remap"]
pub type TI4_RMP_R = crate::FieldReader<u8, TI4_RMP_A>;
#[doc = "Input capture 4 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4_RMP_A {
    #[doc = "0: TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping"]
    Gpio = 0,
    #[doc = "1: TIM2 TI4 is connected to COMP1_OUT"]
    Comp1 = 1,
    #[doc = "2: TIM2 TI4 is connected to COMP2_OUT"]
    Comp2 = 2,
    #[doc = "3: TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT"]
    Comp12 = 3,
}
impl From<TI4_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP_A) -> Self {
        variant as _
    }
}
impl TI4_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI4_RMP_A {
        match self.bits {
            0 => TI4_RMP_A::Gpio,
            1 => TI4_RMP_A::Comp1,
            2 => TI4_RMP_A::Comp2,
            3 => TI4_RMP_A::Comp12,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Gpio`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI4_RMP_A::Gpio
    }
    #[doc = "Checks if the value of the field is `Comp1`"]
    #[inline(always)]
    pub fn is_comp_1(&self) -> bool {
        *self == TI4_RMP_A::Comp1
    }
    #[doc = "Checks if the value of the field is `Comp2`"]
    #[inline(always)]
    pub fn is_comp_2(&self) -> bool {
        *self == TI4_RMP_A::Comp2
    }
    #[doc = "Checks if the value of the field is `Comp12`"]
    #[inline(always)]
    pub fn is_comp_12(&self) -> bool {
        *self == TI4_RMP_A::Comp12
    }
}
#[doc = "Field `TI4_RMP` writer - Input capture 4 remap"]
pub type TI4_RMP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OR1_SPEC, u8, TI4_RMP_A, 2, O>;
impl<'a, const O: u8> TI4_RMP_W<'a, O> {
    #[doc = "TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Gpio)
    }
    #[doc = "TIM2 TI4 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp_1(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Comp1)
    }
    #[doc = "TIM2 TI4 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp_2(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Comp2)
    }
    #[doc = "TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT"]
    #[inline(always)]
    pub fn comp_12(self) -> &'a mut W {
        self.variant(TI4_RMP_A::Comp12)
    }
}
impl R {
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<1> {
        ETR_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<2> {
        TI4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
