#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCC` reader - NMOS compensation code"]
pub type NCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCC` writer - NMOS compensation code"]
pub type NCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCC` reader - PMOS compensation code"]
pub type PCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCC` writer - PMOS compensation code"]
pub type PCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&self) -> NCC_R {
        NCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    #[must_use]
    pub fn ncc(&mut self) -> NCC_W<0> {
        NCC_W::new(self)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    #[must_use]
    pub fn pcc(&mut self) -> PCC_W<4> {
        PCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG compensation cell code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
