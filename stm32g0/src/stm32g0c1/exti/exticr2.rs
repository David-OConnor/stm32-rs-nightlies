#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0_7` reader - GPIO port selection"]
pub type EXTI0_7_R = crate::FieldReader<u8, EXTI0_7_A>;
#[doc = "GPIO port selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_7_A {
    #[doc = "0: GPIO port A selected"]
    Pa = 0,
    #[doc = "1: GPIO port B selected"]
    Pb = 1,
    #[doc = "2: GPIO port C selected"]
    Pc = 2,
    #[doc = "3: GPIO port D selected"]
    Pd = 3,
    #[doc = "5: GPIO port F selected"]
    Pf = 5,
}
impl From<EXTI0_7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_7_A) -> Self {
        variant as _
    }
}
impl EXTI0_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_7_A> {
        match self.bits {
            0 => Some(EXTI0_7_A::Pa),
            1 => Some(EXTI0_7_A::Pb),
            2 => Some(EXTI0_7_A::Pc),
            3 => Some(EXTI0_7_A::Pd),
            5 => Some(EXTI0_7_A::Pf),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa`"]
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI0_7_A::Pa
    }
    #[doc = "Checks if the value of the field is `Pb`"]
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI0_7_A::Pb
    }
    #[doc = "Checks if the value of the field is `Pc`"]
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI0_7_A::Pc
    }
    #[doc = "Checks if the value of the field is `Pd`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI0_7_A::Pd
    }
    #[doc = "Checks if the value of the field is `Pf`"]
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI0_7_A::Pf
    }
}
#[doc = "Field `EXTI0_7` writer - GPIO port selection"]
pub type EXTI0_7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI0_7_A, 8, O>;
impl<'a, const O: u8> EXTI0_7_W<'a, O> {
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::Pa)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::Pb)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::Pc)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::Pd)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::Pf)
    }
}
#[doc = "Field `EXTI8_15` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI8_15_R;
#[doc = "Field `EXTI16_23` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI16_23_R;
#[doc = "Field `EXTI24_31` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI24_31_R;
#[doc = "Field `EXTI8_15` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI8_15_W;
#[doc = "Field `EXTI16_23` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI16_23_W;
#[doc = "Field `EXTI24_31` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI24_31_W;
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<0> {
        EXTI0_7_W::new(self)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<8> {
        EXTI8_15_W::new(self)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<16> {
        EXTI16_23_W::new(self)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<24> {
        EXTI24_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
