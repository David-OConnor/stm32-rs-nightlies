#[doc = "Register `LPTIM_CFGR2` reader"]
pub struct R(crate::R<LPTIM_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CFGR2` writer"]
pub struct W(crate::W<LPTIM_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR2_SPEC>;
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
impl From<crate::W<LPTIM_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN1SEL` reader - IN1SEL"]
pub type IN1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN1SEL` writer - IN1SEL"]
pub type IN1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `IN2SEL` reader - IN2SEL"]
pub type IN2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN2SEL` writer - IN2SEL"]
pub type IN2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - IN1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn in1sel(&mut self) -> IN1SEL_W<0> {
        IN1SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - IN2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn in2sel(&mut self) -> IN2SEL_W<4> {
        IN2SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr2](index.html) module"]
pub struct LPTIM_CFGR2_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cfgr2::R](R) reader structure"]
impl crate::Readable for LPTIM_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr2::W](W) writer structure"]
impl crate::Writable for LPTIM_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CFGR2 to value 0"]
impl crate::Resettable for LPTIM_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}