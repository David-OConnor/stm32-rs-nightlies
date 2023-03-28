#[doc = "Register `BRUR` reader"]
pub struct R(crate::R<BRUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRUR` writer"]
pub struct W(crate::W<BRUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRUR_SPEC>;
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
impl From<crate::W<BRUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUV` reader - source adresse update value"]
pub type SUV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUV` writer - source adresse update value"]
pub type SUV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRUR_SPEC, u16, u16, 16, O>;
#[doc = "Field `DUV` reader - destination address update"]
pub type DUV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUV` writer - destination address update"]
pub type DUV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRUR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<0> {
        SUV_W::new(self)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<16> {
        DUV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brur](index.html) module"]
pub struct BRUR_SPEC;
impl crate::RegisterSpec for BRUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brur::R](R) reader structure"]
impl crate::Readable for BRUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brur::W](W) writer structure"]
impl crate::Writable for BRUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRUR to value 0"]
impl crate::Resettable for BRUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
