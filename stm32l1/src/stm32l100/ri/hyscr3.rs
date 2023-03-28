#[doc = "Register `HYSCR3` reader"]
pub struct R(crate::R<HYSCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSCR3` writer"]
pub struct W(crate::W<HYSCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSCR3_SPEC>;
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
impl From<crate::W<HYSCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - Port E hysteresis control on/off"]
pub type PE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PE` writer - Port E hysteresis control on/off"]
pub type PE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR3_SPEC, u16, u16, 16, O>;
#[doc = "Field `PF` reader - Port F hysteresis control on/off"]
pub type PF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PF` writer - Port F hysteresis control on/off"]
pub type PF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSCR3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port E hysteresis control on/off"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port F hysteresis control on/off"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port E hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bits 16:31 - Port F hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<16> {
        PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI hysteresis control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr3](index.html) module"]
pub struct HYSCR3_SPEC;
impl crate::RegisterSpec for HYSCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyscr3::R](R) reader structure"]
impl crate::Readable for HYSCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyscr3::W](W) writer structure"]
impl crate::Writable for HYSCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HYSCR3 to value 0"]
impl crate::Resettable for HYSCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
