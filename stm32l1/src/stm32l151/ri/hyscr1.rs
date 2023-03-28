#[doc = "Register `HYSCR1` reader"]
pub struct R(crate::R<HYSCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSCR1` writer"]
pub struct W(crate::W<HYSCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSCR1_SPEC>;
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
impl From<crate::W<HYSCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Port A hysteresis control on/off"]
pub type PA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PA` writer - Port A hysteresis control on/off"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `PB` reader - Port B hysteresis control on/off"]
pub type PB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PB` writer - Port B hysteresis control on/off"]
pub type PB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<0> {
        PA_W::new(self)
    }
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<16> {
        PB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI hysteresis control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr1](index.html) module"]
pub struct HYSCR1_SPEC;
impl crate::RegisterSpec for HYSCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyscr1::R](R) reader structure"]
impl crate::Readable for HYSCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyscr1::W](W) writer structure"]
impl crate::Writable for HYSCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HYSCR1 to value 0"]
impl crate::Resettable for HYSCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
