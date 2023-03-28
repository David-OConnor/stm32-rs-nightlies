#[doc = "Register `TISEL` reader"]
pub struct R(crate::R<TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TISEL` writer"]
pub struct W(crate::W<TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISEL_SPEC>;
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
impl From<crate::W<TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL3_0` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
pub type TI1SEL3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI1SEL3_0` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
pub type TI1SEL3_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TI2SEL3_0` reader - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
pub type TI2SEL3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI2SEL3_0` writer - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
pub type TI2SEL3_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TI3SEL3_0` reader - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
pub type TI3SEL3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI3SEL3_0` writer - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
pub type TI3SEL3_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TI4SEL3_0` reader - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
pub type TI4SEL3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI4SEL3_0` writer - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
pub type TI4SEL3_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel3_0(&self) -> TI1SEL3_0_R {
        TI1SEL3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    pub fn ti2sel3_0(&self) -> TI2SEL3_0_R {
        TI2SEL3_0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    pub fn ti3sel3_0(&self) -> TI3SEL3_0_R {
        TI3SEL3_0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
    #[inline(always)]
    pub fn ti4sel3_0(&self) -> TI4SEL3_0_R {
        TI4SEL3_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel3_0(&mut self) -> TI1SEL3_0_W<0> {
        TI1SEL3_0_W::new(self)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel3_0(&mut self) -> TI2SEL3_0_W<8> {
        TI2SEL3_0_W::new(self)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel3_0(&mut self) -> TI3SEL3_0_W<16> {
        TI3SEL3_0_W::new(self)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel3_0(&mut self) -> TI4SEL3_0_W<24> {
        TI4SEL3_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisel](index.html) module"]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tisel::R](R) reader structure"]
impl crate::Readable for TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tisel::W](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
