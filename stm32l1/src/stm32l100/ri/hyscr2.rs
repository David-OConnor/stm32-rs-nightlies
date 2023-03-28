#[doc = "Register `HYSCR2` reader"]
pub struct R(crate::R<HYSCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSCR2` writer"]
pub struct W(crate::W<HYSCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSCR2_SPEC>;
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
impl From<crate::W<HYSCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC` reader - Port C hysteresis control on/off"]
pub type PC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC` writer - Port C hysteresis control on/off"]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR2_SPEC, u16, u16, 16, O>;
#[doc = "Field `PD` reader - Port D hysteresis control on/off"]
pub type PD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PD` writer - Port D hysteresis control on/off"]
pub type PD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port C hysteresis control on/off"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port D hysteresis control on/off"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port C hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<0> {
        PC_W::new(self)
    }
    #[doc = "Bits 16:31 - Port D hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<16> {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI hysteresis control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr2](index.html) module"]
pub struct HYSCR2_SPEC;
impl crate::RegisterSpec for HYSCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyscr2::R](R) reader structure"]
impl crate::Readable for HYSCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyscr2::W](W) writer structure"]
impl crate::Writable for HYSCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HYSCR2 to value 0"]
impl crate::Resettable for HYSCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
