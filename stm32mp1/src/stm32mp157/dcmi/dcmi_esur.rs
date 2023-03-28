#[doc = "Register `DCMI_ESUR` reader"]
pub struct R(crate::R<DCMI_ESUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_ESUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_ESUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_ESUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_ESUR` writer"]
pub struct W(crate::W<DCMI_ESUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_ESUR_SPEC>;
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
impl From<crate::W<DCMI_ESUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_ESUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSU` reader - FSU"]
pub type FSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSU` writer - FSU"]
pub type FSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESUR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LSU` reader - LSU"]
pub type LSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSU` writer - LSU"]
pub type LSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESUR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LEU` reader - LEU"]
pub type LEU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEU` writer - LEU"]
pub type LEU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESUR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FEU` reader - FEU"]
pub type FEU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEU` writer - FEU"]
pub type FEU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESUR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FSU"]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - LSU"]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LEU"]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FEU"]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FSU"]
    #[inline(always)]
    #[must_use]
    pub fn fsu(&mut self) -> FSU_W<0> {
        FSU_W::new(self)
    }
    #[doc = "Bits 8:15 - LSU"]
    #[inline(always)]
    #[must_use]
    pub fn lsu(&mut self) -> LSU_W<8> {
        LSU_W::new(self)
    }
    #[doc = "Bits 16:23 - LEU"]
    #[inline(always)]
    #[must_use]
    pub fn leu(&mut self) -> LEU_W<16> {
        LEU_W::new(self)
    }
    #[doc = "Bits 24:31 - FEU"]
    #[inline(always)]
    #[must_use]
    pub fn feu(&mut self) -> FEU_W<24> {
        FEU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI embedded synchronization unmask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_esur](index.html) module"]
pub struct DCMI_ESUR_SPEC;
impl crate::RegisterSpec for DCMI_ESUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_esur::R](R) reader structure"]
impl crate::Readable for DCMI_ESUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_esur::W](W) writer structure"]
impl crate::Writable for DCMI_ESUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_ESUR to value 0"]
impl crate::Resettable for DCMI_ESUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
